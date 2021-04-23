use if_chain::if_chain;
use imgui::*;

use crate::save_data::{
    common::plot::PlotCategory,
    mass_effect_2::{plot::PlotTable, Me2SaveGame},
};

use super::*;

impl<'ui> Gui<'ui> {
    pub async fn draw_mass_effect_2(
        &self, save_game: &mut Me2SaveGame, known_plots: &KnownPlotsState,
    ) {
        let ui = self.ui;

        // Tab bar
        let _t = match TabBar::new(im_str!("mass_effect_2")).begin(ui) {
            Some(t) => t,
            None => return,
        };

        // Plot
        if_chain! {
            if let Some(_t) = TabItem::new(im_str!("Plot")).begin(ui);
            if let Some(_t) = TabBar::new(im_str!("plot-tab")).begin(ui);
            then {
                // Mass Effect 2
                self.draw_me2_known_plot(&mut save_game.plot, &known_plots).await;
                // Mass Effect 1
                {
                    let _colors = self.style_colors(Theme::MassEffect1).await;
                    if_chain! {
                        if let Some(_t) = TabItem::new(im_str!("Mass Effect 1")).begin(ui);
                        if let Some(me1_known_plot) = &known_plots.me1;
                        then {
                            self.draw_me1_known_plot(&mut save_game.me1_plot, &me1_known_plot).await;
                        }
                    }
                }
            }
        }
        // Raw
        if_chain! {
            if let Some(_t) = TabItem::new(im_str!("Raw")).begin(ui);
            if let Some(_t) = ChildWindow::new(im_str!("scroll")).begin(ui);
            then {
                self.set_next_item_open(true);
                save_game.draw_raw_ui(self, "Mass Effect 2").await;
            }
        }
    }

    async fn draw_me2_known_plot(
        &self, me2_plot_table: &mut PlotTable, known_plots: &KnownPlotsState,
    ) {
        let ui = self.ui;
        let me2_known_plot = match &known_plots.me2 {
            Some(me2) => me2,
            None => return,
        };

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
        if_chain! {
            if let Some(_t) = TabItem::new(im_str!("Player")).begin(ui);
            if let Some(_t) = ChildWindow::new(im_str!("scroll")).begin(ui);
            if let Some(_t) = self.begin_table(im_str!("plot-table"), 1);
            then {
                self.draw_me2_plot_category(me2_plot_table, player).await;
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
            if_chain! {
                if let Some(_t) = TabItem::new(title).begin(ui);
                if let Some(_t) = ChildWindow::new(im_str!("scroll")).begin(ui);
                then {
                    for (category_name, known_plot) in plot_map.iter() {
                        if let Some(_t) = self.begin_table(&im_str!("{}-table", category_name), 1) {
                            self.table_next_row();
                            if let Some(_t) = self.push_tree_node(category_name) {
                                self.draw_me2_plot_category(me2_plot_table, known_plot).await;
                            }
                        }
                    }
                }
            }
        }

        // Rewards
        if_chain! {
            if let Some(_t) = TabItem::new(im_str!("Rewards")).begin(ui);
            if let Some(_t) = ChildWindow::new(im_str!("scroll")).begin(ui);
            if let Some(_t) = self.begin_table(im_str!("plot-table"), 1);
            then {
                self.draw_me2_plot_category(me2_plot_table, rewards).await;
            }
        }
        // Captain's cabin
        if_chain! {
            if let Some(_t) = TabItem::new(im_str!("Captain's cabin")).begin(ui);
            if let Some(_t) = ChildWindow::new(im_str!("scroll")).begin(ui);
            if let Some(_t) = self.begin_table(im_str!("plot-table"), 1);
            then {
                self.draw_me2_plot_category(me2_plot_table, captains_cabin).await;
            }
        }
    }

    async fn draw_me2_plot_category(&self, plot_table: &mut PlotTable, known_plot: &PlotCategory) {
        let PlotCategory { booleans, ints } = known_plot;

        if booleans.is_empty() && ints.is_empty() {
            return;
        }

        // Booleans
        for (plot_id, plot_desc) in booleans {
            let plot = plot_table.bool_variables.get_mut(*plot_id);
            if let Some(mut plot) = plot {
                self.table_next_row();
                plot.draw_raw_ui(self, plot_desc).await;
            }
        }
        // Integers
        for (plot_id, plot_desc) in ints {
            let plot = plot_table.int_variables.get_mut(*plot_id);
            if let Some(plot) = plot {
                self.table_next_row();
                plot.draw_raw_ui(self, plot_desc).await;
            }
        }
    }
}
