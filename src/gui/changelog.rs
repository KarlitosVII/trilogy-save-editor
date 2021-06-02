use imgui::{im_str, ChildWindow};
use lazy_static::lazy_static;

use super::Gui;

lazy_static! {
    static ref CHANGELOGS: Vec<(&'static str, Vec<&'static str>)> = vec![
        (env!("CARGO_PKG_VERSION"), // "1.8.2",
        vec![
            "Fixed `add` button not working for War Assets and some other lists",
            "60 fps limit for those who have a display above 60Hz. (May reduce CPU / GPU usage on some configs)",
            "Various small fixes",
        ]),
        ("1.8.1",
        vec![
            "Fixed `invalid value: '6', expected variant index 0 <= i < 6`",
            "Open save dialog in the same directory of the save",
        ]),
        ("1.8.0",
        vec![
            "ME1: Renamed salvage to omnigel",
            "ME1LE: Added player talent points and some stats",
            "ME1LE: Added squad (with talents, talent points, inventory and stats)",
            "ME1LE: Added map name, location and rotation",
            "ME1LE: Added difficulty, character creation date and player controller",
            "When creating backup, copy the old save instead of renaming it. This should fix the game loading old save.",
        ]),
        ("1.7.2",
        vec![
            "Swapped ME1LE medigel and grenades",
            "Fixed some ME3 bonus power names",
            "Unhide some raw data that can be used for modding purpose (Debug name, placeables, doors, etc.)",
        ]),
        ("1.7.1",
        vec![
            "Changed dialog library, I hope it will fix `Open` dialog not opening",
        ]),
        ("1.7.0",
        vec![
            "Added ME1LE resources (credits, grenades, medigel, salvage) and face code",
            "Added ME1LE raw player inventory",
            "Changed backend (again) with a more robust one, it will choose a supported backend on your system (Vulkan, DX11/12, etc.)",
        ]),
        ("1.6.1",
        vec![
            "Changed backend from OpenGL to Vulkan, I hope it will fix GPU and OpenGL related bugs",
        ]),
        ("1.6.0",
        vec![
            "Added ME1LE Level and Current XP",
            "Added ME1LE Raw talents",
            "Converted raw texts to title case for better readability",
        ]),
        ("1.5.0",
        vec![
            "Added ME1LE `General` tab with basic information such as Name, Gender, Origin, Notoriety and Morality",
            "Added ME1LE `Head Morph` tab with Import / Export and raw data",
        ]),
        ("1.3.2",
        vec![
            "New UNC plots in ME1 (thanks to Yggge)",
            "Added clarification for editing ME1 plot in ME2 save",
        ]),
        ("1.3.1",
        vec![
            "Fix ME1LE `unexpected end of file...` error for some people",
        ]),
        ("1.3.0",
        vec![
            "Initial Mass Effect 1 Legendary support (only plot)",
        ]),
        ("1.2.0",
        vec![
            "ME2/3 Legendary support",
        ]),
        ("1.1.2",
        vec![
            "Changing ME2/3 origin / notoriety will update ME1's",
            "Changing ME3 gender will change Loco / Lola plot corresponding to new gender",
        ]),
        ("1.1.1",
        vec![
            "High CPU usage fix",
        ]),
        ("1.1.0",
        vec![
            "HiDPI fix",
            "Possibility to modify previously read-only ME1 raw strings",
            "Minor fixes",
        ]),
        ("1.0.0",
        vec![
            "Initial release",
        ]),
    ];
}

impl<'ui> Gui<'ui> {
    pub fn draw_change_log(&self) -> Option<()> {
        let ui = self.ui;

        let _t = ChildWindow::new("scroll").begin(ui)?;

        ui.text("Changelog");
        ui.separator();

        let mut first = true;
        for (version, changes) in CHANGELOGS.iter() {
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
