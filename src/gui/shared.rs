use imgui::{im_str, ListClipper};

use crate::{
    event_handler::MainEvent,
    save_data::{
        shared::{
            appearance::HeadMorph,
            plot::{BoolVec, PlotCategory},
        },
        RawUi,
    },
};

use super::{
    imgui_utils::{Table, TreeNode},
    Gui,
};

impl<'ui> Gui<'ui> {
    pub fn draw_plot_category(
        &self, booleans: &mut BoolVec, integers: &mut Vec<i32>, plot_db: &PlotCategory,
    ) {
        let ui = self.ui;
        let PlotCategory { booleans: cat_bools, integers: cat_ints } = plot_db;

        if cat_bools.is_empty() && cat_ints.is_empty() {
            return;
        }

        // Booleans
        let mut clipper = ListClipper::new(cat_bools.len() as i32).begin(ui);
        while clipper.step() {
            for i in clipper.display_start()..clipper.display_end() {
                let (plot_id, plot_desc) = cat_bools.get_index(i as usize).unwrap();
                let plot = booleans.get_mut(*plot_id);
                if let Some(mut plot) = plot {
                    Table::next_row();
                    plot.draw_raw_ui(self, &format!("{}##bool-{}", plot_desc, plot_desc));
                }
            }
        }

        // Integers
        let mut clipper = ListClipper::new(cat_ints.len() as i32).begin(ui);
        while clipper.step() {
            for i in clipper.display_start()..clipper.display_end() {
                let (plot_id, plot_desc) = cat_ints.get_index(i as usize).unwrap();
                let plot = integers.get_mut(*plot_id);
                if let Some(plot) = plot {
                    Table::next_row();
                    plot.draw_raw_ui(self, &format!("{}##int-{}", plot_desc, plot_desc));
                }
            }
        }
    }

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
}
