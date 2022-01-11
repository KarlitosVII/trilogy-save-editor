use indexmap::IndexMap;
use yew::prelude::*;

use crate::{
    gui::{
        components::{Tab, TabBar},
        mass_effect_1::Me1Plot,
        mass_effect_2::Me2Plot,
        mass_effect_3::PlotVariable,
        shared::{IntPlotType, PlotCategory},
        Theme,
    },
    save_data::{
        mass_effect_3::plot_db::Me3PlotDb,
        shared::plot::{BitVec, PlotCategory as PlotCategoryDb},
        RcCell, RcRef,
    },
    services::database::Databases,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub booleans: RcRef<BitVec>,
    pub integers: IntPlotType,
    pub variables: RcRef<IndexMap<String, RcCell<i32>>>,
}

#[function_component(Me3Plot)]
pub fn me3_plot(props: &Props) -> Html {
    let dbs = use_context::<Databases>().expect("no database provider");
    if let Some(plot_db) = dbs.get_me3_plot() {
        let Props { booleans, integers, variables, .. } = props;
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
        } = &*plot_db;

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
                    booleans={RcRef::clone(booleans)}
                    variables={RcRef::clone(variables)}
                    plot_variable={variable.clone()}
                />
            }
        });

        html! {
            <TabBar>
                <Tab title="General">
                    <PlotCategory
                        booleans={RcRef::clone(booleans)}
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
                        booleans={RcRef::clone(booleans)}
                        integers={IntPlotType::clone(integers)}
                        category={intel.clone()}
                    />
                </Tab>
                <Tab title="Mass Effect 2" theme={Theme::MassEffect2}>
                    <Me2Plot
                        booleans={RcRef::clone(booleans)}
                        integers={IntPlotType::clone(integers)}
                    />
                </Tab>
                <Tab title="Mass Effect 1" theme={Theme::MassEffect1}>
                    <Me1Plot
                        me3_imported_me1={true}
                        booleans={RcRef::clone(booleans)}
                        integers={IntPlotType::clone(integers)}
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
