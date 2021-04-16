use imgui::*;

use crate::save_data::mass_effect_1::Me1SaveGame;

use super::Gui;

impl<'a> Gui<'a> {
    pub async fn draw_mass_effect_1(&self, _save_game: &mut Me1SaveGame) {
        let ui = self.ui;

        // Tabs
        if let Some(_t) = TabBar::new(im_str!("me1-tabs")).begin(ui) {
            if let Some(_t) = TabItem::new(im_str!("Raw")).begin(ui) {
                if let Some(_t) = ChildWindow::new("mass_effect_1").size([0.0, 0.0]).begin(ui) {
                    // save_game.draw_raw_ui(self, "Mass Effect 1").await;
                }
            }
        }
    }
}
