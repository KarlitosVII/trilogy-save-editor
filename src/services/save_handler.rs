use std::path::PathBuf;

use anyhow::{bail, Context, Error, Result};
use crc::{Crc, CRC_32_BZIP2};
use ron::ser::PrettyConfig;
use serde::Deserialize;
use yew_agent::{Agent, AgentLink, HandlerId, Job};

use crate::{
    gui::RcUi,
    save_data::mass_effect_1_le::Me1LeMagicNumber,
    save_data::{
        mass_effect_1::{Me1MagicNumber, Me1SaveGame},
        mass_effect_1_le::{Me1LeSaveData, Me1LeSaveGame, Me1LeVersion},
        mass_effect_2::{Me2LeSaveGame, Me2LeVersion, Me2SaveGame, Me2Version},
        mass_effect_3::{Me3SaveGame, Me3Version},
        shared::appearance::HeadMorph,
    },
    services::rpc::{self, Base64File, DialogParams, RpcFile},
    unreal,
};

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
    Noop,
    Error(HandlerId, Error),
}

pub enum Request {
    OpenSave(bool),
    OpenCommandLineSave,
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
            Msg::Noop => {
                #[cfg(debug_assertions)]
                gloo::console::log!("No op");
            }
            Msg::Error(who, err) => self.link.respond(who, Response::Error(err)),
        }
    }

    fn handle_input(&mut self, msg: Self::Input, who: HandlerId) {
        match msg {
            Request::OpenSave(last_dir) => self.open_save(who, last_dir),
            Request::OpenCommandLineSave => self.open_command_line_save(who),
            Request::SaveDropped(file_name, bytes) => self.open_dropped_file(who, file_name, bytes),
            Request::SaveSave(save_game) => self.save_save(who, save_game),
            Request::ReloadSave(path) => self.reload_save(who, path),
            Request::ImportHeadMorph => self.import_head_morph(who),
            Request::ExportHeadMorph(head_morph) => self.export_head_morph(who, head_morph),
        }
    }
}

impl SaveHandler {
    fn open_save(&self, who: HandlerId, last_dir: bool) {
        self.link.send_future(async move {
            let handle_save = async {
                let has_rpc_file = rpc::open_save(last_dir).await?;
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
                Ok(None) => Msg::Noop,
                Err(err) => Msg::Error(who, err),
            }
        });
    }

    fn open_command_line_save(&self, who: HandlerId) {
        self.link.send_future(async move {
            let handle_save = async {
                let has_rpc_file = rpc::open_command_line_save().await?;
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
                Ok(None) => Msg::Noop,
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
        let (path, filters) = match save_game {
            SaveGame::MassEffect1 { ref file_path, .. } => {
                (file_path.clone(), vec![("Mass Effect 1 save", vec!["MassEffectSave"])])
            }
            SaveGame::MassEffect1Le { ref file_path, .. } => {
                (file_path.clone(), vec![("Mass Effect 1 Legendary PC save", vec!["pcsav"])])
            }
            SaveGame::MassEffect1LePs4 { ref file_path, .. } => {
                (file_path.clone(), vec![("Mass Effect 1 Legendary PS4 save", vec!["ps4sav"])])
            }
            SaveGame::MassEffect2 { ref file_path, .. } => (
                file_path.clone(),
                vec![
                    ("Mass Effect 2 PC save", vec!["pcsav"]),
                    ("Mass Effect 2 XBOX 360 save", vec!["xbsav"]),
                ],
            ),
            SaveGame::MassEffect2Le { ref file_path, .. } => {
                (file_path.clone(), vec![("Mass Effect 2 Legendary save", vec!["pcsav"])])
            }
            SaveGame::MassEffect3 { ref file_path, .. } => (
                file_path.clone(),
                vec![
                    ("Mass Effect 3 PC save", vec!["pcsav"]),
                    ("Mass Effect 3 XBOX 360 save", vec!["xbsav"]),
                ],
            ),
        };
        self.link.send_future(async move {
            let handle_save = async {
                let has_path = rpc::save_save_dialog(DialogParams { path, filters }).await?;
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
                Ok(true) => Msg::Noop,
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
        fn header<'de, T>(header: &'de [u8]) -> Result<T, unreal::Error>
        where
            T: Deserialize<'de>,
        {
            unreal::Deserializer::from_bytes::<T>(header)
        }

        let save_game = if header::<Me1MagicNumber>(&input).is_ok() {
            // ME1
            SaveGame::MassEffect1 {
                file_path,
                save_game: unreal::Deserializer::from_bytes(&input)?,
            }
        } else if header::<Me1LeMagicNumber>(&input).is_ok() {
            // ME1 Legendary
            SaveGame::MassEffect1Le {
                file_path,
                save_game: unreal::Deserializer::from_bytes(&input)?,
            }
        } else if header::<Me1LeVersion>(&input).is_ok() {
            // ME1LE PS4
            SaveGame::MassEffect1LePs4 {
                file_path,
                save_game: unreal::Deserializer::from_bytes(&input)?,
            }
        } else if let Ok(save) = header::<Me2Version>(&input) {
            // ME2
            let save_game = if save.is_xbox360 {
                unreal::Deserializer::from_be_bytes(&input)?
            } else {
                unreal::Deserializer::from_bytes(&input)?
            };
            SaveGame::MassEffect2 { file_path, save_game }
        } else if header::<Me2LeVersion>(&input).is_ok() {
            // ME2 Legendary
            SaveGame::MassEffect2Le {
                file_path,
                save_game: unreal::Deserializer::from_bytes(&input)?,
            }
        } else if let Ok(save) = header::<Me3Version>(&input) {
            // ME3
            let save_game = if save.is_xbox360 {
                unreal::Deserializer::from_be_bytes(&input)?
            } else {
                unreal::Deserializer::from_bytes(&input)?
            };
            SaveGame::MassEffect3 { file_path, save_game }
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
                let is_xbox360 = path
                    .extension()
                    .map(|ext| ext.eq_ignore_ascii_case("xbsav"))
                    .unwrap_or_default();

                let mut output = if is_xbox360 {
                    unreal::Serializer::to_be_vec(&save_game)?
                } else {
                    unreal::Serializer::to_vec(&save_game)?
                };

                let crc = Crc::<u32>::new(&CRC_32_BZIP2);
                let checksum = crc.checksum(&output);

                let extend = if is_xbox360 {
                    u32::to_be_bytes(checksum)
                } else {
                    u32::to_le_bytes(checksum)
                };
                output.extend(extend);
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
                let is_xbox360 = path
                    .extension()
                    .map(|ext| ext.eq_ignore_ascii_case("xbsav"))
                    .unwrap_or_default();

                let mut output = if is_xbox360 {
                    unreal::Serializer::to_be_vec(&save_game)?
                } else {
                    unreal::Serializer::to_vec(&save_game)?
                };

                let crc = Crc::<u32>::new(&CRC_32_BZIP2);
                let checksum = crc.checksum(&output);

                let extend = if is_xbox360 {
                    u32::to_be_bytes(checksum)
                } else {
                    u32::to_le_bytes(checksum)
                };
                output.extend(extend);
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
                        let file = rpc_file.file.decode()?;
                        if file.starts_with(b"GIBBEDMASSEFFECT2HEADMORPH")
                            || file.starts_with(b"GIBBEDMASSEFFECT3HEADMORPH")
                        {
                            // Gibbed's head morph
                            unreal::Deserializer::from_bytes(&file[31..]).map(Some)?
                        } else {
                            // TSE head morph
                            let ron = String::from_utf8(file)?;
                            ron::from_str(&ron).map(Some)?
                        }
                    }
                    None => None,
                };
                Ok::<_, Error>(result)
            };

            match handle_save.await.context("Failed to import the head morph") {
                Ok(Some(head_morph)) => Msg::HeadMorphImported(who, head_morph),
                Ok(None) => Msg::Noop,
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
                        let pretty_config =
                            PrettyConfig::new().enumerate_arrays(true).new_line(String::from('\n'));

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
                Ok(true) => Msg::Noop,
                Err(err) => Msg::Error(who, err),
            }
        });
    }
}
