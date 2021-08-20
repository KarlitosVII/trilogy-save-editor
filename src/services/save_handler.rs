use std::path::PathBuf;

use anyhow::{bail, Context, Error, Result};
use crc::{Crc, CRC_32_BZIP2};
use gloo::console;
use ron::ser::PrettyConfig;
use yew_agent::{Agent, AgentLink, HandlerId, Job};

use crate::gui::RcUi;
use crate::save_data::{
    mass_effect_1::Me1SaveGame,
    mass_effect_1_le::{Me1LeSaveData, Me1LeSaveGame},
    mass_effect_2::{Me2LeSaveGame, Me2LeVersion, Me2SaveGame, Me2Version},
    mass_effect_3::{Me3SaveGame, Me3Version},
    shared::appearance::HeadMorph,
};
use crate::services::rpc::{self, Base64File, RpcFile};
use crate::unreal;

#[derive(Clone)]
pub enum SaveGame {
    MassEffect1 { file_path: PathBuf, save_game: RcUi<Me1SaveGame> },
    MassEffect1Le { file_path: PathBuf, save_game: RcUi<Me1LeSaveGame> },
    MassEffect1LePs4 { file_path: PathBuf, save_game: RcUi<Me1LeSaveData> },
    MassEffect2 { file_path: PathBuf, save_game: RcUi<Me2SaveGame> },
    MassEffect2Le { file_path: PathBuf, save_game: RcUi<Me2LeSaveGame> },
    MassEffect3 { file_path: PathBuf, save_game: RcUi<Me3SaveGame> },
}

pub enum Msg {
    SaveOpened(HandlerId, SaveGame),
    SaveSaved(HandlerId),
    HeadMorphImported(HandlerId, HeadMorph),
    HeadMorphExported(HandlerId),
    DialogCancelled,
    Error(HandlerId, Error),
}

pub enum Request {
    OpenSave,
    SaveDropped(String, Vec<u8>),
    SaveSave(SaveGame),
    ReloadSave(PathBuf),
    ImportHeadMorph,
    ExportHeadMorph(RcUi<HeadMorph>),
}

pub enum Response {
    SaveOpened(SaveGame),
    SaveSaved,
    HeadMorphImported(HeadMorph),
    HeadMorphExported,
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
            Msg::HeadMorphImported(who, head_morph) => {
                self.link.respond(who, Response::HeadMorphImported(head_morph))
            }
            Msg::HeadMorphExported(who) => self.link.respond(who, Response::HeadMorphExported),
            Msg::DialogCancelled => {
                #[cfg(debug_assertions)]
                console::log!("Dialog cancelled");
            }
            Msg::Error(who, err) => self.link.respond(who, Response::Error(err)),
        }
    }

    fn handle_input(&mut self, msg: Self::Input, who: HandlerId) {
        match msg {
            Request::OpenSave => self.open_save(who),
            Request::SaveDropped(file_name, bytes) => self.open_dropped_file(who, file_name, bytes),
            Request::SaveSave(save_game) => self.save_save(who, save_game),
            Request::ReloadSave(path) => self.reload_save(who, path),
            Request::ImportHeadMorph => self.import_head_morph(who),
            Request::ExportHeadMorph(head_morph) => self.export_head_morph(who, head_morph),
        }
    }
}

impl SaveHandler {
    fn open_save(&self, who: HandlerId) {
        self.link.send_future(async move {
            let handle_save = async {
                let has_rpc_file = rpc::open_save().await?;
                let result = match has_rpc_file {
                    Some(rpc_file) => {
                        let RpcFile { path, file } = rpc_file;
                        Self::deserialize(path, file.decode()?).map(Some)?
                    }
                    None => None,
                };
                Ok::<_, Error>(result)
            };

            match handle_save.await.context("Failed to open the save") {
                Ok(Some(save_game)) => Msg::SaveOpened(who, save_game),
                Ok(None) => Msg::DialogCancelled,
                Err(err) => Msg::Error(who, err),
            }
        });
    }

    fn open_dropped_file(&self, who: HandlerId, file_name: String, bytes: Vec<u8>) {
        self.link.send_message({
            let deserialize = || Self::deserialize(file_name.into(), bytes);

            match deserialize().context("Failed to open the save") {
                Ok(save_game) => Msg::SaveOpened(who, save_game),
                Err(err) => Msg::Error(who, err),
            }
        });
    }

    fn save_save(&self, who: HandlerId, save_game: SaveGame) {
        let path = match save_game {
            SaveGame::MassEffect1 { ref file_path, .. }
            | SaveGame::MassEffect1Le { ref file_path, .. }
            | SaveGame::MassEffect1LePs4 { ref file_path, .. }
            | SaveGame::MassEffect2 { ref file_path, .. }
            | SaveGame::MassEffect2Le { ref file_path, .. }
            | SaveGame::MassEffect3 { ref file_path, .. } => file_path.clone(),
        };
        self.link.send_future(async move {
            let handle_save = async {
                let has_path = rpc::save_save_dialog(path).await?;
                let cancelled = match has_path {
                    Some(path) => {
                        let rpc_file = Self::serialize(path, save_game)?;
                        rpc::save_file(rpc_file).await?;
                        false
                    }
                    None => true,
                };
                Ok::<_, Error>(cancelled)
            };

            match handle_save.await.context("Failed to save the save") {
                Ok(false) => Msg::SaveSaved(who),
                Ok(true) => Msg::DialogCancelled,
                Err(err) => Msg::Error(who, err),
            }
        });
    }

    fn reload_save(&self, who: HandlerId, path: PathBuf) {
        self.link.send_future(async move {
            let handle_save = async move {
                let rpc_file = rpc::reload_save(path).await?;
                let RpcFile { path, file } = rpc_file;
                Self::deserialize(path, file.decode()?)
            };

            match handle_save.await.context("Failed to reload the save") {
                Ok(save_game) => Msg::SaveOpened(who, save_game),
                Err(err) => Msg::Error(who, err),
            }
        });
    }

    fn deserialize(file_path: PathBuf, input: Vec<u8>) -> Result<SaveGame> {
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
            } else if ext.eq_ignore_ascii_case("MassEffectSave") {
                // ME1
                SaveGame::MassEffect1 {
                    file_path,
                    save_game: unreal::Deserializer::from_bytes(&input)?,
                }
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
            SaveGame::MassEffect1 { save_game, .. } => unreal::Serializer::to_vec(&save_game)?,
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

        let rpc_file = RpcFile {
            path,
            file: Base64File { unencoded_size: output.len(), base64: base64::encode(output) },
        };

        Ok(rpc_file)
    }

    fn import_head_morph(&self, who: HandlerId) {
        self.link.send_future(async move {
            let handle_save = async {
                let has_rpc_file = rpc::import_head_morph().await?;
                let result = match has_rpc_file {
                    Some(rpc_file) => {
                        let file = String::from_utf8(rpc_file.file.decode()?)?;
                        ron::from_str(&file).map(Some)?
                    }
                    None => None,
                };
                Ok::<_, Error>(result)
            };

            match handle_save.await.context("Failed to import the head morph") {
                Ok(Some(head_morph)) => Msg::HeadMorphImported(who, head_morph),
                Ok(None) => Msg::DialogCancelled,
                Err(err) => Msg::Error(who, err),
            }
        });
    }

    fn export_head_morph(&self, who: HandlerId, head_morph: RcUi<HeadMorph>) {
        self.link.send_future(async move {
            let handle_save = async {
                let has_path = rpc::export_head_morph_dialog().await?;
                let cancelled = match has_path {
                    Some(path) => {
                        let pretty_config = PrettyConfig::new()
                            .with_enumerate_arrays(true)
                            .with_new_line(String::from('\n'));

                        let output = ron::ser::to_string_pretty(&head_morph, pretty_config)?;
                        let rpc_file = RpcFile {
                            path,
                            file: Base64File {
                                unencoded_size: output.len(),
                                base64: base64::encode(output),
                            },
                        };
                        rpc::save_file(rpc_file).await?;
                        false
                    }
                    None => true,
                };
                Ok::<_, Error>(cancelled)
            };

            match handle_save.await.context("Failed to export the head morph") {
                Ok(false) => Msg::HeadMorphExported(who),
                Ok(true) => Msg::DialogCancelled,
                Err(err) => Msg::Error(who, err),
            }
        });
    }
}
