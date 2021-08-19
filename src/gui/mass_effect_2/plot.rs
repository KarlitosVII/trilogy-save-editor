use std::rc::Rc;

use anyhow::Error;
use indexmap::IndexMap;
use yew::{prelude::*, utils::NeqAssign};
use yew_agent::{Bridge, Bridged};

use crate::gui::{
    components::{Helper, Tab, TabBar},
    mass_effect_1::Me1Plot,
    shared::{IntPlotType, PlotCategory},
    RcUi, Theme,
};
use crate::save_data::{
    mass_effect_2::plot_db::Me2PlotDb,
    shared::plot::{BitVec, PlotCategory as PlotCategoryDb},
};
use crate::services::database::{Database, DatabaseService, Request, Response, Type};

pub enum Msg {
    PlotDb(Rc<Me2PlotDb>),
    Error(Error),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub booleans: RcUi<BitVec>,
    pub integers: IntPlotType,
    pub me1_booleans: Option<RcUi<BitVec>>,
    pub me1_integers: Option<IntPlotType>,
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
                                title={title.clone()}
                                booleans={RcUi::clone(booleans)}
                                integers={IntPlotType::clone(integers)}
                                category={category.clone()}
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
                    <Tab title={tab.to_owned()}>
                        <div class="flex-auto flex flex-col gap-1">
                            { for view_categories(categories) }
                        </div>
                    </Tab>
                })
            });

            let mass_effect_1 = me1_booleans.as_ref().map(|me1_booleans| {
                if !me1_booleans.borrow().is_empty() {
                    let me1_integers = me1_integers.as_ref().unwrap();
                    // Workaround for unused_braces warning
                    #[allow(unused_braces)]
                    (html_nested! {
                            <Tab title="Mass Effect 1" theme={Theme::MassEffect1}>
                                <div class="flex-auto flex flex-col gap-1">
                                    <div>
                                        { "If you change these plots this will ONLY take effect after a new game +." }
                                        <hr class="border-t border-default-border" />
                                    </div>
                                    <Me1Plot
                                        booleans={RcUi::clone(me1_booleans)}
                                        integers={IntPlotType::clone(me1_integers)}
                                        onerror={self.link.callback(Msg::Error)}
                                    />
                                </div>
                            </Tab>
                        })
                } else {
                    // Workaround for unused_braces warning
                    #[allow(unused_braces)]
                    (html_nested! {
                        <Tab title="Mass Effect 1" theme={Theme::MassEffect1}>
                            { "You cannot edit ME1 plot if you have not imported a ME1 save." }
                            <hr class="border-t border-default-border" />
                        </Tab>
                    })
                }
            }).into_iter();

            html! {
                <TabBar>
                    <Tab title="Player">
                        <PlotCategory
                            booleans={RcUi::clone(booleans)}
                            integers={IntPlotType::clone(integers)}
                            category={player.clone()}
                        />
                    </Tab>
                    { for categories }
                    <Tab title="Captain's cabin">
                        <PlotCategory
                            booleans={RcUi::clone(booleans)}
                            integers={IntPlotType::clone(integers)}
                            category={captains_cabin.clone()}
                        />
                    </Tab>
                    <Tab title="Rewards">
                        <PlotCategory
                            booleans={RcUi::clone(booleans)}
                            integers={IntPlotType::clone(integers)}
                            category={rewards.clone()}
                        />
                    </Tab>
                    <Tab title="Imported ME1" theme={Theme::MassEffect1}>
                        <div class="flex-auto flex flex-col gap-1">
                            <div>
                                <div class="flex items-center gap-1">
                                    { "For proper ME3 import change the same plot flags in `Mass Effect 1` tab. Conrad Verner paragon fix :" }
                                    <Helper text=
                                        "• Untick `[The Fan] Intimidated him`\n\
                                        • Tick `[The Fan] Met Conrad Verner` and `[The Fan] Charmed him`\n\
                                        • Only works if you didn't talk to Aethyta"
                                    />
                                </div>
                                <hr class="border-t border-default-border" />
                            </div>
                            { for view_categories(imported_me1) }
                        </div>
                    </Tab>
                    { for mass_effect_1 }
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
