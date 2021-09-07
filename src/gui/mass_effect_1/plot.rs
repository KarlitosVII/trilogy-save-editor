use std::rc::Rc;

use anyhow::Error;
use indexmap::IndexMap;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::gui::{
    components::{Tab, TabBar},
    shared::{IntPlotType, PlotCategory},
    RcUi,
};
use crate::save_data::{
    mass_effect_1::plot_db::Me1PlotDb,
    shared::plot::{BitVec, PlotCategory as PlotCategoryDb},
};
use crate::services::database::{Database, DatabaseService, Request, Response, Type};

pub enum Msg {
    PlotDb(Rc<Me1PlotDb>),
    Error(Error),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub booleans: RcUi<BitVec>,
    pub integers: IntPlotType,
    #[prop_or(false)]
    pub me3_imported_me1: bool,
    pub onerror: Callback<Error>,
}

pub struct Me1Plot {
    _database_service: Box<dyn Bridge<DatabaseService>>,
    plot_db: Option<Rc<Me1PlotDb>>,
}

impl Component for Me1Plot {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let mut database_service =
            DatabaseService::bridge(ctx.link().callback(|response| match response {
                Response::Database(Database::Me1Plot(db)) => Msg::PlotDb(db),
                Response::Error(err) => Msg::Error(err),
                _ => unreachable!(),
            }));

        database_service.send(Request::Database(Type::Me1Plot));

        Me1Plot { _database_service: database_service, plot_db: None }
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
            let Props { booleans, integers, .. } = &ctx.props();
            let Me1PlotDb { player_crew, missions } = plot_db.as_ref();

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
                                me3_imported_me1={ctx.props().me3_imported_me1}
                            />
                        }
                    })
                    .collect::<Vec<_>>()
            };

            let categories = [("Player / Crew", player_crew), ("Missions", missions)];

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

            html! {
                <TabBar>
                    { for categories }
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
