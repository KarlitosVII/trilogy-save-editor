use anyhow::Error;
use std::mem;
use yew::prelude::*;

use crate::{
    agents::{Request, Response, SaveGame, SaveHandler},
    gui::{components::*, mass_effect_2::Me2General, raw_ui::RawUi},
    save_data::mass_effect_2::{Me2LeSaveGame, Me2SaveGame},
};

use super::RcUi;

#[derive(Clone)]
pub enum Me2Type {
    Vanilla(RcUi<Me2SaveGame>),
    Legendary(RcUi<Me2LeSaveGame>),
}

impl PartialEq for Me2Type {
    fn eq(&self, other: &Me2Type) -> bool {
        match self {
            Me2Type::Vanilla(me2) => match other {
                Me2Type::Vanilla(other) => me2.ptr_eq(other),
                _ => false,
            },
            Me2Type::Legendary(me2) => match other {
                Me2Type::Legendary(other) => me2.ptr_eq(other),
                _ => false,
            },
        }
    }
}

pub enum Msg {
    OpenSave,
    SaveOpened(SaveGame),
    SaveSave,
    ReloadSave,
    Error(Error),
}

#[derive(Properties, Clone, Default)]
pub struct Props {
    #[prop_or_default]
    save_game: Option<SaveGame>,
}

pub struct App {
    props: Props,
    link: ComponentLink<Self>,
    save_handle: Box<dyn Bridge<SaveHandler>>,
}

impl Component for App {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|response| match response {
            Response::SaveOpened(save_game) => Msg::SaveOpened(save_game),
            Response::Error(err) => Msg::Error(err),
        });
        let save_handle = SaveHandler::bridge(callback);

        App { props, link, save_handle }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OpenSave => {
                self.save_handle.send(Request::OpenSave(String::from("test/ME2LeSave.pcsav")));
                false
            }
            Msg::SaveOpened(save_game) => {
                self.props.save_game = Some(save_game);
                true
            }
            Msg::SaveSave => false,
            Msg::ReloadSave => {
                let file_path = match self.props.save_game {
                    Some(SaveGame::MassEffect2Le { ref file_path, .. }) => file_path.to_owned(),
                    None => return false,
                };

                self.save_handle.send(Request::OpenSave(file_path));
                false
            }
            Msg::Error(_) => todo!(),
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let (content, theme) = if let Some(ref save_game) = self.props.save_game {
            match save_game {
                SaveGame::MassEffect2Le { save_game, .. } => {
                    (App::mass_effect_2(Me2Type::Legendary(RcUi::clone(save_game))), "me2")
                }
            }
        } else {
            (App::changelog(), "me3")
        };

        html! {
            <div class=classes![
                "h-screen",
                "flex",
                "flex-col",
                "font-mono",
                "text-base",
                "leading-tight",
                "text-white",
                theme,
            ]>
                <NavBar
                    save_loaded=self.props.save_game.is_some()
                    onopen=self.link.callback(|_| Msg::OpenSave)
                    onsave=self.link.callback(|_| Msg::SaveSave)
                    onreload=self.link.callback(|_| Msg::ReloadSave)
                />
                { content }
            </div>
        }
    }
}

impl App {
    fn changelog() -> Html {
        let changelog = {
            let file = include_str!("../../CHANGELOG.md");
            let mut changelog = Vec::new();
            let mut changes = Vec::new();
            let mut version = "";

            for line in file.split('\n') {
                if let Some((prefix, text)) = line.split_once(' ') {
                    match prefix {
                        "##" => version = text,
                        "-" | "*" => changes.push(text),
                        _ => {}
                    }
                } else {
                    changelog.push((version, mem::replace(&mut changes, Vec::new())));
                }
            }
            changelog
        };

        let logs = changelog.into_iter().enumerate().map(|(i, (version, changes))| {
            html! {
                <Table title=Some(version.to_owned()) opened=i==0>
                    { for changes }
                </Table>
            }
        });

        html! {
            <section class="flex-auto flex flex-col gap-1 p-1">
                <div>
                    { "Changelog" }
                    <hr class="border-t border-default-border" />
                </div>
                { for logs }
            </section>
        }
    }

    fn mass_effect_2(me2: Me2Type) -> Html {
        let raw_data = match me2 {
            Me2Type::Vanilla(ref me2) => me2.view_opened("Mass Effect 2", true),
            Me2Type::Legendary(ref me2) => me2.view_opened("Mass Effect 2", true),
        };

        html! {
            <section class="flex-auto flex p-1">
                <TabBar>
                    <Tab title="Général">
                        <Me2General save_game=Me2Type::clone(&me2) />
                    </Tab>
                    <Tab title="Raw Data">
                        { raw_data }
                    </Tab>
                </TabBar>
            </section>
        }
    }
}
