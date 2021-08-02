use anyhow::Error;
use yew::agent::{Agent, AgentLink, HandlerId, Job};

use crate::{
    gui::RcUi,
    save_data::mass_effect_2::{Me2LeSaveGame, Me2SaveGame},
    unreal,
};

#[derive(Clone)]
pub enum SaveGame {
    // MassEffect1 { file_path: String, save_game: RcUi<Me1SaveGame> },
    // MassEffect1Le { file_path: String, save_game: RcUi<Me1LeSaveGame> },
    // MassEffect1LePs4 { file_path: String, save_game: RcUi<Me1LeSaveData> },
    MassEffect2 { file_path: String, save_game: RcUi<Me2SaveGame> },
    MassEffect2Le { file_path: String, save_game: RcUi<Me2LeSaveGame> },
    // MassEffect3 { file_path: String, save_game: RcUi<Me3SaveGame> },
}

pub enum Request {
    OpenSave(String),
}

pub enum Response {
    SaveOpened(SaveGame),
    Error(Error),
}

pub struct SaveHandler {
    link: AgentLink<SaveHandler>,
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
        let handle_request = || match msg {
            Request::OpenSave(file_path) => {
                let save_game: RcUi<Me2LeSaveGame> =
                    unreal::Deserializer::from_bytes(include_bytes!("../../test/ME2LeSave.pcsav"))?;
                let response = SaveGame::MassEffect2Le { file_path, save_game };
                self.link.respond(who, Response::SaveOpened(response));
                Ok(())
            }
        };

        if let Err(err) = handle_request() {
            self.link.respond(who, Response::Error(err));
        }
    }
}
