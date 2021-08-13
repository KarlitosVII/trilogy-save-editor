use std::path::PathBuf;

use anyhow::{bail, Context, Error, Result};
use yew_agent::{Agent, AgentLink, HandlerId, Job};

use crate::gui::RcUi;
use crate::rpc::RpcFile;
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
    Error(HandlerId, Error),
}

pub enum Request {
    OpenSave,
    ReloadSave(PathBuf),
}

pub enum Response {
    SaveOpened(SaveGame),
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
            Msg::Error(who, err) => self.link.respond(who, Response::Error(err)),
        }
    }

    fn handle_input(&mut self, msg: Self::Input, who: HandlerId) {
        match msg {
            Request::OpenSave => self.open_save(who),
            Request::ReloadSave(path) => self.reload_save(who, path),
        }
    }
}

impl SaveHandler {
    fn open_save(&self, who: HandlerId) {
        self.link.send_future(async move {
            let handle_save = async {
                let rpc_file = rpc::open().await?;
                Self::rpc_file_to_save_game(rpc_file)
            };

            match handle_save.await.context("Failed to open the save") {
                Ok(save_game) => Msg::SaveOpened(who, save_game),
                Err(err) => Msg::Error(who, err),
            }
        });
    }

    fn reload_save(&self, who: HandlerId, path: PathBuf) {
        self.link.send_future(async move {
            let handle_save = async move {
                let rpc_file = rpc::reload(path).await?;
                Self::rpc_file_to_save_game(rpc_file)
            };

            match handle_save.await.context("Failed to reload the save") {
                Ok(save_game) => Msg::SaveOpened(who, save_game),
                Err(err) => Msg::Error(who, err),
            }
        });
    }

    fn rpc_file_to_save_game(rpc_file: RpcFile) -> Result<SaveGame> {
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
}
