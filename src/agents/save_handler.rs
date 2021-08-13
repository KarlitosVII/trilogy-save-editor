use anyhow::{Error, Result};
use yew_agent::{Agent, AgentLink, HandlerId, Job};

use crate::gui::RcUi;
use crate::save_data::{
    mass_effect_1_le::{Me1LeSaveData, Me1LeSaveGame},
    mass_effect_2::{Me2LeSaveGame, Me2SaveGame},
    mass_effect_3::Me3SaveGame,
};
use crate::unreal;

#[derive(Clone)]
pub enum SaveGame {
    // MassEffect1 { file_path: String, save_game: RcUi<Me1SaveGame> },
    MassEffect1Le { file_path: String, save_game: RcUi<Me1LeSaveGame> },
    MassEffect1LePs4 { file_path: String, save_game: RcUi<Me1LeSaveData> },
    MassEffect2 { file_path: String, save_game: RcUi<Me2SaveGame> },
    MassEffect2Le { file_path: String, save_game: RcUi<Me2LeSaveGame> },
    MassEffect3 { file_path: String, save_game: RcUi<Me3SaveGame> },
}

pub enum Request {
    OpenSave(String),
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
    type Message = ();
    type Input = Request;
    type Output = Response;

    fn create(link: AgentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, who: HandlerId) {
        let result = match msg {
            Request::OpenSave(file_path) => self.open_save(who, file_path),
        };

        if let Err(err) = result {
            self.link.respond(who, Response::Error(err));
        }
    }
}

impl SaveHandler {
    fn open_save(&self, who: HandlerId, file_path: String) -> Result<()> {
        let save_game: RcUi<Me3SaveGame> =
            unreal::Deserializer::from_bytes(include_bytes!("../../test/ME3Save.pcsav"))?;
        let response = SaveGame::MassEffect3 { file_path, save_game };
        self.link.respond(who, Response::SaveOpened(response));
        Ok(())
    }
}
