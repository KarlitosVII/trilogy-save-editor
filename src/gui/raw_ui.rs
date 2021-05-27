use if_chain::if_chain;
use imgui::{
    im_str, ColorEdit, ComboBox, Condition, ImStr, ImString, InputFloat, InputInt, ListClipper,
    TreeNode,
};
use indexmap::IndexMap;
use std::{fmt::Display, hash::Hash};

use crate::save_data::{
    shared::{plot::BoolSlice, Guid},
    RawUi,
};

use super::Gui;

impl<'ui> Gui<'ui> {
    // Edit boxes
    pub fn draw_edit_string(&self, ident: &str, value: &mut ImString) {
        self.ui.input_text(&ImString::new(ident), value).resize_buffer(true).build();
    }

    pub fn draw_edit_bool(&self, ident: &str, value: &mut bool) {
        let ui = self.ui;

        let width = ui.push_item_width(120.0);
        ui.checkbox(&ImString::new(ident), value);
        width.pop(ui);
    }

    pub fn draw_edit_i32(&self, ident: &str, value: &mut i32) {
        let ui = self.ui;

        let width = ui.push_item_width(120.0);
        InputInt::new(ui, &ImString::new(ident), value).build();
        width.pop(ui);
    }

    pub fn draw_edit_f32(&self, ident: &str, value: &mut f32) {
        let ui = self.ui;

        let width = ui.push_item_width(120.0);
        InputFloat::new(ui, &ImString::new(ident), value).build();
        width.pop(ui);
    }

    pub fn draw_edit_enum(&self, ident: &str, current_item: &mut usize, items: &[&ImStr]) -> bool {
        let ui = self.ui;

        let width = ui.push_item_width(200.0);
        let edited =
            ComboBox::new(&ImString::new(ident)).build_simple_string(ui, current_item, items);
        width.pop(ui);
        edited
    }

    pub fn draw_edit_color(&self, ident: &str, color: &mut [f32; 4]) {
        let ui = self.ui;

        let width = ui.push_item_width(200.0);
        ColorEdit::new(&ImString::new(ident), color).build(ui);
        width.pop(ui);
    }

    pub fn draw_edit_guid(&self, ident: &str, guid: &mut Guid) {
        let ui = self.ui;
        {
            let width = ui.push_item_width(65.0);
            ui.input_text(&im_str!("##{}-part1", ident), &mut guid.part1)
                .chars_hexadecimal(true)
                .build();
            width.pop(ui);
        }
        ui.same_line();
        ui.text("-");
        {
            ui.same_line();
            let width = ui.push_item_width(36.0);
            ui.input_text(&im_str!("##{}-part2", ident), &mut guid.part2)
                .chars_hexadecimal(true)
                .build();
            width.pop(ui);
        }
        ui.same_line();
        ui.text("-");
        {
            ui.same_line();
            let width = ui.push_item_width(36.0);
            ui.input_text(&im_str!("##{}-part3", ident), &mut guid.part3)
                .chars_hexadecimal(true)
                .build();
            width.pop(ui);
        }
        ui.same_line();
        ui.text("-");
        {
            ui.same_line();
            let width = ui.push_item_width(36.0);
            ui.input_text(&im_str!("##{}-part4", ident), &mut guid.part4)
                .chars_hexadecimal(true)
                .build();
            width.pop(ui);
        }
        ui.same_line();
        ui.text("-");
        {
            ui.same_line();
            let width = ui.push_item_width(93.0);
            ui.input_text(&im_str!("{}##{}-part5", ident, ident), &mut guid.part5)
                .chars_hexadecimal(true)
                .build();
            width.pop(ui);
        }
    }

    // View widgets
    pub fn draw_struct(&self, ident: &str, fields: &mut [(&mut dyn RawUi, &str)]) {
        if let Some(_t) = self.push_tree_node(ident) {
            if let Some(_t) = self.begin_table(&ImString::new(ident), 1) {
                for (field, ident) in fields {
                    self.table_next_row();
                    field.draw_raw_ui(self, ident);
                }
            }
        }
    }

    pub fn draw_boolvec(&self, ident: &str, list: &mut BoolSlice) {
        let ui = self.ui;
        // Tree node
        let _t = match self.push_tree_node(ident) {
            Some(t) => t,
            None => return,
        };

        // Table
        let _t = match self.begin_table(&ImString::new(ident), 1) {
            Some(t) => t,
            None => return,
        };

        if !list.is_empty() {
            let mut clipper = ListClipper::new(list.len() as i32).begin(ui);
            while clipper.step() {
                for i in clipper.display_start()..clipper.display_end() {
                    self.table_next_row();
                    list.get_mut(i as usize).unwrap().draw_raw_ui(self, &i.to_string());
                }
            }
        } else {
            self.table_next_row();
            ui.text("Empty");
        }
    }

    pub fn draw_vec<T>(&self, ident: &str, list: &mut Vec<T>)
    where
        T: RawUi + Default,
    {
        let ui = self.ui;

        // Tree node
        let _t = match self.push_tree_node(ident) {
            Some(t) => t,
            None => return,
        };

        // Table
        let _t = match self.begin_table(&ImString::new(ident), 1) {
            Some(t) => t,
            None => return,
        };

        if !list.is_empty() {
            // Item
            let mut remove = None;
            for (i, item) in list.iter_mut().enumerate() {
                self.table_next_row();
                ui.align_text_to_frame_padding();
                if ui.small_button(&im_str!("remove##remove-{}", i)) {
                    remove = Some(i);
                }
                ui.same_line();
                item.draw_raw_ui(self, &i.to_string());
            }

            // Remove
            if let Some(i) = remove {
                list.remove(i);
            }
        } else {
            self.table_next_row();
            ui.text("Empty");
        }

        // Add
        self.table_next_row();
        if ui.button(im_str!("add")) {
            // Ça ouvre automatiquement le tree node de l'élément ajouté
            TreeNode::new(&im_str!("{}", list.len()))
                .opened(true, Condition::Always)
                .build(ui, || {});

            list.push(T::default());
        }
    }

    pub fn draw_indexmap<K, V>(&self, ident: &str, list: &mut IndexMap<K, V>)
    where
        K: RawUi + Eq + Hash + Default + Display,
        V: RawUi + Default,
    {
        let ui = self.ui;

        // Tree node
        let _t = match self.push_tree_node(ident) {
            Some(t) => t,
            None => return,
        };

        // Table
        let _t = match self.begin_table(&ImString::new(ident), 1) {
            Some(t) => t,
            None => return,
        };

        if !list.is_empty() {
            // Item
            let mut remove = None;
            for i in 0..list.len() {
                self.table_next_row();
                ui.align_text_to_frame_padding();
                if ui.small_button(&im_str!("remove##remove-{}", i)) {
                    remove = Some(i);
                }
                ui.same_line();

                if_chain! {
                    if let Some((key, value)) = list.get_index_mut(i);
                    if let Some(_t) = self.push_tree_node(&format!("{}##{}", key, i));
                    if let Some(_t) = self.begin_table(&im_str!("table-{}", i), 1);
                    then {
                        self.table_next_row();
                        key.draw_raw_ui(self, "id##key");
                        self.table_next_row();
                        value.draw_raw_ui(self, "value##value");
                    }
                }
            }

            // Remove
            if let Some(i) = remove {
                list.shift_remove_index(i);
            }
        } else {
            self.table_next_row();
            ui.text("Empty");
        }

        // Add
        self.table_next_row();
        if ui.button(im_str!("add")) {
            // Ça ouvre automatiquement le tree node de l'élément ajouté
            let new_k = K::default();
            TreeNode::new(&im_str!("{}", list.len()))
                .label(&ImString::new(new_k.to_string()))
                .opened(true, Condition::Always)
                .build(ui, || {});

            // FIXME: Ajout d'un nouvel élément si K = 0i32 déjà présent
            list.entry(new_k).or_default();
        }
    }
}
