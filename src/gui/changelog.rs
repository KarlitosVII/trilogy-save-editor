use imgui::{im_str, ChildWindow};

use super::Gui;

impl<'ui> Gui<'ui> {
    pub fn draw_change_log(&self) -> Option<()> {
        let ui = self.ui;

        let _t = ChildWindow::new("scroll").begin(ui)?;

        ui.text("Changelog");
        ui.separator();
        // 1.6.0
        if let Some(_t) = self.begin_table(im_str!("changelog-table"), 1) {
            self.table_next_row();
            self.set_next_item_open(true);
            if let Some(_t) = self.push_tree_node(env!("CARGO_PKG_VERSION")) {
                self.table_next_row();
                ui.text("Added ME1LE Level and Current XP");
                self.table_next_row();
                ui.text("Added ME1LE Raw talents");
                self.table_next_row();
                ui.text("Converted raw texts to title case for better readability");
            }
        }
        // 1.5.0
        if let Some(_t) = self.begin_table(im_str!("changelog-table"), 1) {
            self.table_next_row();
            if let Some(_t) = self.push_tree_node("1.5.0") {
                self.table_next_row();
                ui.text("Added ME1LE `General` tab with basic informations such as Name, Gender, Origin, Notoriety and Morality");
                self.table_next_row();
                ui.text("Added ME1LE `Head Morph` tab with Import / Export and raw data");
            }
        }
        // 1.4.0
        if let Some(_t) = self.begin_table(im_str!("changelog-table"), 1) {
            self.table_next_row();
            if let Some(_t) = self.push_tree_node("1.4.0") {
                self.table_next_row();
                ui.text("Added `Imported ME1` plots in ME2 saves");
            }
        }
        // 1.3.2
        if let Some(_t) = self.begin_table(im_str!("changelog-table"), 1) {
            self.table_next_row();
            if let Some(_t) = self.push_tree_node("1.3.2") {
                self.table_next_row();
                ui.text("New UNC plots in ME1 (thanks to Yggge)");
                self.table_next_row();
                ui.text("Added clarification for editing ME1 plot in ME2 save");
            }
        }
        // 1.3.1
        if let Some(_t) = self.begin_table(im_str!("changelog-table"), 1) {
            self.table_next_row();
            if let Some(_t) = self.push_tree_node("1.3.1") {
                self.table_next_row();
                ui.text("Fix ME1LE `unexpected end of file...` error for some people");
            }
        }
        // 1.3.0
        if let Some(_t) = self.begin_table(im_str!("changelog-table"), 1) {
            self.table_next_row();
            if let Some(_t) = self.push_tree_node("1.3.0") {
                self.table_next_row();
                ui.text("Initial Mass Effect 1 Legendary support (only plot)");
            }
        }
        // 1.2.0
        if let Some(_t) = self.begin_table(im_str!("changelog-table"), 1) {
            self.table_next_row();
            if let Some(_t) = self.push_tree_node("1.2.0") {
                self.table_next_row();
                ui.text("ME2/3 Legendary support");
            }
        }
        // 1.1.2
        if let Some(_t) = self.begin_table(im_str!("changelog-table"), 1) {
            self.table_next_row();
            if let Some(_t) = self.push_tree_node("1.1.2") {
                self.table_next_row();
                ui.text("Changing ME2/3 origin / notoriety will update ME1's");
                self.table_next_row();
                ui.text(
                    "Changing ME3 gender will change Loco / Lola plot corresponding to new gender",
                );
            }
        }
        // 1.1.1
        if let Some(_t) = self.begin_table(im_str!("changelog-table"), 1) {
            self.table_next_row();
            if let Some(_t) = self.push_tree_node("1.1.1") {
                self.table_next_row();
                ui.text("High CPU usage fix");
            }
        }
        // 1.1.0
        if let Some(_t) = self.begin_table(im_str!("changelog-table"), 1) {
            self.table_next_row();
            if let Some(_t) = self.push_tree_node("1.1.0") {
                self.table_next_row();
                ui.text("HiDPI fix");
                self.table_next_row();
                ui.text("Possibility to modify previously read-only ME1 raw strings");
                self.table_next_row();
                ui.text("Minor fixes");
            }
        }
        // 1.0.0
        if let Some(_t) = self.begin_table(im_str!("changelog-table"), 1) {
            self.table_next_row();
            if let Some(_t) = self.push_tree_node("1.0.0") {
                self.table_next_row();
                ui.text("Initial release");
            }
        }
        Some(())
    }
}
