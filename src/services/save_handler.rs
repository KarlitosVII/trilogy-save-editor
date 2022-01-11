use std::{path::PathBuf, rc::Rc};

use anyhow::{bail, Context as ErrorContext, Error, Result};
use crc::{Crc, CRC_32_BZIP2};
use gloo::utils;
use ron::ser::PrettyConfig;
use serde::Deserialize;
use yew::{prelude::*, ContextProvider};

use crate::{
    gui::Theme,
    save_data::mass_effect_1_le::Me1LeMagicNumber,
    save_data::{
        mass_effect_1::{Me1MagicNumber, Me1SaveGame},
        mass_effect_1_le::{Me1LeSaveData, Me1LeSaveGame, Me1LeVersion},
        mass_effect_2::{Me2LeSaveGame, Me2LeVersion, Me2SaveGame, Me2Version},
        mass_effect_3::{Me3SaveGame, Me3Version},
        shared::appearance::HeadMorph,
        RcRef,
    },
    services::rpc::{self, Base64File, DialogParams, RpcFile},
    unreal,
};

use super::drop_handler::DropHandler;

#[derive(Clone)]
pub enum SaveGame {
    MassEffect1 { file_path: PathBuf, save_game: RcRef<Me1SaveGame> },
    MassEffect1Le { file_path: PathBuf, save_game: RcRef<Me1LeSaveGame> },
    MassEffect1LePs4 { file_path: PathBuf, save_game: RcRef<Me1LeSaveData> },
    MassEffect2 { file_path: PathBuf, save_game: RcRef<Me2SaveGame> },
    MassEffect2Le { file_path: PathBuf, save_game: RcRef<Me2LeSaveGame> },
    MassEffect3 { file_path: PathBuf, save_game: RcRef<Me3SaveGame> },
}

pub enum Action {
    OpenSave,
    SaveSave,
    ReloadSave,
    ImportHeadMorph(Callback<HeadMorph>),
    ExportHeadMorph(RcRef<HeadMorph>),
}

pub enum Msg {
    Action(Action),
    SaveOpened(SaveGame),
    SaveDropped(Result<(String, Vec<u8>)>),
    SaveSaved,
    HeadMorphImported(HeadMorph, Callback<HeadMorph>),
    HeadMorphExported,
    Error(Error),
    Noop,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
    pub onnotification: Callback<&'static str>,
    pub onerror: Callback<Error>,
}

#[derive(Clone)]
pub struct SaveHandler {
    pub save_game: Option<Rc<SaveGame>>,
    callback: Callback<Action>,
}

impl SaveHandler {
    pub fn action(&self, action: Action) {
        self.callback.emit(action);
    }
}

impl PartialEq for SaveHandler {
    fn eq(&self, other: &Self) -> bool {
        match (&self.save_game, &other.save_game) {
            (Some(this), Some(other)) => Rc::ptr_eq(this, other),
            (None, None) => true,
            _ => false,
        }
    }
}

pub struct SaveHandlerProvider {
    _drop_handler: DropHandler,
    save_handler: SaveHandler,
}

impl Component for SaveHandlerProvider {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let _drop_handler = DropHandler::new(ctx.link().callback(Msg::SaveDropped));
        let save_handler =
            SaveHandler { save_game: None, callback: ctx.link().callback(Msg::Action) };
        Self::open_command_line_save(ctx);

        SaveHandlerProvider { _drop_handler, save_handler }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            // Actions
            Msg::Action(action) => {
                match action {
                    Action::OpenSave => {
                        let last_dir = self.save_handler.save_game.is_some();
                        Self::open_save(ctx, last_dir);
                    }
                    Action::SaveSave => {
                        if let Some(ref save_game) = self.save_handler.save_game {
                            Self::save_save(ctx, save_game);
                        }
                    }
                    Action::ReloadSave => {
                        if let Some(ref save_game) = self.save_handler.save_game {
                            match save_game.as_ref() {
                                SaveGame::MassEffect1 { file_path, .. }
                                | SaveGame::MassEffect1Le { file_path, .. }
                                | SaveGame::MassEffect1LePs4 { file_path, .. }
                                | SaveGame::MassEffect2 { file_path, .. }
                                | SaveGame::MassEffect2Le { file_path, .. }
                                | SaveGame::MassEffect3 { file_path, .. } => {
                                    Self::reload_save(ctx, file_path.clone())
                                }
                            }
                        }
                    }
                    Action::ImportHeadMorph(callback) => Self::import_head_morph(ctx, callback),
                    Action::ExportHeadMorph(head_morph) => Self::export_head_morph(ctx, head_morph),
                }
                false
            }
            // Messages
            Msg::SaveOpened(save_game) => {
                self.save_handler.save_game = Some(save_game.into());
                self.change_theme();
                ctx.props().onnotification.emit("Opened");
                true
            }
            Msg::SaveDropped(result) => {
                match result {
                    Ok((file_name, bytes)) => Self::open_dropped_file(ctx, file_name, bytes),
                    Err(err) => ctx.props().onerror.emit(err),
                }
                false
            }
            Msg::SaveSaved => {
                ctx.props().onnotification.emit("Saved");
                false
            }
            Msg::HeadMorphImported(head_morph, callback) => {
                callback.emit(head_morph);
                ctx.props().onnotification.emit("Imported");
                false
            }
            Msg::HeadMorphExported => {
                ctx.props().onnotification.emit("Exported");
                false
            }
            Msg::Error(err) => {
                ctx.props().onerror.emit(err);
                false
            }
            Msg::Noop => {
                #[cfg(debug_assertions)]
                gloo::console::log!("No op");
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <ContextProvider<SaveHandler> context={self.save_handler.clone()}>
                { ctx.props().children.clone() }
            </ContextProvider<SaveHandler>>
        }
    }
}

impl SaveHandlerProvider {
    fn open_save(ctx: &Context<Self>, last_dir: bool) {
        ctx.link().send_future(async move {
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
                Ok(Some(save_game)) => Msg::SaveOpened(save_game),
                Ok(None) => Msg::Noop,
                Err(err) => Msg::Error(err),
            }
        });
    }

    fn open_command_line_save(ctx: &Context<Self>) {
        ctx.link().send_future(async move {
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
                Ok(Some(save_game)) => Msg::SaveOpened(save_game),
                Ok(None) => Msg::Noop,
                Err(err) => Msg::Error(err),
            }
        });
    }

    fn open_dropped_file(ctx: &Context<Self>, file_name: String, bytes: Vec<u8>) {
        ctx.link().send_message({
            let deserialize = || Self::deserialize(file_name.into(), bytes);

            match deserialize().context("Failed to open the save") {
                Ok(save_game) => Msg::SaveOpened(save_game),
                Err(err) => Msg::Error(err),
            }
        });
    }

    fn save_save(ctx: &Context<Self>, save_game: &Rc<SaveGame>) {
        let (path, filters) = match save_game.as_ref() {
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

        let save_game = Rc::clone(save_game);
        ctx.link().send_future(async move {
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
                Ok(false) => Msg::SaveSaved,
                Ok(true) => Msg::Noop,
                Err(err) => Msg::Error(err),
            }
        });
    }

    fn reload_save(ctx: &Context<Self>, path: PathBuf) {
        ctx.link().send_future(async move {
            let handle_save = async move {
                let rpc_file = rpc::reload_save(path).await?;
                let RpcFile { path, file } = rpc_file;
                Self::deserialize(path, file.decode()?)
            };

            match handle_save.await.context("Failed to reload the save") {
                Ok(save_game) => Msg::SaveOpened(save_game),
                Err(err) => Msg::Error(err),
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

    fn serialize(path: PathBuf, save_game: Rc<SaveGame>) -> Result<RpcFile> {
        let output = match save_game.as_ref() {
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

    fn import_head_morph(ctx: &Context<Self>, callback: Callback<HeadMorph>) {
        ctx.link().send_future(async move {
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
                Ok(Some(head_morph)) => Msg::HeadMorphImported(head_morph, callback),
                Ok(None) => Msg::Noop,
                Err(err) => Msg::Error(err),
            }
        });
    }

    fn export_head_morph(ctx: &Context<Self>, head_morph: RcRef<HeadMorph>) {
        ctx.link().send_future(async move {
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
                Ok(false) => Msg::HeadMorphExported,
                Ok(true) => Msg::Noop,
                Err(err) => Msg::Error(err),
            }
        });
    }

    fn change_theme(&self) {
        if let Some(ref save_game) = self.save_handler.save_game {
            let theme = match save_game.as_ref() {
                SaveGame::MassEffect1 { .. }
                | SaveGame::MassEffect1Le { .. }
                | SaveGame::MassEffect1LePs4 { .. } => Theme::MassEffect1,
                SaveGame::MassEffect2 { .. } | SaveGame::MassEffect2Le { .. } => Theme::MassEffect2,
                SaveGame::MassEffect3 { .. } => Theme::MassEffect3,
            };

            let body = utils::document().body().unwrap();
            let classes = body.class_list();

            let _ = classes.remove_3(&Theme::MassEffect1, &Theme::MassEffect2, &Theme::MassEffect3);
            let _ = classes.add_1(&theme);
        }
    }
}
