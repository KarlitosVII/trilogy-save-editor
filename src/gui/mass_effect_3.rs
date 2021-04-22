use imgui::*;

use crate::save_data::{
    common::plot::*,
    mass_effect_1::known_plot::*,
    mass_effect_2::known_plot::*,
    mass_effect_3::{known_plot::*, plot::*, *},
    *,
};

use super::*;

impl<'ui> Gui<'ui> {
    pub async fn draw_mass_effect_3(
        &self, save_game: &mut Me3SaveGame, known_plots: &KnownPlotsState,
    ) {
        let ui = self.ui;

        // Tabs
        if let Some(_t) = TabBar::new(im_str!("mass_effect_3")).begin(ui) {
            // Plot
            if let Some(_t) = TabItem::new(im_str!("Plot")).begin(ui) {
                if let Some(_t) = TabBar::new(im_str!("plot-tab")).begin(ui) {
                    self.draw_me3_known_plot(
                        &mut save_game.plot,
                        &mut save_game.player_variables,
                        known_plots,
                    )
                    .await;
                }
            }
            // Raw
            if let Some(_t) = TabItem::new(im_str!("Raw")).begin(ui) {
                self.set_next_item_open(true);
                save_game.draw_raw_ui(self, "Mass Effect 3").await;
            }
        }
    }

    pub async fn draw_me3_known_plot(
        &self, plot_table: &mut PlotTable, player_variables: &mut IndexMap<ImString, i32>,
        known_plots: &KnownPlotsState,
    ) {
        let ui = self.ui;
        if let Some(me3_known_plot) = &known_plots.me3 {
            let Me3KnownPlot {
                general,
                appearances,
                crew,
                romance,
                missions,
                citadel_dlc,
                intel,
                normandy,
                weapons_powers,
                me1_imported,
            } = me3_known_plot;

            // Mass Effect 3
            if let Some(_t) = TabItem::new(&im_str!("General")).begin(ui) {
                if let Some(_t) = self.begin_table(&im_str!("plot-table"), 1) {
                    self.draw_me3_plot_category(plot_table, general).await;
                }
            }

            let categories = [
                (im_str!("Appearances"), appearances),
                (im_str!("Crew"), crew),
                (im_str!("Romance"), romance),
                (im_str!("Missions"), missions),
                (im_str!("Normandy"), normandy),
                (im_str!("Citadel DLC"), citadel_dlc),
            ];

            for (title, plot_map) in &categories {
                if let Some(_t) = TabItem::new(title).begin(ui) {
                    if let Some(_t) = self.begin_table(&im_str!("plot-table"), 1) {
                        for (category_name, known_plot) in plot_map.iter() {
                            self.table_next_row();
                            self.table_next_column();
                            if let Some(_t) = self.push_tree_node(category_name) {
                                self.draw_me3_plot_category(plot_table, known_plot).await;
                            }
                        }
                    }
                }
            }

            if let Some(_t) = TabItem::new(&im_str!("Intel")).begin(ui) {
                if let Some(_t) = self.begin_table(&im_str!("plot-table"), 1) {
                    self.draw_me3_plot_category(plot_table, intel).await;
                }
            }

            // Weapons / Powers
            if let Some(_t) = TabItem::new(&im_str!("Weapons / Powers")).begin(ui) {
                if let Some(_t) = self.begin_table(&im_str!("plot-table"), 1) {
                    for (category_name, known_plot) in weapons_powers {
                        self.table_next_row();
                        self.table_next_column();
                        if let Some(_t) = self.push_tree_node(category_name) {
                            self.draw_me3_plot_variable(plot_table, player_variables, known_plot)
                                .await;
                        }
                    }
                }
            }

            // Mass Effect 2
            if let Some(me2_known_plot) = &known_plots.me2 {
                let _colors = self.style_colors(Theme::MassEffect2).await;
                if let Some(_t) = TabItem::new(&im_str!("Mass Effect 2")).begin(ui) {
                    self.draw_me2_imported_known_plot(plot_table, me2_known_plot).await;
                }
            }
            // Mass Effect 1
            {
                let _colors = self.style_colors(Theme::MassEffect1).await;
                if let Some(_t) = TabItem::new(&im_str!("Mass Effect 1")).begin(ui) {
                    self.draw_me1_imported_known_plot(plot_table, me1_imported).await;
                }
            }
        }
    }

    pub async fn draw_me2_imported_known_plot(
        &self, me3_plot_table: &mut PlotTable, me2_known_plot: &Me2KnownPlot,
    ) {
        let ui = self.ui;
        let Me2KnownPlot {
            player,
            crew,
            romance,
            missions,
            loyalty_missions,
            research_upgrades,
            rewards,
            captains_cabin,
        } = me2_known_plot;

        if let Some(_t) = TabBar::new(im_str!("plot-tab")).begin(ui) {
            // Player
            if let Some(_t) = TabItem::new(im_str!("Player")).begin(ui) {
                if let Some(_t) = self.begin_table(&im_str!("plot-table"), 1) {
                    self.draw_me3_plot_category(me3_plot_table, player).await;
                }
            }

            let categories = [
                (im_str!("Crew"), crew),
                (im_str!("Romance"), romance),
                (im_str!("Missions"), missions),
                (im_str!("Loyalty missions"), loyalty_missions),
                (im_str!("Research / Upgrades"), research_upgrades),
            ];

            for (title, plot_map) in &categories {
                if let Some(_t) = TabItem::new(title).begin(ui) {
                    if let Some(_t) = self.begin_table(&im_str!("plot-table"), 1) {
                        for (category_name, known_plot) in plot_map.iter() {
                            self.table_next_row();
                            self.table_next_column();
                            if let Some(_t) = self.push_tree_node(category_name) {
                                self.draw_me3_plot_category(me3_plot_table, known_plot).await;
                            }
                        }
                    }
                }
            }

            // Rewards
            if let Some(_t) = TabItem::new(im_str!("Rewards")).begin(ui) {
                if let Some(_t) = self.begin_table(&im_str!("plot-table"), 1) {
                    self.draw_me3_plot_category(me3_plot_table, rewards).await;
                }
            }
            // Captain's cabin
            if let Some(_t) = TabItem::new(im_str!("Captain's cabin")).begin(ui) {
                if let Some(_t) = self.begin_table(&im_str!("plot-table"), 1) {
                    self.draw_me3_plot_category(me3_plot_table, captains_cabin).await;
                }
            }
        }
    }

    pub async fn draw_me1_imported_known_plot(
        &self, me1_plot_table: &mut PlotTable, me1_imported: &Me1KnownPlot,
    ) {
        let ui = self.ui;
        let Me1KnownPlot { player_crew, missions } = me1_imported;

        if let Some(_t) = TabBar::new(im_str!("plot-tab")).begin(ui) {
            let categories =
                [(im_str!("Player / Crew"), player_crew), (im_str!("Missions"), missions)];

            for (title, plot_map) in &categories {
                if let Some(_t) = TabItem::new(title).begin(ui) {
                    if let Some(_t) = self.begin_table(&im_str!("plot-table"), 1) {
                        for (category_name, known_plot) in plot_map.iter() {
                            self.table_next_row();
                            self.table_next_column();
                            if let Some(_t) = self.push_tree_node(category_name) {
                                self.draw_me3_plot_category(me1_plot_table, known_plot).await;
                            }
                        }
                    }
                }
            }
        }
    }

    async fn draw_me3_plot_category(&self, plot_table: &mut PlotTable, known_plot: &PlotCategory) {
        let PlotCategory { booleans, ints } = known_plot;

        if booleans.is_empty() && ints.is_empty() {
            return;
        }

        // Booleans
        for (plot_id, plot_desc) in booleans {
            let plot = plot_table.bool_variables.get_mut(*plot_id);
            if let Some(mut plot) = plot {
                self.table_next_row();
                self.table_next_column();
                plot.draw_raw_ui(self, plot_desc).await;
            }
        }
        // Integers
        for (plot_id, plot_desc) in ints {
            let plot = plot_table.int_variables.entry(*plot_id as i32).or_default();

            self.table_next_row();
            self.table_next_column();
            plot.draw_raw_ui(self, plot_desc).await;
        }
    }

    async fn draw_me3_plot_variable(
        &self, plot_table: &mut PlotTable, player_variables: &mut IndexMap<ImString, i32>,
        known_plot: &PlotVariable,
    ) {
        let PlotVariable { booleans, variables } = known_plot;

        if booleans.is_empty() && player_variables.is_empty() {
            return;
        }

        // Booleans
        for (plot_id, plot_desc) in booleans {
            let plot = plot_table.bool_variables.get_mut(*plot_id);
            if let Some(mut plot) = plot {
                self.table_next_row();
                self.table_next_column();
                plot.draw_raw_ui(self, plot_desc).await;
            }
        }
        // Variables
        for (variable_id, variable_desc) in variables {
            let variable =
                player_variables.iter_mut().find(|(key, _)| unicase::eq(key.to_str(), variable_id));

            let value = match variable {
                Some((_, value)) => value,
                None => player_variables.entry(ImString::new(variable_id)).or_default(),
            };

            self.table_next_row();
            self.table_next_column();
            value.draw_raw_ui(self, variable_desc).await;
        }
    }
}
