use anyhow::Error;
use indexmap::IndexMap;
use std::rc::Rc;
use yew::prelude::*;
use yewtil::NeqAssign;

use crate::{
    database_service::{Database, DatabaseService, Request, Response, Type},
    gui::{
        components::{
            shared::{IntPlotType, PlotCategory},
            Tab, TabBar,
        },
        RcUi,
    },
    save_data::{
        mass_effect_1::plot_db::Me1PlotDb,
        shared::plot::{BitVec, PlotCategory as PlotCategoryDb},
    },
};

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
    props: Props,
    _link: ComponentLink<Self>,
    _database_service: Box<dyn Bridge<DatabaseService>>,
    plot_db: Option<Rc<Me1PlotDb>>,
}

impl Component for Me1Plot {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut database_service =
            DatabaseService::bridge(link.callback(|response| match response {
                Response::Database(Database::Me1Plot(db)) => Msg::PlotDb(db),
                Response::Error(err) => Msg::Error(err),
                _ => unreachable!(),
            }));

        database_service.send(Request::Database(Type::Me1Plot));

        Me1Plot { props, _link: link, _database_service: database_service, plot_db: None }
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
            let (booleans, integers) = (&self.props.booleans, &self.props.integers);
            let Me1PlotDb { player_crew, missions } = plot_db.as_ref();

            let view_categories = |categories: &IndexMap<String, PlotCategoryDb>| {
                categories
                    .iter()
                    .map(|(title, category)| {
                        html! {
                            <PlotCategory
                                title=title.to_owned()
                                booleans=RcUi::clone(booleans)
                                integers=IntPlotType::clone(integers)
                                category=category.clone()
                                me3_imported_me1=self.props.me3_imported_me1
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
                    <Tab title=tab.to_owned()>
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
