use std::path::PathBuf;

use anyhow::{bail, Context, Error, Result};
use crc::{Crc, CRC_32_BZIP2};
use yew_agent::{Agent, AgentLink, HandlerId, Job};

use crate::gui::RcUi;
use crate::rpc::{Base64File, RpcFile};
use crate::save_data::{
    mass_effect_1_le::{Me1LeSaveData, Me1LeSaveGame},
    mass_effect_2::{Me2LeSaveGame, Me2LeVersion, Me2SaveGame, Me2Version},
    mass_effect_3::{Me3SaveGame, Me3Version},
};
use crate::{rpc, unreal};

#[derive(Clone)]
pub enum SaveGame {
    // MassEffect1 { file_path: PathBuf, save_game: RcUi<Me1SaveGame> },
    MassEffect1Le { file_path: PathBuf, save_game: RcUi<Me1LeSaveGame> },
    MassEffect1LePs4 { file_path: PathBuf, save_game: RcUi<Me1LeSaveData> },
    MassEffect2 { file_path: PathBuf, save_game: RcUi<Me2SaveGame> },
    MassEffect2Le { file_path: PathBuf, save_game: RcUi<Me2LeSaveGame> },
    MassEffect3 { file_path: PathBuf, save_game: RcUi<Me3SaveGame> },
}

pub enum Msg {
    SaveOpened(HandlerId, SaveGame),
    SaveSaved(HandlerId),
    DialogCancelled,
    Error(HandlerId, Error),
}

pub enum Request {
    OpenSave,
    SaveSave(SaveGame),
    ReloadSave(PathBuf),
    // TODO: Head Morph
}

pub enum Response {
    SaveOpened(SaveGame),
    SaveSaved,
    Error(Error),
}

pub struct SaveHandler {
    link: AgentLink<Self>,
}

impl Agent for SaveHandler {
    type Reach = Job<Self>;
    type Message = Msg;
    type Input = Request;
    type Output = Response;

    fn create(link: AgentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::SaveOpened(who, save_game) => {
                self.link.respond(who, Response::SaveOpened(save_game))
            }
            Msg::SaveSaved(who) => self.link.respond(who, Response::SaveSaved),
            Msg::DialogCancelled => (),
            Msg::Error(who, err) => self.link.respond(who, Response::Error(err)),
        }
    }

    fn handle_input(&mut self, msg: Self::Input, who: HandlerId) {
        match msg {
            Request::OpenSave => self.open_save(who),
            Request::SaveSave(save_game) => self.save_save(who, save_game),
            Request::ReloadSave(path) => self.reload_save(who, path),
        }
    }
}

impl SaveHandler {
    fn open_save(&self, who: HandlerId) {
        self.link.send_future(async move {
            let handle_save = async {
                let rpc_file = rpc::open().await?;
                Self::deserialize(rpc_file)
            };

            match handle_save.await.context("Failed to open the save") {
                Ok(save_game) => Msg::SaveOpened(who, save_game),
                Err(err) => Msg::Error(who, err),
            }
        });
    }

    fn save_save(&self, who: HandlerId, save_game: SaveGame) {
        let path = match save_game {
            SaveGame::MassEffect1Le { ref file_path, .. }
            | SaveGame::MassEffect1LePs4 { ref file_path, .. }
            | SaveGame::MassEffect2 { ref file_path, .. }
            | SaveGame::MassEffect2Le { ref file_path, .. }
            | SaveGame::MassEffect3 { ref file_path, .. } => file_path.to_owned(),
        };
        self.link.send_future(async move {
            let handle_save = async {
                let has_path = rpc::save_dialog(path).await?;
                let cancelled = match has_path {
                    Some(path) => {
                        let rpc_file = Self::serialize(path, save_game)?;
                        rpc::save(rpc_file).await?;
                        false
                    }
                    None => true,
                };
                Ok::<_, Error>(cancelled)
            };

            match handle_save.await.context("Failed to save the save") {
                Ok(cancelled) => {
                    if cancelled {
                        Msg::DialogCancelled
                    } else {
                        Msg::SaveSaved(who)
                    }
                }
                Err(err) => Msg::Error(who, err),
            }
        });
    }

    fn reload_save(&self, who: HandlerId, path: PathBuf) {
        self.link.send_future(async move {
            let handle_save = async move {
                let rpc_file = rpc::reload(path).await?;
                Self::deserialize(rpc_file)
            };

            match handle_save.await.context("Failed to reload the save") {
                Ok(save_game) => Msg::SaveOpened(who, save_game),
                Err(err) => Msg::Error(who, err),
            }
        });
    }

    fn deserialize(rpc_file: RpcFile) -> Result<SaveGame> {
        let file_path = rpc_file.path;
        let input = rpc_file.file.into_bytes()?;

        let save_game = if input.len() >= 4 && input[0..4] == [0xC1, 0x83, 0x2A, 0x9E] {
            // ME1 Legendary
            SaveGame::MassEffect1Le {
                file_path,
                save_game: unreal::Deserializer::from_bytes(&input)?,
            }
        } else if unreal::Deserializer::from_bytes::<Me2Version>(&input).is_ok() {
            // ME2
            SaveGame::MassEffect2 {
                file_path,
                save_game: unreal::Deserializer::from_bytes(&input)?,
            }
        } else if unreal::Deserializer::from_bytes::<Me2LeVersion>(&input).is_ok() {
            // ME2 Legendary
            SaveGame::MassEffect2Le {
                file_path,
                save_game: unreal::Deserializer::from_bytes(&input)?,
            }
        } else if unreal::Deserializer::from_bytes::<Me3Version>(&input).is_ok() {
            // ME3
            SaveGame::MassEffect3 {
                file_path,
                save_game: unreal::Deserializer::from_bytes(&input)?,
            }
        } else if let Some(ext) = file_path.extension() {
            if ext.eq_ignore_ascii_case("ps4sav") {
                // ME1LE PS4
                SaveGame::MassEffect1LePs4 {
                    file_path,
                    save_game: unreal::Deserializer::from_bytes(&input)?,
                }
            // } else if ext.eq_ignore_ascii_case("MassEffectSave") {
            //     // ME1
            //     SaveGame::MassEffect1 {
            //         file_path,
            //         save_game: unreal::Deserializer::from_bytes(&input)?,
            //     }
            } else {
                bail!("Unsupported file");
            }
        } else {
            bail!("Unsupported file");
        };
        Ok(save_game)
    }

    fn serialize(path: PathBuf, save_game: SaveGame) -> Result<RpcFile> {
        let output = match save_game {
            // SaveGame::MassEffect1 { save_game, .. } => unreal::Serializer::to_vec(&save_game)?,
            SaveGame::MassEffect1Le { save_game, .. } => {
                let mut output = unreal::Serializer::to_vec(&save_game)?;

                // Checksum
                let checksum_offset = output.len() - 12;
                let crc = Crc::<u32>::new(&CRC_32_BZIP2);
                let checksum = crc.checksum(&output[..checksum_offset]);

                // Update checksum
                let end = checksum_offset + 4;
                output[checksum_offset..end].swap_with_slice(&mut u32::to_le_bytes(checksum));
                output
            }
            SaveGame::MassEffect1LePs4 { save_game, .. } => unreal::Serializer::to_vec(&save_game)?,
            SaveGame::MassEffect2 { save_game, .. } => {
                let mut output = unreal::Serializer::to_vec(&save_game)?;

                let crc = Crc::<u32>::new(&CRC_32_BZIP2);
                let checksum = crc.checksum(&output);
                output.extend(&u32::to_le_bytes(checksum));
                output
            }
            SaveGame::MassEffect2Le { save_game, .. } => {
                let mut output = unreal::Serializer::to_vec(&save_game)?;

                let crc = Crc::<u32>::new(&CRC_32_BZIP2);
                let checksum = crc.checksum(&output);
                output.extend(&u32::to_le_bytes(checksum));
                output
            }
            SaveGame::MassEffect3 { save_game, .. } => {
                let mut output = unreal::Serializer::to_vec(&save_game)?;

                let crc = Crc::<u32>::new(&CRC_32_BZIP2);
                let checksum = crc.checksum(&output);
                output.extend(&u32::to_le_bytes(checksum));
                output
            }
        };

        let file = Base64File { unencoded_size: output.len(), base64: base64::encode(output) };
        let rpc_file = RpcFile { path, file };

        Ok(rpc_file)
    }
}
