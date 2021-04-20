use imgui::*;

use crate::save_data::{
    common::plot::*,
    mass_effect_1::known_plot::*,
    mass_effect_2::{known_plot::*, plot::*, *},
    *,
};

use super::*;

impl<'ui> Gui<'ui> {
    pub async fn draw_mass_effect_2(&self, save_game: &mut Me2SaveGame) {
        let ui = self.ui;

        // TODO: Change ça
        let string = include_str!("../../plot/Me1KnownPlot.ron");
        let me1_known_plot: Me1KnownPlot = ron::from_str(&string).unwrap();

        let string = include_str!("../../plot/Me2KnownPlot.ron");
        let me2_known_plot: Me2KnownPlot = ron::from_str(&string).unwrap();

        // Tabs
        if let Some(_t) = TabBar::new(im_str!("mass_effect_2")).begin(ui) {
            // Plot
            if let Some(_t) = TabItem::new(im_str!("Plot")).begin(ui) {
                let me2_plot_table = &mut save_game.plot;
                let me1_plot_table = &mut save_game.me1_plot;
                if let Some(_t) = TabBar::new(im_str!("plot-tab")).begin(ui) {
                    self.draw_me2_plot(me2_plot_table, &me2_known_plot).await;
                    {
                        let _colors = self.style_colors(Theme::MassEffect1).await;
                        if let Some(_t) = TabItem::new(im_str!("Mass Effect 1")).begin(ui) {
                            self.draw_me1_plot(me1_plot_table, &me1_known_plot).await;
                        }
                    }
                }
            }
            // Raw
            if let Some(_t) = TabItem::new(im_str!("Raw")).begin(ui) {
                self.set_next_item_open(true);
                save_game.draw_raw_ui(self, "Mass Effect 2").await;
            }
        }
    }

    pub async fn draw_me2_plot(
        &self, me2_plot_table: &mut PlotTable, me2_known_plot: &Me2KnownPlot,
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

        // Player
        if let Some(_t) = TabItem::new(im_str!("Player")).begin(ui) {
            if let Some(_t) = self.begin_table(&im_str!("plot-table"), 1) {
                self.draw_me2_known_plot(me2_plot_table, player).await;
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
                            self.draw_me2_known_plot(me2_plot_table, known_plot).await;
                        }
                    }
                }
            }
        }

        // Rewards
        if let Some(_t) = TabItem::new(im_str!("Rewards")).begin(ui) {
            if let Some(_t) = self.begin_table(&im_str!("plot-table"), 1) {
                self.draw_me2_known_plot(me2_plot_table, rewards).await;
            }
        }
        // Captain's cabin
        if let Some(_t) = TabItem::new(im_str!("Captain's cabin")).begin(ui) {
            if let Some(_t) = self.begin_table(&im_str!("plot-table"), 1) {
                self.draw_me2_known_plot(me2_plot_table, captains_cabin).await;
            }
        }
    }

    async fn draw_me2_known_plot(&self, me2_plot_table: &mut PlotTable, known_plot: &KnownPlot) {
        let KnownPlot { booleans, ints } = known_plot;

        if booleans.is_empty() && ints.is_empty() {
            return;
        }

        // Booleans
        for (plot_id, plot_desc) in &known_plot.booleans {
            let plot = me2_plot_table.bool_variables.get_mut(*plot_id);
            if let Some(mut plot) = plot {
                self.table_next_row();
                self.table_next_column();
                plot.draw_raw_ui(self, plot_desc).await;
            }
        }
        // Integers
        for (plot_id, plot_desc) in &known_plot.ints {
            let plot = me2_plot_table.int_variables.get_mut(*plot_id);
            if let Some(plot) = plot {
                self.table_next_row();
                self.table_next_column();
                plot.draw_raw_ui(self, plot_desc).await;
            }
        }
    }
}
