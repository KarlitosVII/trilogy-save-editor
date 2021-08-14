use std::mem;

use anyhow::{anyhow, Error};
use gloo::timers::future::TimeoutFuture;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::database_service::DatabaseService;
use crate::gui::{
    components::{NavBar, Tab, TabBar, Table},
    mass_effect_1::{Me1Plot, Me1RawPlot},
    mass_effect_1_le::{Me1LeGeneral, Me1LeInventory},
    mass_effect_2::{Me2General, Me2Plot, Me2RawPlot, Me2Type},
    mass_effect_3::{Me3General, Me3Plot, Me3RawPlot},
    raw_ui::RawUi,
    shared::{FloatPlotType, IntPlotType},
    RcUi, Theme,
};
use crate::save_data::{mass_effect_1_le::Me1LeSaveData, mass_effect_3::Me3SaveGame};
use crate::save_handler::{Request, Response, SaveGame, SaveHandler};

pub enum Msg {
    OpenSave,
    SaveOpened(SaveGame),
    SaveSave,
    SaveSaved,
    ReloadSave,
    Notification(&'static str),
    CloseNotification,
    Error(Error),
    CloseError,
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
    notification: Option<&'static str>,
    error: Option<Error>,
}

impl Component for App {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let save_handle = SaveHandler::bridge(link.callback(|response| match response {
            Response::SaveOpened(save_game) => Msg::SaveOpened(save_game),
            Response::Error(err) => Msg::Error(err),
            Response::SaveSaved => Msg::SaveSaved,
        }));

        let _dbs_service = DatabaseService::bridge(link.callback(|_| {
            Msg::Error(anyhow!("Database service should not send a message to App component"))
        }));

        App { props, link, save_handle, _dbs_service, notification: None, error: None }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OpenSave => {
                self.save_handle.send(Request::OpenSave);
                false
            }
            Msg::SaveOpened(save_game) => {
                self.props.save_game = Some(save_game);
                self.link.send_message(Msg::Notification("Opened"));
                false
            }
            Msg::SaveSave => {
                if let Some(ref save_game) = self.props.save_game {
                    self.save_handle.send(Request::SaveSave(save_game.clone()));
                }
                false
            }
            Msg::SaveSaved => {
                self.link.send_message(Msg::Notification("Saved"));
                false
            }
            Msg::ReloadSave => {
                match self.props.save_game {
                    Some(
                        SaveGame::MassEffect1Le { ref file_path, .. }
                        | SaveGame::MassEffect1LePs4 { ref file_path, .. }
                        | SaveGame::MassEffect2 { ref file_path, .. }
                        | SaveGame::MassEffect2Le { ref file_path, .. }
                        | SaveGame::MassEffect3 { ref file_path, .. },
                    ) => {
                        self.save_handle.send(Request::ReloadSave(file_path.to_owned()));
                    }
                    None => (),
                }
                false
            }
            Msg::Notification(notification) => {
                self.notification = Some(notification);
                self.link.send_future(async {
                    TimeoutFuture::new(1500).await;
                    Msg::CloseNotification
                });
                true
            }
            Msg::CloseNotification => {
                self.notification = None;
                true
            }
            Msg::Error(error) => {
                self.error = Some(error);
                true
            }
            Msg::CloseError => {
                self.error = None;
                true
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

        let notification =
            self.notification.as_ref().map(|notification| App::notification(self, notification));
        let error = self.error.as_ref().map(|error| App::error(self, error));

        html! {
            <div class={classes![
                "h-screen",
                "flex",
                "flex-col",
                "font-default",
                "text-[80%]",
                "leading-[20px]",
                "text-white",
                theme,
            ]}>
                <NavBar
                    save_loaded={self.props.save_game.is_some()}
                    onopen={self.link.callback(|_| Msg::OpenSave)}
                    onsave={self.link.callback(|_| Msg::SaveSave)}
                    onreload={self.link.callback(|_| Msg::ReloadSave)}
                />
                { content }
                { for notification }
                { for error }
            </div>
        }
    }
}

impl App {
    fn notification(&self, notification: &str) -> Html {
        html! {
            <div class={classes![
                "absolute",
                "top-1.5",
                "left-1/2",
                "-translate-x-1/2",
                "border",
                "border-default-border",
                "bg-default-bg/50",
                "px-2",
                "pt-0.5",
                "pb-1.5",
                "z-50"
            ]}>
                { notification }
                <div class="relative h-0.5 bg-theme-bg">
                    <div class="absolute notification-animation w-full h-0.5 bg-white"></div>
                </div>
            </div>
        }
    }

    fn error(&self, error: &Error) -> Html {
        let chain = error.chain().skip(1).map(|error| {
            html! {
                <>
                    <hr class="mt-0.5 border-t border-default-border" />
                    { error }
                </>
            }
        });
        html! {
            <div class="absolute w-screen h-screen grid place-content-center bg-white/30 z-50">
                <div class="border border-default-border bg-default-bg">
                    <div class="px-1 bg-theme-tab select-none">{"Error"}</div>
                    <div class="p-1 pt-0.5">
                        { error }
                        { for chain }
                        <hr class="my-0.5 border-t border-default-border" />
                        <button class="button w-12"
                            onclick={self.link.callback(|_| Msg::CloseError)}
                        >
                            {"OK"}
                        </button>
                    </div>
                </div>
            </div>
        }
    }

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
                <Table title={version.to_owned()} opened={i==0}>
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
                <div class="flex-auto flex flex-col gap-1 h-0 overflow-y-auto">
                    { for logs }
                </div>
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
                        <Me1LeGeneral save_game={RcUi::clone(&save_game)} />
                    </Tab>
                    <Tab title="Plot">
                        <Me1Plot
                            booleans={RcUi::clone(&plot.booleans)}
                            integers={IntPlotType::Vec(RcUi::clone(&plot.integers))}
                            onerror={self.link.callback(Msg::Error)}
                        />
                    </Tab>
                    <Tab title="Inventory">
                        <Me1LeInventory
                            player={RcUi::clone(&me1.player)}
                            squad={RcUi::clone(&me1.squad)}
                            onerror={self.link.callback(Msg::Error)}
                        />
                    </Tab>
                    <Tab title="Raw Data">
                        { save_game.view_opened("Mass Effect 1", true) }
                    </Tab>
                    <Tab title="Raw Plot">
                        <Me1RawPlot
                            booleans={RcUi::clone(&plot.booleans)}
                            integers={IntPlotType::Vec(RcUi::clone(&plot.integers))}
                            floats={FloatPlotType::Vec(RcUi::clone(&plot.floats))}
                            onerror={self.link.callback(Msg::Error)}
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
        let (plot, me1_plot) = (plot.borrow(), me1_plot.borrow());

        html! {
            <section class="flex-auto flex p-1">
                <TabBar>
                    <Tab title="General">
                        <Me2General save_game={Me2Type::clone(&save_game)} />
                    </Tab>
                    <Tab title="Plot">
                        <Me2Plot
                            booleans={RcUi::clone(&plot.booleans)}
                            integers={IntPlotType::Vec(RcUi::clone(&plot.integers))}
                            me1_booleans={RcUi::clone(&me1_plot.booleans)}
                            me1_integers={IntPlotType::Vec(RcUi::clone(&me1_plot.integers))}
                            onerror={self.link.callback(Msg::Error)}
                        />
                    </Tab>
                    <Tab title="Raw Data">
                        { raw_data }
                    </Tab>
                    <Tab title="Raw Plot">
                        <Me2RawPlot
                            booleans={RcUi::clone(&plot.booleans)}
                            integers={IntPlotType::Vec(RcUi::clone(&plot.integers))}
                            floats={FloatPlotType::Vec(RcUi::clone(&plot.floats))}
                            onerror={self.link.callback(Msg::Error)}
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
                        <Me3General save_game={RcUi::clone(&save_game)} />
                    </Tab>
                    <Tab title="Plot">
                        <Me3Plot
                            booleans={RcUi::clone(&plot.booleans)}
                            integers={IntPlotType::IndexMap(RcUi::clone(&plot.integers))}
                            variables={RcUi::clone(&me3.player_variables)}
                            onerror={self.link.callback(Msg::Error)}
                        />
                    </Tab>
                    <Tab title="Raw Data">
                        { save_game.view_opened("Mass Effect 3", true) }
                    </Tab>
                    <Tab title="Raw Plot">
                        <Me3RawPlot
                            booleans={RcUi::clone(&plot.booleans)}
                            integers={IntPlotType::IndexMap(RcUi::clone(&plot.integers))}
                            floats={FloatPlotType::IndexMap(RcUi::clone(&plot.floats))}
                            onerror={self.link.callback(Msg::Error)}
                        />
                    </Tab>
                </TabBar>
            </section>
        }
    }
}
