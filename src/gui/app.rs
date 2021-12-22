use std::cell::Ref;
use std::mem;

use anyhow::{Error, Result};
use gloo::{timers::future::TimeoutFuture, utils};
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::{
    gui::{
        components::{AutoUpdate, NavBar, Tab, TabBar, Table},
        format_code,
        mass_effect_1::{Me1General, Me1Plot, Me1RawData, Me1RawPlot},
        mass_effect_1_le::{Me1LeGeneral, Me1LeInventory},
        mass_effect_2::{Me2General, Me2Plot, Me2RawPlot, Me2Type},
        mass_effect_3::{Me3General, Me3Plot, Me3RawPlot},
        raw_ui::RawUi,
        shared::HeadMorph,
        shared::{FloatPlotType, IntPlotType},
        RcUi, Theme,
    },
    save_data::{
        mass_effect_1::Me1SaveGame, mass_effect_1_le::Me1LeSaveData, mass_effect_3::Me3SaveGame,
    },
    services::{
        database::DatabaseService,
        drop_handler::DropHandler,
        save_handler::{Request, Response, SaveGame, SaveHandler},
    },
};

pub enum Msg {
    OpenSave,
    SaveDropped(Result<(String, Vec<u8>)>),
    SaveOpened(SaveGame),
    SaveSave,
    SaveSaved,
    ReloadSave,
    Notification(&'static str),
    DismissNotification,
    Error(Error),
    DismissError,
}

pub struct App {
    save_handler: Box<dyn Bridge<SaveHandler>>,
    _dbs_service: Box<dyn Bridge<DatabaseService>>,
    _drop_handler: DropHandler,
    notification: Option<&'static str>,
    error: Option<Error>,
    save_game: Option<SaveGame>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let mut save_handler =
            SaveHandler::bridge(ctx.link().callback(|response| match response {
                Response::SaveOpened(save_game) => Msg::SaveOpened(save_game),
                Response::SaveSaved => Msg::SaveSaved,
                Response::Error(err) => Msg::Error(err),
                _ => unreachable!(),
            }));
        save_handler.send(Request::OpenCommandLineSave);

        let dbs_service = DatabaseService::bridge(Callback::noop());
        let drop_handler = DropHandler::new(ctx.link().callback(Msg::SaveDropped));

        App {
            save_handler,
            _dbs_service: dbs_service,
            _drop_handler: drop_handler,
            notification: None,
            error: None,
            save_game: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::OpenSave => {
                let last_dir = self.save_game.is_some();
                self.save_handler.send(Request::OpenSave(last_dir));
                false
            }
            Msg::SaveDropped(result) => {
                match result {
                    Ok((file_name, bytes)) => {
                        self.save_handler.send(Request::SaveDropped(file_name, bytes))
                    }
                    Err(err) => ctx.link().send_message(Msg::Error(err)),
                }
                false
            }
            Msg::SaveOpened(save_game) => {
                self.save_game = Some(save_game);
                self.change_theme();
                ctx.link().send_message(Msg::Notification("Opened"));
                false
            }
            Msg::SaveSave => {
                if let Some(ref save_game) = self.save_game {
                    self.save_handler.send(Request::SaveSave(save_game.clone()));
                }
                false
            }
            Msg::SaveSaved => {
                ctx.link().send_message(Msg::Notification("Saved"));
                false
            }
            Msg::ReloadSave => {
                #[allow(clippy::single_match)]
                match self.save_game {
                    Some(
                        SaveGame::MassEffect1 { ref file_path, .. }
                        | SaveGame::MassEffect1Le { ref file_path, .. }
                        | SaveGame::MassEffect1LePs4 { ref file_path, .. }
                        | SaveGame::MassEffect2 { ref file_path, .. }
                        | SaveGame::MassEffect2Le { ref file_path, .. }
                        | SaveGame::MassEffect3 { ref file_path, .. },
                    ) => {
                        self.save_handler.send(Request::ReloadSave(file_path.clone()));
                    }
                    None => (),
                }
                false
            }
            Msg::Notification(notification) => {
                self.notification = Some(notification);
                ctx.link().send_future(async {
                    TimeoutFuture::new(1500).await;
                    Msg::DismissNotification
                });
                true
            }
            Msg::DismissNotification => {
                self.notification = None;
                true
            }
            Msg::Error(error) => {
                self.error = Some(error);
                true
            }
            Msg::DismissError => {
                self.error = None;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let content = if let Some(ref save_game) = self.save_game {
            match save_game {
                SaveGame::MassEffect1 { save_game, .. } => {
                    App::mass_effect_1(self, ctx, save_game.borrow())
                }
                SaveGame::MassEffect1Le { save_game, .. } => {
                    App::mass_effect_1_le(self, ctx, RcUi::clone(&save_game.borrow().save_data))
                }
                SaveGame::MassEffect1LePs4 { save_game, .. } => {
                    App::mass_effect_1_le(self, ctx, RcUi::clone(save_game))
                }
                SaveGame::MassEffect2 { save_game, .. } => {
                    App::mass_effect_2(self, ctx, Me2Type::Vanilla(RcUi::clone(save_game)))
                }
                SaveGame::MassEffect2Le { save_game, .. } => {
                    App::mass_effect_2(self, ctx, Me2Type::Legendary(RcUi::clone(save_game)))
                }

                SaveGame::MassEffect3 { save_game, .. } => {
                    App::mass_effect_3(self, ctx, RcUi::clone(save_game))
                }
            }
        } else {
            App::changelog()
        };

        let notification =
            self.notification.as_ref().map(|notification| App::notification(self, notification));
        let error = self.error.as_ref().map(|error| App::error(self, ctx, error));

        let link = ctx.link();
        html! {
            <div class="h-[calc(100vh-28px)] flex flex-col">
                <NavBar
                    save_loaded={self.save_game.is_some()}
                    onopen={link.callback(|_| Msg::OpenSave)}
                    onsave={link.callback(|_| Msg::SaveSave)}
                    onreload={link.callback(|_| Msg::ReloadSave)}
                >
                    <AutoUpdate onerror={link.callback(Msg::Error)} />
                </NavBar>
                { content }
                { for notification }
                { for error }
            </div>
        }
    }
}

impl App {
    fn mass_effect_1(&self, ctx: &Context<Self>, save_game: Ref<'_, Me1SaveGame>) -> Html {
        let state = save_game.state();
        let plot = state.plot();

        let link = ctx.link();
        html! {
            <section class="flex-auto flex p-1">
                <TabBar is_main_tab_bar=true>
                    <Tab title="General">
                        <Me1General
                            player={RcUi::clone(&save_game.player)}
                            plot={RcUi::clone(&state.plot)}
                        />
                    </Tab>
                    <Tab title="Plot">
                        <Me1Plot
                            booleans={RcUi::clone(&plot.booleans)}
                            integers={IntPlotType::Vec(RcUi::clone(&plot.integers))}
                            onerror={link.callback(Msg::Error)}
                        />
                    </Tab>
                    <Tab title="Raw Data">
                        <Me1RawData player={RcUi::clone(&save_game.player)} />
                    </Tab>
                    <Tab title="Raw Plot">
                        <Me1RawPlot
                            booleans={RcUi::clone(&plot.booleans)}
                            integers={IntPlotType::Vec(RcUi::clone(&plot.integers))}
                            floats={FloatPlotType::Vec(RcUi::clone(&plot.floats))}
                            onerror={link.callback(Msg::Error)}
                        />
                    </Tab>
                </TabBar>
            </section>
        }
    }

    fn mass_effect_1_le(&self, ctx: &Context<Self>, save_game: RcUi<Me1LeSaveData>) -> Html {
        let me1 = save_game.borrow();
        let plot = me1.plot();
        let head_morph = RcUi::clone(&me1.player().head_morph);

        let link = ctx.link();
        html! {
            <section class="flex-auto flex p-1">
                <TabBar is_main_tab_bar=true>
                    <Tab title="General">
                        <Me1LeGeneral
                            save_game={RcUi::clone(&save_game)}
                            onerror={link.callback(Msg::Error)}
                        />
                    </Tab>
                    <Tab title="Plot">
                        <Me1Plot
                            booleans={RcUi::clone(&plot.booleans)}
                            integers={IntPlotType::Vec(RcUi::clone(&plot.integers))}
                            onerror={link.callback(Msg::Error)}
                        />
                    </Tab>
                    <Tab title="Inventory">
                        <Me1LeInventory
                            player={RcUi::clone(&me1.player)}
                            squad={RcUi::clone(&me1.squad)}
                            onerror={link.callback(Msg::Error)}
                        />
                    </Tab>
                    <Tab title="Head Morph">
                        <HeadMorph {head_morph}
                            onnotification={link.callback(Msg::Notification)}
                            onerror={link.callback(Msg::Error)}
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
                            onerror={link.callback(Msg::Error)}
                        />
                    </Tab>
                </TabBar>
            </section>
        }
    }

    fn mass_effect_2(&self, ctx: &Context<Self>, save_game: Me2Type) -> Html {
        let (raw_data, plot, me1_plot, head_morph) = match save_game {
            Me2Type::Vanilla(ref me2) => (
                me2.view_opened("Mass Effect 2", true),
                RcUi::clone(&me2.borrow().plot),
                RcUi::clone(&me2.borrow().me1_plot),
                RcUi::clone(&me2.borrow().player().appearance().head_morph),
            ),
            Me2Type::Legendary(ref me2) => (
                me2.view_opened("Mass Effect 2", true),
                RcUi::clone(&me2.borrow().plot),
                RcUi::clone(&me2.borrow().me1_plot),
                RcUi::clone(&me2.borrow().player().appearance().head_morph),
            ),
        };
        let (plot, me1_plot) = (plot.borrow(), me1_plot.borrow());

        let link = ctx.link();
        html! {
            <section class="flex-auto flex p-1">
                <TabBar is_main_tab_bar=true>
                    <Tab title="General">
                        <Me2General save_game={Me2Type::clone(&save_game)} />
                    </Tab>
                    <Tab title="Plot">
                        <Me2Plot
                            booleans={RcUi::clone(&plot.booleans)}
                            integers={IntPlotType::Vec(RcUi::clone(&plot.integers))}
                            me1_booleans={RcUi::clone(&me1_plot.booleans)}
                            me1_integers={IntPlotType::Vec(RcUi::clone(&me1_plot.integers))}
                            onerror={link.callback(Msg::Error)}
                        />
                    </Tab>
                    <Tab title="Head Morph">
                        <HeadMorph {head_morph}
                            onnotification={link.callback(Msg::Notification)}
                            onerror={link.callback(Msg::Error)}
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
                            onerror={link.callback(Msg::Error)}
                        />
                    </Tab>
                </TabBar>
            </section>
        }
    }

    fn mass_effect_3(&self, ctx: &Context<Self>, save_game: RcUi<Me3SaveGame>) -> Html {
        let me3 = save_game.borrow();
        let plot = me3.plot();
        let head_morph = RcUi::clone(&me3.player().appearance().head_morph);

        let link = ctx.link();
        html! {
            <section class="flex-auto flex p-1">
                <TabBar is_main_tab_bar=true>
                    <Tab title="General">
                        <Me3General save_game={RcUi::clone(&save_game)} />
                    </Tab>
                    <Tab title="Plot">
                        <Me3Plot
                            booleans={RcUi::clone(&plot.booleans)}
                            integers={IntPlotType::IndexMap(RcUi::clone(&plot.integers))}
                            variables={RcUi::clone(&me3.player_variables)}
                            onerror={link.callback(Msg::Error)}
                        />
                    </Tab>
                    <Tab title="Head Morph">
                        <HeadMorph {head_morph}
                            onnotification={link.callback(Msg::Notification)}
                            onerror={link.callback(Msg::Error)}
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
                            onerror={link.callback(Msg::Error)}
                        />
                    </Tab>
                </TabBar>
            </section>
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
            let changes = changes.into_iter().map(format_code);
            html! {
                <Table title={version} opened={i==0}>
                    { for changes }
                </Table>
            }
        });

        html! {
            <section class="flex-auto flex flex-col gap-1 p-1">
                <div>
                    <p>{ "Changelog" }</p>
                    <hr class="border-t border-default-border" />
                </div>
                <div class="flex-auto flex flex-col gap-1 h-0 overflow-y-auto">
                    { for logs }
                </div>
            </section>
        }
    }

    fn notification(&self, notification: &str) -> Html {
        html! {
            <div class={classes![
                "absolute",
                "top-8",
                "left-1/2",
                "-translate-x-1/2",
                "border",
                "border-default-border",
                "bg-default-bg/50",
                "select-none",
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

    fn error(&self, ctx: &Context<Self>, error: &Error) -> Html {
        let chain = error.chain().skip(1).map(|error| {
            let text = error.to_string();
            let error = text.split_terminator('\n').map(|text| {
                html! { <p>{ format_code(text) }</p> }
            });
            html! {
                <>
                    <hr class="my-0.5 border-t border-default-border" />
                    { for error }
                </>
            }
        });
        html! {
            <div class="absolute w-screen h-[calc(100vh-28px)] grid place-content-center bg-white/30 z-50">
                <div class="border border-default-border bg-default-bg max-w-xl">
                    <div class="px-1 bg-theme-tab select-none">{"Error"}</div>
                    <div class="p-1 pt-0.5">
                        { format_code(error.to_string()) }
                        { for chain }
                        <hr class="my-0.5 border-t border-default-border" />
                        <button class="button w-12"
                            onclick={ctx.link().callback(|_| Msg::DismissError)}
                        >
                            {"OK"}
                        </button>
                    </div>
                </div>
            </div>
        }
    }

    fn change_theme(&self) {
        if let Some(ref save_game) = self.save_game {
            let theme = match save_game {
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
