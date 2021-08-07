use anyhow::{anyhow, Error};
use std::mem;
use yew::{prelude::*, services::ConsoleService};

use crate::{
    database_service::DatabaseService,
    gui::{
            shared::{FloatPlotType, IntPlotType},
        components::{
            NavBar, Tab, TabBar, Table,
        },
        mass_effect_1::{Me1Plot, Me1RawPlot},
        mass_effect_1_le::Me1LeGeneral,
        mass_effect_2::{Me2General, Me2Plot, Me2RawPlot, Me2Type},
        mass_effect_3::{Me3General, Me3Plot, Me3RawPlot},
        raw_ui::RawUi,
        RcUi, Theme,
    },
    save_data::{mass_effect_1_le::Me1LeSaveData, mass_effect_3::Me3SaveGame},
    save_handler::{Request, Response, SaveGame, SaveHandler},
};

pub enum Msg {
    OpenSave,
    SaveOpened(SaveGame),
    SaveSave,
    ReloadSave,
    Error(Error),
    Notification(String),
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
    _dbs_service: Box<dyn Bridge<DatabaseService>>,
}

impl Component for App {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let save_handle = SaveHandler::bridge(link.callback(|response| match response {
            Response::SaveOpened(save_game) => Msg::SaveOpened(save_game),
            Response::Error(err) => Msg::Error(err),
        }));

        let _dbs_service = DatabaseService::bridge(link.callback(|_| {
            Msg::Error(anyhow!("Database service should not send a message to App component"))
        }));

        App { props, link, save_handle, _dbs_service }
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
                    Some(
                        SaveGame::MassEffect1Le { ref file_path, .. }
                        | SaveGame::MassEffect1LePs4 { ref file_path, .. }
                        | SaveGame::MassEffect2 { ref file_path, .. }
                        | SaveGame::MassEffect2Le { ref file_path, .. }
                        | SaveGame::MassEffect3 { ref file_path, .. },
                    ) => file_path.to_owned(),
                    None => unreachable!(),
                };

                self.save_handle.send(Request::OpenSave(file_path));
                false
            }
            Msg::Error(err) => {
                // TODO: Error
                ConsoleService::warn(&err.to_string());
                false
            }
            Msg::Notification(_) => {
                // TODO: Notification
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let (content, theme) = if let Some(ref save_game) = self.props.save_game {
            match save_game {
                SaveGame::MassEffect1Le { save_game, .. } => (
                    App::mass_effect_1_le(self, RcUi::clone(&save_game.borrow().save_data)),
                    Theme::MassEffect1,
                ),
                SaveGame::MassEffect1LePs4 { save_game, .. } => {
                    (App::mass_effect_1_le(self, RcUi::clone(save_game)), Theme::MassEffect1)
                }
                SaveGame::MassEffect2 { save_game, .. } => (
                    App::mass_effect_2(self, Me2Type::Vanilla(RcUi::clone(save_game))),
                    Theme::MassEffect2,
                ),
                SaveGame::MassEffect2Le { save_game, .. } => (
                    App::mass_effect_2(self, Me2Type::Legendary(RcUi::clone(save_game))),
                    Theme::MassEffect2,
                ),
                SaveGame::MassEffect3 { save_game, .. } => {
                    (App::mass_effect_3(self, RcUi::clone(save_game)), Theme::MassEffect3)
                }
            }
        } else {
            (App::changelog(), Theme::MassEffect3)
        };

        html! {
            <div class=classes![
                "h-screen",
                "flex",
                "flex-col",
                "font-default",
                "text-[80%]",
                "leading-[20px]",
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
                    changelog.push((version, mem::take(&mut changes)));
                }
            }
            changelog
        };

        let logs = changelog.into_iter().enumerate().map(|(i, (version, changes))| {
            html! {
                <Table title=version.to_owned() opened=i==0>
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

    fn mass_effect_1_le(&self, save_game: RcUi<Me1LeSaveData>) -> Html {
        let me1 = save_game.borrow();
        let plot = me1.plot();

        html! {
            <section class="flex-auto flex p-1">
                <TabBar>
                    <Tab title="General">
                        <Me1LeGeneral save_game=RcUi::clone(&save_game) />
                    </Tab>
                    <Tab title="Plot">
                        <Me1Plot
                            booleans=RcUi::clone(&plot.booleans)
                            integers=IntPlotType::Vec(RcUi::clone(&plot.integers))
                            onerror=self.link.callback(Msg::Error)
                        />
                    </Tab>
                    <Tab title="Raw Data">
                        { save_game.view_opened("Mass Effect 1", true) }
                    </Tab>
                    <Tab title="Raw Plot">
                        <Me1RawPlot
                            booleans=RcUi::clone(&plot.booleans)
                            integers=IntPlotType::Vec(RcUi::clone(&plot.integers))
                            floats=FloatPlotType::Vec(RcUi::clone(&plot.floats))
                            onerror=self.link.callback(Msg::Error)
                        />
                    </Tab>
                </TabBar>
            </section>
        }
    }

    fn mass_effect_2(&self, save_game: Me2Type) -> Html {
        let (raw_data, plot, me1_plot) = match save_game {
            Me2Type::Vanilla(ref me2) => (
                me2.view_opened("Mass Effect 2", true),
                RcUi::clone(&me2.borrow().plot),
                RcUi::clone(&me2.borrow().me1_plot),
            ),
            Me2Type::Legendary(ref me2) => (
                me2.view_opened("Mass Effect 2", true),
                RcUi::clone(&me2.borrow().plot),
                RcUi::clone(&me2.borrow().me1_plot),
            ),
        };

        html! {
            <section class="flex-auto flex p-1">
                <TabBar>
                    <Tab title="General">
                        <Me2General save_game=Me2Type::clone(&save_game) />
                    </Tab>
                    <Tab title="Plot">
                        <Me2Plot
                            booleans=RcUi::clone(&plot.borrow().booleans)
                            integers=IntPlotType::Vec(RcUi::clone(&plot.borrow().integers))
                            me1_booleans=RcUi::clone(&me1_plot.borrow().booleans)
                            me1_integers=IntPlotType::Vec(RcUi::clone(&me1_plot.borrow().integers))
                            onerror=self.link.callback(Msg::Error)
                        />
                    </Tab>
                    <Tab title="Raw Data">
                        { raw_data }
                    </Tab>
                    <Tab title="Raw Plot">
                        <Me2RawPlot
                            booleans=RcUi::clone(&plot.borrow().booleans)
                            integers=IntPlotType::Vec(RcUi::clone(&plot.borrow().integers))
                            floats=FloatPlotType::Vec(RcUi::clone(&plot.borrow().floats))
                            onerror=self.link.callback(Msg::Error)
                        />
                    </Tab>
                </TabBar>
            </section>
        }
    }

    fn mass_effect_3(&self, save_game: RcUi<Me3SaveGame>) -> Html {
        let me3 = save_game.borrow();
        let plot = me3.plot();

        html! {
            <section class="flex-auto flex p-1">
                <TabBar>
                    <Tab title="General">
                        <Me3General save_game=RcUi::clone(&save_game) />
                    </Tab>
                    <Tab title="Plot">
                        <Me3Plot
                            booleans=RcUi::clone(&plot.booleans)
                            integers=IntPlotType::IndexMap(RcUi::clone(&plot.integers))
                            variables=RcUi::clone(&me3.player_variables)
                            onerror=self.link.callback(Msg::Error)
                        />
                    </Tab>
                    <Tab title="Raw Data">
                        { save_game.view_opened("Mass Effect 3", true) }
                    </Tab>
                    <Tab title="Raw Plot">
                        <Me3RawPlot
                            booleans=RcUi::clone(&plot.booleans)
                            integers=IntPlotType::IndexMap(RcUi::clone(&plot.integers))
                            floats=FloatPlotType::IndexMap(RcUi::clone(&plot.floats))
                            onerror=self.link.callback(Msg::Error)
                        />
                    </Tab>
                </TabBar>
            </section>
        }
    }
}
