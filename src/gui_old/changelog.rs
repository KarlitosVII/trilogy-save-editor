use imgui::{im_str, ChildWindow};
use lazy_static::lazy_static;

use crate::gui::imgui_utils::{Table, TreeNode};

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
                changelog.push((version, changes.drain(..).collect()));
            }
        }

        changelog
    };
}

impl<'ui> Gui<'ui> {
    pub fn draw_change_log(&self) -> Option<()> {
        let ui = self.ui;

        let _scroll = ChildWindow::new("scroll").begin(ui)?;

        ui.text("Changelog");
        ui.separator();

        let mut first = true;
        for (version, changes) in CHANGELOG.iter() {
            Table::new(&im_str!("{}-table", version), 1).build(ui, || {
                Table::next_row();
                if first {
                    self.set_next_item_open(true);
                    first = false;
                }
                TreeNode::new(version).build(ui, || {
                    for change in changes {
                        Table::next_row();
                        ui.text(change);
                    }
                });
            });
        }
        Some(())
    }
}
