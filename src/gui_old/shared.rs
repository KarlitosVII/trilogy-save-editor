use imgui::{im_str, ChildWindow, ListClipper, TabBar, TabItem};
use indexmap::IndexMap;

use crate::{
    event_handler::MainEvent,
    save_data::{
        shared::{
            appearance::HeadMorph,
            plot::{BoolVec, PlotCategory, RawPlotDb},
        },
        RawUi,
    },
};

use super::{
    imgui_utils::{Table, TreeNode},
    Gui, PlotFilterState,
};

pub enum PlotType<'a, T> {
    Vec(&'a mut Vec<T>),
    IndexMap(&'a mut IndexMap<i32, T>),
}

impl<'ui> Gui<'ui> {
    pub fn draw_head_morph(&self, has_head_morph: &mut Option<HeadMorph>) {
        let ui = self.ui;

        // Import
        if ui.button(im_str!("Import")) {
            let file =
                tinyfiledialogs::open_file_dialog("", "", Some((&["*.ron"], "Head Morph (*.ron)")));

            if let Some(path) = file {
                let _ = self.event_addr.send(MainEvent::ImportHeadMorph(path));
            }
        }
        if let Some(head_morph) = has_head_morph {
            // Export
            ui.same_line();
            if ui.button(im_str!("Export")) {
                let file = tinyfiledialogs::save_file_dialog_with_filter(
                    "",
                    "",
                    &["*.ron"],
                    "Head Morph (*.ron)",
                );

                if let Some(path) = file {
                    let _ = self
                        .event_addr
                        .send(MainEvent::ExportHeadMorph(path, Box::new(head_morph.clone())));
                }
            }
            // Remove head morph
            ui.same_line();
            ui.text("-");
            ui.same_line();
            let remove = ui.button(im_str!("Remove head morph"));
            ui.separator();

            // Raw
            let HeadMorph {
                hair_mesh,
                accessory_mesh,
                morph_features,
                offset_bones,
                lod0_vertices,
                lod1_vertices,
                lod2_vertices,
                lod3_vertices,
                scalar_parameters,
                vector_parameters,
                texture_parameters,
            } = head_morph;

            Table::new(im_str!("head-morph-table"), 1).build(ui, || {
                Table::next_row();
                self.set_next_item_open(true);
                TreeNode::new("Raw").build(ui, || {
                    Table::next_row();
                    hair_mesh.draw_raw_ui(self, "Hair Mesh");
                    Table::next_row();
                    accessory_mesh.draw_raw_ui(self, "Accessory Mesh");
                    Table::next_row();
                    morph_features.draw_raw_ui(self, "Morph Features");
                    Table::next_row();
                    offset_bones.draw_raw_ui(self, "Offset Bones");
                    Table::next_row();
                    lod0_vertices.draw_raw_ui(self, "Lod0 Vertices");
                    Table::next_row();
                    lod1_vertices.draw_raw_ui(self, "Lod1 Vertices");
                    Table::next_row();
                    lod2_vertices.draw_raw_ui(self, "Lod2 Vertices");
                    Table::next_row();
                    lod3_vertices.draw_raw_ui(self, "Lod3 Vertices");
                    Table::next_row();
                    scalar_parameters.draw_raw_ui(self, "Scalar Parameters");
                    Table::next_row();
                    vector_parameters.draw_raw_ui(self, "Vector Parameters");
                    Table::next_row();
                    texture_parameters.draw_raw_ui(self, "Texture Parameters");
                });
            });

            // Remove
            if remove {
                *has_head_morph = None;
            }
        } else {
            ui.separator()
        }
    }

    pub fn draw_plot_category(
        &self, booleans: &mut BoolVec, integers: PlotType<i32>, plot_db: &PlotCategory,
    ) {
        let PlotCategory { booleans: bool_db, integers: int_db } = plot_db;

        self.draw_plot_bools(booleans, bool_db, false);
        self.draw_plot_ints(integers, int_db, false);
    }

    pub fn draw_plot_bools(
        &self, booleans: &mut BoolVec, bool_db: &IndexMap<usize, String>, me3_imported_me1: bool,
    ) {
        let ui = self.ui;

        if bool_db.is_empty() {
            return;
        }

        let mut clipper = ListClipper::new(bool_db.len() as i32).begin(ui);
        while clipper.step() {
            for i in clipper.display_start()..clipper.display_end() {
                let (plot_id, plot_desc) = bool_db.get_index(i as usize).unwrap();
                let mut plot_id = *plot_id;
                // Add 10 000 to ME1 import ids
                if me3_imported_me1 {
                    plot_id += 10_000;
                }
                // Add missing bools
                if plot_id >= booleans.len() {
                    booleans.resize(plot_id + 1, Default::default());
                };
                let mut plot = booleans.get_mut(plot_id).unwrap();
                Table::next_row();
                plot.draw_raw_ui(self, &format!("{}##bool-{}", plot_desc, plot_desc));
            }
        }
        clipper.end();
    }

    pub fn draw_plot_ints(
        &self, mut integers: PlotType<i32>, int_db: &IndexMap<usize, String>,
        me3_imported_me1: bool,
    ) {
        let ui = self.ui;
        if int_db.is_empty() {
            return;
        }

        let mut clipper = ListClipper::new(int_db.len() as i32).begin(ui);
        while clipper.step() {
            for i in clipper.display_start()..clipper.display_end() {
                let (plot_id, plot_desc) = int_db.get_index(i as usize).unwrap();
                let mut plot_id = *plot_id;
                // Add 10 000 to ME1 import ids
                if me3_imported_me1 {
                    plot_id += 10_000;
                }
                let plot = match integers {
                    PlotType::Vec(ref mut integers) => {
                        // Add missing ints
                        if plot_id >= integers.len() {
                            integers.resize(plot_id + 1, Default::default());
                        };
                        integers.get_mut(plot_id).unwrap()
                    }
                    PlotType::IndexMap(ref mut integers) => {
                        integers.entry(plot_id as i32).or_default()
                    }
                };
                Table::next_row();
                plot.draw_raw_ui(self, &format!("{}##int-{}", plot_desc, plot_desc));
            }
        }
        clipper.end();
    }

    pub fn draw_raw_plot(
        &self, booleans: &mut BoolVec, mut integers: PlotType<i32>, mut floats: PlotType<f32>,
        plot_db: &RawPlotDb, plot_filter: &mut PlotFilterState,
    ) -> Option<()> {
        let ui = &self.ui;
        let PlotFilterState { bool_filter, int_filter, float_filter, filter_db } = plot_filter;
        if filter_db.is_none() {
            *filter_db = Some(plot_db.clone())
        }
        let RawPlotDb { booleans: bool_db, integers: int_db, floats: float_db } =
            filter_db.as_mut().unwrap();

        // Tab bar
        let _tab_bar = TabBar::new(im_str!("raw_plot")).begin(ui)?;

        // Booleans
        TabItem::new(im_str!("Booleans")).build(ui, || {
            // Filter
            if self.draw_edit_string("Filter", bool_filter) {
                *bool_db = plot_db
                    .booleans
                    .iter()
                    .filter_map(|(k, v)| {
                        (v.to_lowercase().contains(&bool_filter.to_str().to_lowercase())
                            || k.to_string().contains(bool_filter.to_str()))
                        .then(|| (*k, v.clone()))
                    })
                    .collect()
            }
            ui.separator();

            // Table
            if bool_db.is_empty() {
                return;
            }
            ChildWindow::new(im_str!("scroll")).build(ui, || {
                Table::new(im_str!("plot-table"), 1).build(ui, || {
                    let mut clipper = ListClipper::new(bool_db.len() as i32).begin(ui);
                    while clipper.step() {
                        for i in clipper.display_start()..clipper.display_end() {
                            let (&plot_id, plot_label) = bool_db.get_index(i as usize).unwrap();
                            // Add missing bools
                            if plot_id >= booleans.len() {
                                booleans.resize(plot_id + 1, Default::default());
                            };
                            let mut plot = booleans.get_mut(plot_id).unwrap();
                            Table::next_row();
                            plot.draw_raw_ui(self, &format!("{} - {}", plot_id, plot_label));
                        }
                    }
                });
            });
        });

        // Integers
        TabItem::new(im_str!("Integers")).build(ui, || {
            // Filter
            if self.draw_edit_string("Filter", int_filter) {
                *int_db = plot_db
                    .integers
                    .iter()
                    .filter_map(|(k, v)| {
                        (v.to_lowercase().contains(&int_filter.to_str().to_lowercase())
                            || k.to_string().contains(int_filter.to_str()))
                        .then(|| (*k, v.clone()))
                    })
                    .collect()
            }
            ui.separator();

            // Table
            if int_db.is_empty() {
                return;
            }
            ChildWindow::new(im_str!("scroll")).build(ui, || {
                Table::new(im_str!("plot-table"), 1).build(ui, || {
                    let mut clipper = ListClipper::new(int_db.len() as i32).begin(ui);
                    while clipper.step() {
                        for i in clipper.display_start()..clipper.display_end() {
                            let (&plot_id, plot_label) = int_db.get_index(i as usize).unwrap();
                            let plot = match integers {
                                PlotType::Vec(ref mut integers) => {
                                    // Add missing ints
                                    if plot_id >= integers.len() {
                                        integers.resize(plot_id + 1, Default::default());
                                    };
                                    integers.get_mut(plot_id).unwrap()
                                }
                                PlotType::IndexMap(ref mut integers) => {
                                    integers.entry(plot_id as i32).or_default()
                                }
                            };
                            Table::next_row();
                            plot.draw_raw_ui(self, &format!("{} - {}", plot_id, plot_label));
                        }
                    }
                });
            });
        });

        // Floats
        TabItem::new(im_str!("Floats")).build(ui, || {
            // Filter
            if self.draw_edit_string("Filter", float_filter) {
                *float_db = plot_db
                    .floats
                    .iter()
                    .filter_map(|(k, v)| {
                        (v.to_lowercase().contains(&float_filter.to_str().to_lowercase())
                            || k.to_string().contains(float_filter.to_str()))
                        .then(|| (*k, v.clone()))
                    })
                    .collect()
            }
            ui.separator();

            // Table
            if float_db.is_empty() {
                return;
            }
            ChildWindow::new(im_str!("scroll")).build(ui, || {
                Table::new(im_str!("plot-table"), 1).build(ui, || {
                    let mut clipper = ListClipper::new(float_db.len() as i32).begin(ui);
                    while clipper.step() {
                        for i in clipper.display_start()..clipper.display_end() {
                            let (&plot_id, plot_label) = float_db.get_index(i as usize).unwrap();
                            let plot = match floats {
                                PlotType::Vec(ref mut floats) => {
                                    // Add missing ints
                                    if plot_id >= floats.len() {
                                        floats.resize(plot_id + 1, Default::default());
                                    };
                                    floats.get_mut(plot_id).unwrap()
                                }
                                PlotType::IndexMap(ref mut floats) => {
                                    floats.entry(plot_id as i32).or_default()
                                }
                            };
                            Table::next_row();
                            plot.draw_raw_ui(self, &format!("{} - {}", plot_id, plot_label));
                        }
                    }
                });
            });
        });
        Some(())
    }
}
