use indexmap::IndexMap;
use yew::prelude::*;

use crate::{
    gui::{
        components::{Tab, TabBar},
        shared::{IntPlotType, PlotCategory},
    },
    save_data::{
        mass_effect_1::plot_db::Me1PlotDb,
        shared::plot::{BitVec, PlotCategory as PlotCategoryDb},
        RcRef,
    },
    services::database::Databases,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub booleans: RcRef<BitVec>,
    pub integers: IntPlotType,
    #[prop_or(false)]
    pub me3_imported_me1: bool,
}

#[function_component(Me1Plot)]
pub fn me1_plot(props: &Props) -> Html {
    let dbs = use_context::<Databases>().expect("no database provider");
    if let Some(plot_db) = dbs.get_me1_plot() {
        let Props { booleans, integers, .. } = props;
        let Me1PlotDb { player_crew, missions } = &*plot_db;

        let view_categories = |categories: &IndexMap<String, PlotCategoryDb>| {
            categories
                .iter()
                .map(|(title, category)| {
                    html! {
                        <PlotCategory
                            title={title.clone()}
                            booleans={RcRef::clone(booleans)}
                            integers={IntPlotType::clone(integers)}
                            category={category.clone()}
                            me3_imported_me1={props.me3_imported_me1}
                        />
                    }
                })
                .collect::<Vec<_>>()
        };

        let categories = [("Player / Crew", player_crew), ("Missions", missions)];

        let categories = categories.iter().map(|(tab, categories)| {
            html_nested! {
                <Tab title={tab.to_owned()}>
                    <div class="flex-auto flex flex-col gap-1">
                        { for view_categories(categories) }
                    </div>
                </Tab>
            }
        });

        html! {
            <TabBar>
                { for categories }
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
