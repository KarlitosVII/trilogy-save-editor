use imgui::*;
use std::{cmp::Ordering, future::Future, pin::Pin};

use crate::save_data::{
    mass_effect_1::{data::*, player::*, *},
    SaveData,
};

use super::Gui;

fn get_name(names: &[Name], id: u32) -> &ImStr {
    &names[id as usize].string
}

fn get_class(classes: &[Class], id: i32) -> &Class {
    &classes[id.abs() as usize - 1]
}

fn get_object(objects: &[Object], id: i32) -> &Object {
    &objects[id as usize - 1]
}

impl<'a> Gui<'a> {
    pub async fn draw_mass_effect_1(&self, save_game: &mut Me1SaveGame) {
        let ui = self.ui;

        // Tabs
        if let Some(_t) = TabBar::new(im_str!("me1-tabs")).begin(ui) {
            if let Some(_t) = TabItem::new(im_str!("Raw")).begin(ui) {
                if let Some(_t) = ChildWindow::new("mass_effect_1").size([0.0, 0.0]).begin(ui) {
                    if let Some(_t) = TreeNode::new(im_str!("Mass Effect 1")).push(ui) {
                        // Player
                        self.draw_player(&mut save_game.player).await;
                        // State
                        save_game.state.draw_raw_ui(self, "state.sav").await;
                    }
                }
            }
        }
    }

    async fn draw_player(&self, player: &mut Player) {
        let ui = self.ui;
        let Player { names, classes, objects, datas, .. } = player;

        if let Some(_t) = TreeNode::new(im_str!("player.sav")).push(ui) {
            // Data
            for (i, data) in datas.iter_mut().enumerate() {
                let object_name = get_name(names, objects[i].object_name_id);

                // Properties
                if let Some(_t) = TreeNode::new(&im_str!("{} {}", i, object_name)).push(ui) {
                    for (j, property) in data.properties.iter_mut().enumerate() {
                        self.draw_property(names, classes, objects, j, property).await;
                    }
                }
            }
        }
    }

    fn draw_property(
        &'a self, names: &'a [Name], classes: &'a [Class], objects: &'a [Object], ident: usize,
        property: &'a mut Property,
    ) -> Pin<Box<dyn Future<Output = ()> + 'a>> {
        Box::pin(async move {
            match property {
                Property::Array { name_id, array, .. } => {
                    self.draw_array_property(
                        names,
                        classes,
                        objects,
                        ident,
                        get_name(names, *name_id),
                        array,
                    )
                    .await
                }
                Property::Bool { name_id, value, .. } => {
                    self.draw_edit_bool(
                        im_str!("{}##{}", get_name(names, *name_id), ident).to_str(),
                        value,
                    )
                    .await
                }
                Property::Float { name_id, value, .. } => {
                    self.draw_edit_f32(
                        im_str!("{}##{}", get_name(names, *name_id), ident).to_str(),
                        value,
                    )
                    .await
                }
                Property::Int { name_id, value, .. } => {
                    self.draw_edit_i32(
                        im_str!("{}##{}", get_name(names, *name_id), ident).to_str(),
                        value,
                    )
                    .await
                }
                Property::Name { name_id, value_name_id, .. } => {
                    self.draw_text(
                        get_name(names, *value_name_id),
                        get_name(names, *name_id),
                        ident,
                    )
                    .await;
                }
                Property::Object { name_id, object_id, .. } => {
                    let object_name = match (*object_id).cmp(&0) {
                        Ordering::Less => {
                            let class = get_class(classes, *object_id);
                            get_name(names, class.class_name_id)
                        }
                        Ordering::Greater => {
                            let object = get_object(objects, *object_id);
                            get_name(names, object.object_name_id)
                        }
                        Ordering::Equal => &im_str!("Class"),
                    };
                    self.draw_text(object_name, get_name(names, *name_id), ident).await;
                }
                Property::Str { name_id, string, .. } => {
                    self.draw_edit_string(
                        im_str!("{}##{}", get_name(names, *name_id), ident).to_str(),
                        string,
                    )
                    .await
                }
                Property::StringRef { name_id, value, .. } => {
                    self.draw_edit_i32(
                        im_str!("{}##{}", get_name(names, *name_id), ident).to_str(),
                        value,
                    )
                    .await;
                }
                Property::Struct { name_id, struct_name_id, properties, .. } => {
                    self.draw_struct_property(
                        names,
                        classes,
                        objects,
                        ident,
                        &im_str!(
                            "{} : {}",
                            get_name(names, *name_id),
                            get_name(names, *struct_name_id)
                        ),
                        properties,
                    )
                    .await
                }
                _ => {}
            }
        })
    }

    async fn draw_array_property(
        &self, names: &[Name], classes: &[Class], objects: &[Object], ident: usize, label: &ImStr,
        array: &mut [ArrayType],
    ) {
        let ui = self.ui;
        if let Some(_t) = TreeNode::new(&im_str!("{}", ident)).label(label).push(ui) {
            if !array.is_empty() {
                for (i, property) in array.iter_mut().enumerate() {
                    match property {
                        ArrayType::Int(int) => {
                            int.draw_raw_ui(self, im_str!("{}##{}", i, i).to_str()).await
                        }
                        ArrayType::Object(object_id) => {
                            let object_name = if *object_id != 0 {
                                let object = get_object(objects, *object_id);
                                get_name(names, object.object_name_id)
                            } else {
                                &im_str!("Null")
                            };
                            self.draw_text(object_name, &im_str!("{}", i), i).await;
                        }
                        ArrayType::Vector(vector) => {
                            vector.draw_raw_ui(self, im_str!("{}##{}", i, i).to_str()).await
                        }
                        ArrayType::String(string) => {
                            string.draw_raw_ui(self, im_str!("##{}", i).to_str()).await
                        }
                        ArrayType::Properties(properties) => {
                            if let Some(_t) =
                                TreeNode::new(&im_str!("##{}", i)).label(&im_str!("{}", i)).push(ui)
                            {
                                for (j, property) in properties.iter_mut().enumerate() {
                                    self.draw_property(names, classes, objects, j, property).await;
                                }
                            }
                        }
                    }
                }
            } else {
                ui.text("Empty");
            }
        }
    }

    async fn draw_struct_property(
        &self, names: &[Name], classes: &[Class], objects: &[Object], ident: usize, label: &ImStr,
        struct_property: &mut StructType,
    ) {
        match struct_property {
            StructType::LinearColor(color) => {
                color.draw_raw_ui(self, im_str!("{}##{}", label, ident).to_str()).await
            }
            StructType::Vector(vector) => {
                vector.draw_raw_ui(self, im_str!("{}##{}", label, ident).to_str()).await
            }
            StructType::Rotator(rotator) => {
                rotator.draw_raw_ui(self, im_str!("{}##{}", label, ident).to_str()).await
            }
            StructType::Properties(properties) => {
                if let Some(_t) = TreeNode::new(&im_str!("##{}", ident)).label(label).push(self.ui)
                {
                    for (i, property) in properties.iter_mut().enumerate() {
                        self.draw_property(names, classes, objects, i, property).await;
                    }
                }
            }
        }
    }

    async fn draw_text(&self, text: &ImStr, label: &ImStr, ident: usize) {
        let ui = self.ui;
        self.draw_colored_bg(im_str!("##{}", ident).to_str(), || {
            ui.align_text_to_frame_padding();
            ui.text(im_str!(" {}", text));
            ui.same_line_with_pos(ui.window_content_region_width() * 2.0 / 3.0 - 11.0);
            ui.text(label);
        });
    }
}
