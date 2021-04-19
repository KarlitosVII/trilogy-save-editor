use imgui::*;

use crate::save_data::{
    common::plot::*,
    mass_effect_1::known_plot::*,
    mass_effect_2::{known_plot::*, plot::*, *},
    *,
};

use super::*;

impl<'a> Gui<'a> {
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
                save_game.draw_raw_ui(self, "Mass Effect 2").await;
            }
        }
    }

    pub async fn draw_me2_plot(
        &self, me2_plot_table: &mut PlotTable, me2_known_plot: &Me2KnownPlot,
    ) {
        let ui = self.ui;

        // Player
        if let Some(_t) = TabItem::new(im_str!("Player")).begin(ui) {
            self.draw_me2_known_plot(me2_plot_table, &me2_known_plot.player).await;
        }
        // Crew
        if let Some(_t) = TabItem::new(im_str!("Crew")).begin(ui) {
            for (category_name, known_plot) in &me2_known_plot.crew {
                if let Some(_t) = TreeNode::new(&ImString::new(category_name)).push(ui) {
                    self.draw_me2_known_plot(me2_plot_table, known_plot).await;
                }
            }
        }
        // Romance
        if let Some(_t) = TabItem::new(im_str!("Romance")).begin(ui) {
            for (category_name, known_plot) in &me2_known_plot.romance {
                if let Some(_t) = TreeNode::new(&ImString::new(category_name)).push(ui) {
                    self.draw_me2_known_plot(me2_plot_table, known_plot).await;
                }
            }
        }
        // Missions
        if let Some(_t) = TabItem::new(im_str!("Missions")).begin(ui) {
            for (category_name, known_plot) in &me2_known_plot.missions {
                if let Some(_t) = TreeNode::new(&ImString::new(category_name)).push(ui) {
                    self.draw_me2_known_plot(me2_plot_table, known_plot).await;
                }
            }
        }
        // Loyalty missions
        if let Some(_t) = TabItem::new(im_str!("Loyalty missions")).begin(ui) {
            for (category_name, known_plot) in &me2_known_plot.loyalty_missions {
                if let Some(_t) = TreeNode::new(&ImString::new(category_name)).push(ui) {
                    self.draw_me2_known_plot(me2_plot_table, known_plot).await;
                }
            }
        }
        // Research
        if let Some(_t) = TabItem::new(im_str!("Research / Upgrades")).begin(ui) {
            for (category_name, known_plot) in &me2_known_plot.research_upgrades {
                if let Some(_t) = TreeNode::new(&ImString::new(category_name)).push(ui) {
                    self.draw_me2_known_plot(me2_plot_table, known_plot).await;
                }
            }
        }
        // Rewards
        if let Some(_t) = TabItem::new(im_str!("Rewards")).begin(ui) {
            self.draw_me2_known_plot(me2_plot_table, &me2_known_plot.rewards).await;
        }
        // Captain's cabin
        if let Some(_t) = TabItem::new(im_str!("Captain's cabin")).begin(ui) {
            self.draw_me2_known_plot(me2_plot_table, &me2_known_plot.captains_cabin).await;
        }
    }

    async fn draw_me2_known_plot(&self, me2_plot_table: &mut PlotTable, known_plot: &KnownPlot) {
        // Booleans
        for (plot_id, plot_desc) in &known_plot.booleans {
            let plot = me2_plot_table.bool_variables.get_mut(*plot_id);
            if let Some(mut plot) = plot {
                plot.draw_raw_ui(self, plot_desc).await;
            }
        }
        // Integers
        for (plot_id, plot_desc) in &known_plot.ints {
            let plot = me2_plot_table.int_variables.get_mut(*plot_id);
            if let Some(plot) = plot {
                plot.draw_raw_ui(self, plot_desc).await;
            }
        }
    }
}
