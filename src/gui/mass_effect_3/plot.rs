use std::rc::Rc;

use anyhow::Error;
use indexmap::IndexMap;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::gui::{
    components::{Tab, TabBar},
    mass_effect_1::Me1Plot,
    mass_effect_2::Me2Plot,
    mass_effect_3::PlotVariable,
    shared::{IntPlotType, PlotCategory},
    RcUi, Theme,
};
use crate::save_data::{
    mass_effect_3::plot_db::Me3PlotDb,
    shared::plot::{BitVec, PlotCategory as PlotCategoryDb},
};
use crate::services::database::{Database, DatabaseService, Request, Response, Type};

pub enum Msg {
    PlotDb(Rc<Me3PlotDb>),
    Error(Error),
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub booleans: RcUi<BitVec>,
    pub integers: IntPlotType,
    pub variables: RcUi<IndexMap<String, RcUi<i32>>>,
    pub onerror: Callback<Error>,
}

pub struct Me3Plot {
    _database_service: Box<dyn Bridge<DatabaseService>>,
    plot_db: Option<Rc<Me3PlotDb>>,
}

impl Component for Me3Plot {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let mut database_service =
            DatabaseService::bridge(ctx.link().callback(|response| match response {
                Response::Database(Database::Me3Plot(db)) => Msg::PlotDb(db),
                Response::Error(err) => Msg::Error(err),
                _ => unreachable!(),
            }));

        database_service.send(Request::Database(Type::Me3Plot));

        Me3Plot { _database_service: database_service, plot_db: None }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::PlotDb(db) => {
                self.plot_db = Some(db);
                true
            }
            Msg::Error(err) => {
                ctx.props().onerror.emit(err);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if let Some(ref plot_db) = self.plot_db {
            let Props { booleans, integers, variables, .. } = &ctx.props();
            let Me3PlotDb {
                general,
                crew,
                romance,
                missions,
                citadel_dlc,
                normandy,
                appearances,
                weapons_powers,
                intel,
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
                ("Normandy", normandy),
                ("Citadel DLC", citadel_dlc),
                ("Appearances", appearances),
            ];

            let categories = categories.iter().map(|(tab, categories)| {
                html_nested! {
                    <Tab title={tab.to_owned()}>
                        <div class="flex-auto flex flex-col gap-1">
                            { for view_categories(categories) }
                        </div>
                    </Tab>
                }
            });

            let weapons_powers = weapons_powers.iter().map(|(title, variable)| {
                html! {
                    <PlotVariable
                        title={title.clone()}
                        booleans={RcUi::clone(booleans)}
                        variables={RcUi::clone(variables)}
                        plot_variable={variable.clone()}
                    />
                }
            });

            html! {
                <TabBar>
                    <Tab title="General">
                        <PlotCategory
                            booleans={RcUi::clone(booleans)}
                            integers={IntPlotType::clone(integers)}
                            category={general.clone()}
                        />
                    </Tab>
                    { for categories }
                    <Tab title="Weapons / Powers">
                        <div class="flex-auto flex flex-col gap-1">
                            { for weapons_powers }
                        </div>
                    </Tab>
                    <Tab title="Intel">
                        <PlotCategory
                            booleans={RcUi::clone(booleans)}
                            integers={IntPlotType::clone(integers)}
                            category={intel.clone()}
                        />
                    </Tab>
                    <Tab title="Mass Effect 2" theme={Theme::MassEffect2}>
                        <Me2Plot
                            booleans={RcUi::clone(booleans)}
                            integers={IntPlotType::clone(integers)}
                            onerror={ctx.link().callback(Msg::Error)}
                        />
                    </Tab>
                    <Tab title="Mass Effect 1" theme={Theme::MassEffect1}>
                        <Me1Plot
                            me3_imported_me1={true}
                            booleans={RcUi::clone(booleans)}
                            integers={IntPlotType::clone(integers)}
                            onerror={ctx.link().callback(Msg::Error)}
                        />
                    </Tab>
                </TabBar>
            }
        } else {
            html! {
                <>
                    <p>{ "Loading database..." }</p>
                    <hr class="border-t border-default-border" />
                </>
            }
        }
    }
}
