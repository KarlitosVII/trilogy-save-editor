use std::cell::Ref;
use std::mem;

use anyhow::Error;
use gloo::timers::future::TimeoutFuture;
use yew::prelude::*;

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
        RcUi,
    },
    save_data::{
        mass_effect_1::Me1SaveGame, mass_effect_1_le::Me1LeSaveData, mass_effect_3::Me3SaveGame,
    },
    services::{
        database::DatabaseProvider,
        save_handler::{SaveGame, SaveHandler, SaveHandlerProvider},
    },
};

pub enum Msg {
    Notification(&'static str),
    DismissNotification,
    Error(Error),
    DismissError,
}

pub struct App {
    notification: Option<&'static str>,
    error: Option<Error>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App { notification: None, error: None }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
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
        let notification =
            self.notification.as_ref().map(|notification| Self::notification(notification));
        let error = self.error.as_ref().map(|error| Self::error(ctx, error));

        let link = ctx.link();
        html! {
            <div class="h-[calc(100vh-28px)] flex flex-col">
                <SaveHandlerProvider
                    onnotification={link.callback(Msg::Notification)}
                    onerror={link.callback(Msg::Error)}
                >
                    <NavBar>
                        <AutoUpdate onerror={link.callback(Msg::Error)} />
                    </NavBar>
                    <DatabaseProvider onerror={link.callback(Msg::Error)}>
                        <SaveContent/>
                    </DatabaseProvider>
                </SaveHandlerProvider>
                { for notification }
                { for error }
            </div>
        }
    }
}

impl App {
    fn notification(notification: &str) -> Html {
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

    fn error(ctx: &Context<Self>, error: &Error) -> Html {
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
}

#[function_component(SaveContent)]
fn save_content() -> Html {
    let save_handler = use_context::<SaveHandler>().expect("no save handler provider");
    if let Some(save_game) = save_handler.save_game {
        match save_game.as_ref() {
            SaveGame::MassEffect1 { save_game, .. } => mass_effect_1(save_game.borrow()),
            SaveGame::MassEffect1Le { save_game, .. } => {
                mass_effect_1_le(RcUi::clone(&save_game.borrow().save_data))
            }
            SaveGame::MassEffect1LePs4 { save_game, .. } => {
                mass_effect_1_le(RcUi::clone(save_game))
            }
            SaveGame::MassEffect2 { save_game, .. } => {
                mass_effect_2(Me2Type::Vanilla(RcUi::clone(save_game)))
            }
            SaveGame::MassEffect2Le { save_game, .. } => {
                mass_effect_2(Me2Type::Legendary(RcUi::clone(save_game)))
            }

            SaveGame::MassEffect3 { save_game, .. } => mass_effect_3(RcUi::clone(save_game)),
        }
    } else {
        changelog()
    }
}

fn mass_effect_1(save_game: Ref<'_, Me1SaveGame>) -> Html {
    let state = save_game.state();
    let plot = state.plot();

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
                    />
                </Tab>
            </TabBar>
        </section>
    }
}

fn mass_effect_1_le(save_game: RcUi<Me1LeSaveData>) -> Html {
    let me1 = save_game.borrow();
    let plot = me1.plot();
    let head_morph = RcUi::clone(&me1.player().head_morph);

    html! {
        <section class="flex-auto flex p-1">
            <TabBar is_main_tab_bar=true>
                <Tab title="General">
                    <Me1LeGeneral save_game={RcUi::clone(&save_game)} />
                </Tab>
                <Tab title="Plot">
                    <Me1Plot
                        booleans={RcUi::clone(&plot.booleans)}
                        integers={IntPlotType::Vec(RcUi::clone(&plot.integers))}
                    />
                </Tab>
                <Tab title="Inventory">
                    <Me1LeInventory
                        player={RcUi::clone(&me1.player)}
                        squad={RcUi::clone(&me1.squad)}
                    />
                </Tab>
                <Tab title="Head Morph">
                    <HeadMorph {head_morph} />
                </Tab>
                <Tab title="Raw Data">
                    { save_game.view_opened("Mass Effect 1", true) }
                </Tab>
                <Tab title="Raw Plot">
                    <Me1RawPlot
                        booleans={RcUi::clone(&plot.booleans)}
                        integers={IntPlotType::Vec(RcUi::clone(&plot.integers))}
                        floats={FloatPlotType::Vec(RcUi::clone(&plot.floats))}
                    />
                </Tab>
            </TabBar>
        </section>
    }
}

fn mass_effect_2(save_game: Me2Type) -> Html {
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
                    />
                </Tab>
                <Tab title="Head Morph">
                    <HeadMorph {head_morph} />
                </Tab>
                <Tab title="Raw Data">
                    { raw_data }
                </Tab>
                <Tab title="Raw Plot">
                    <Me2RawPlot
                        booleans={RcUi::clone(&plot.booleans)}
                        integers={IntPlotType::Vec(RcUi::clone(&plot.integers))}
                        floats={FloatPlotType::Vec(RcUi::clone(&plot.floats))}
                    />
                </Tab>
            </TabBar>
        </section>
    }
}

fn mass_effect_3(save_game: RcUi<Me3SaveGame>) -> Html {
    let me3 = save_game.borrow();
    let plot = me3.plot();
    let head_morph = RcUi::clone(&me3.player().appearance().head_morph);

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
                    />
                </Tab>
                <Tab title="Head Morph">
                    <HeadMorph {head_morph} />
                </Tab>
                <Tab title="Raw Data">
                    { save_game.view_opened("Mass Effect 3", true) }
                </Tab>
                <Tab title="Raw Plot">
                    <Me3RawPlot
                        booleans={RcUi::clone(&plot.booleans)}
                        integers={IntPlotType::IndexMap(RcUi::clone(&plot.integers))}
                        floats={FloatPlotType::IndexMap(RcUi::clone(&plot.floats))}
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
