use indexmap::IndexMap;
use yew::prelude::*;

use crate::{
    gui::{
        components::{Helper, Tab, TabBar},
        format_code,
        mass_effect_1::Me1Plot,
        shared::{IntPlotType, PlotCategory},
        RcUi, Theme,
    },
    save_data::{
        mass_effect_2::plot_db::Me2PlotDb,
        shared::plot::{BitVec, PlotCategory as PlotCategoryDb},
    },
    services::database::Databases,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub booleans: RcUi<BitVec>,
    pub integers: IntPlotType,
    pub me1_booleans: Option<RcUi<BitVec>>,
    pub me1_integers: Option<IntPlotType>,
}

#[function_component(Me2Plot)]
pub fn me2_plot(props: &Props) -> Html {
    let dbs = use_context::<Databases>().expect("no database provider");
    if let Some(plot_db) = dbs.get_me2_plot() {
        let Props { booleans, integers, me1_booleans, me1_integers, .. } = props;
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
        } = &*plot_db;

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
            html_nested! {
                <Tab title={tab.to_owned()}>
                    <div class="flex-auto flex flex-col gap-1">
                        { for view_categories(categories) }
                    </div>
                </Tab>
            }
        });

        let mass_effect_1 = me1_booleans.as_ref().map(|me1_booleans| {
                if !me1_booleans.borrow().is_empty() {
                    let me1_integers = me1_integers.as_ref().unwrap();
                    html_nested! {
                        <Tab title="Mass Effect 1" theme={Theme::MassEffect1}>
                            <div class="flex-auto flex flex-col gap-1">
                                <div>
                                    <p>{ "If you change these plots this will ONLY take effect after a new game +." }</p>
                                    <hr class="border-t border-default-border" />
                                </div>
                                <Me1Plot
                                    booleans={RcUi::clone(me1_booleans)}
                                    integers={IntPlotType::clone(me1_integers)}
                                />
                            </div>
                        </Tab>
                    }
                } else {
                    html_nested! {
                        <Tab title="Mass Effect 1" theme={Theme::MassEffect1}>
                            <p>{ "You cannot edit ME1 plot if you have not imported a ME1 save." }</p>
                            <hr class="border-t border-default-border" />
                        </Tab>
                    }
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
                                <p>{ format_code("For proper ME3 import change the same plot flags in `Mass Effect 1` tab. Conrad Verner paragon fix :") }</p>
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
                <p>{ "Loading database..." }</p>
                <hr class="border-t border-default-border" />
            </>
        }
    }
}
