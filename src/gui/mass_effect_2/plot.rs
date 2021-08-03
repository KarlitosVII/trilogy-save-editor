use anyhow::Error;
use indexmap::IndexMap;
use std::rc::Rc;
use yew::prelude::*;
use yewtil::NeqAssign;

use crate::{
    database_service::{Database, DatabaseService, Request, Response, Type},
    gui::{
        components::{
            shared::{IntegerPlotType, PlotCategory},
            Tab, TabBar,
        },
        mass_effect_1::Me1Plot,
        RcUi, Theme,
    },
    save_data::{
        mass_effect_2::plot_db::Me2PlotDb,
        shared::plot::{BitVec, PlotCategory as PlotCategoryDb},
    },
};

pub enum Msg {
    PlotDb(Rc<Me2PlotDb>),
    Error(Error),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub booleans: RcUi<BitVec>,
    pub integers: IntegerPlotType,
    pub me1_booleans: RcUi<BitVec>,
    pub me1_integers: IntegerPlotType,
    pub onerror: Callback<Error>,
}

pub struct Me2Plot {
    props: Props,
    link: ComponentLink<Self>,
    _database_service: Box<dyn Bridge<DatabaseService>>,
    plot_db: Option<Rc<Me2PlotDb>>,
}

impl Component for Me2Plot {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut database_service =
            DatabaseService::bridge(link.callback(|response| match response {
                Response::Database(Database::Me2Plot(db)) => Msg::PlotDb(db),
                Response::Error(err) => Msg::Error(err),
                _ => unreachable!(),
            }));

        database_service.send(Request::Database(Type::Me2Plot));

        Me2Plot { props, link, _database_service: database_service, plot_db: None }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::PlotDb(db) => {
                self.plot_db = Some(db);
                true
            }
            Msg::Error(err) => {
                self.props.onerror.emit(err);
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        if let Some(ref plot_db) = self.plot_db {
            let (booleans, integers, me1_booleans, me1_integers) = (
                &self.props.booleans,
                &self.props.integers,
                &self.props.me1_booleans,
                &self.props.me1_integers,
            );
            let Me2PlotDb {
                player,
                crew,
                romance,
                missions,
                loyalty_missions,
                research_upgrades,
                rewards,
                captains_cabin,
                imported_me1,
            } = plot_db.as_ref();

            let view_categories = |categories: &IndexMap<String, PlotCategoryDb>| {
                categories
                    .iter()
                    .map(|(title, category)| {
                        html! {
                            <PlotCategory
                                title=title.to_owned()
                                booleans=RcUi::clone(booleans)
                                integers=IntegerPlotType::clone(integers)
                                category=category.clone()
                            />
                        }
                    })
                    .collect::<Vec<_>>()
            };

            let categories = [
                ("Crew", crew),
                ("Romance", romance),
                ("Missions", missions),
                ("Loyalty missions", loyalty_missions),
                ("Research / Upgrades", research_upgrades),
            ];

            let categories = categories.iter().map(|(tab, categories)| {
                // Workaround for unused_braces warning
                #[allow(unused_braces)]
                (html_nested! {
                    <Tab title=tab.to_owned()>
                        <div class="flex-auto flex flex-col gap-1">
                            { for view_categories(categories) }
                        </div>
                    </Tab>
                })
            });

            let mass_effect_1 = if !me1_booleans.borrow().is_empty() {
                html! {
                    <>
                        <div>
                            { "If you change these plots this will ONLY take effect after a new game +" }
                            <hr class="border-t border-default-border" />
                        </div>
                        <Me1Plot
                            booleans=RcUi::clone(me1_booleans)
                            integers=IntegerPlotType::clone(me1_integers)
                            onerror=self.link.callback(Msg::Error)
                        />
                    </>
                }
            } else {
                html! {
                    <div>
                        { "You cannot edit ME1 plot if you have not imported a ME1 save." }
                        <hr class="border-t border-default-border" />
                    </div>
                }
            };

            html! {
                <TabBar>
                    <Tab title="Player">
                        <PlotCategory
                            booleans=RcUi::clone(booleans)
                            integers=IntegerPlotType::clone(integers)
                            category=player.clone()
                        />
                    </Tab>
                    <Tab title="Rewards">
                        <PlotCategory
                            booleans=RcUi::clone(booleans)
                            integers=IntegerPlotType::clone(integers)
                            category=rewards.clone()
                        />
                    </Tab>
                    { for categories }
                    <Tab title="Captain's cabin">
                        <PlotCategory
                            booleans=RcUi::clone(booleans)
                            integers=IntegerPlotType::clone(integers)
                            category=captains_cabin.clone()
                        />
                    </Tab>
                    <Tab title="Imported ME1" theme=Theme::MassEffect1>
                        <div class="flex-auto flex flex-col gap-1">
                            <div>
                                { "For proper ME3 import change the same plot flags in `Mass Effect 1` tab. Conrad Verner paragon fix : //TODO: (?)" }
                                <hr class="border-t border-default-border" />
                            </div>
                            { for view_categories(imported_me1) }
                        </div>
                    </Tab>
                    <Tab title="Mass Effect 1" theme=Theme::MassEffect1>
                        <div class="flex-auto flex flex-col gap-1">
                            { mass_effect_1 }
                        </div>
                    </Tab>
                </TabBar>
            }
        } else {
            html! {
                <>
                    { "Loading database..." }
                    <hr class="border-t border-default-border" />
                </>
            }
        }
    }
}
