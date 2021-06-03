use imgui::{im_str, ChildWindow};
use lazy_static::lazy_static;

use super::Gui;

lazy_static! {
    static ref CHANGELOG: Vec<(&'static str, Vec<&'static str>)> = {
        let file = include_str!("../../CHANGELOG.md");
        let mut changelog = Vec::new();
        let mut changes = Vec::new();
        let mut version = "";

        for line in file.split('\n') {
            if let Some((prefix, text)) = line.split_once(' ') {
                match prefix {
                    "##" => version = text,
                    "-" | "*" => changes.push(text),
                    _ => {}
                }
            } else {
                changelog.push((version, changes.clone()));
                changes.clear();
            }
        }

        changelog
    };
}

impl<'ui> Gui<'ui> {
    pub fn draw_change_log(&self) -> Option<()> {
        let ui = self.ui;

        let _t = ChildWindow::new("scroll").begin(ui)?;

        ui.text("Changelog");
        ui.separator();

        let mut first = true;
        for (version, changes) in CHANGELOG.iter() {
            if let Some(_t) = self.begin_table(&im_str!("{}-table", version), 1) {
                self.table_next_row();
                if first {
                    self.set_next_item_open(true);
                    first = false;
                }
                if let Some(_t) = self.push_tree_node(version) {
                    for change in changes {
                        self.table_next_row();
                        ui.text(change);
                    }
                }
            }
        }
        Some(())
    }
}
