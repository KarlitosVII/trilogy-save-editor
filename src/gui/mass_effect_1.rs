use async_recursion::async_recursion;
use imgui::*;
use std::cmp::Ordering;

use crate::save_data::{
    common::plot::{Me1PlotTable, PlotCategory},
    mass_effect_1::{
        data::{ArrayType, Property, StructType},
        player::Player,
        Me1SaveGame,
    },
    SaveData,
};

use super::*;

impl<'ui> Gui<'ui> {
    pub async fn draw_mass_effect_1(
        &self, save_game: &mut Me1SaveGame, known_plots: &KnownPlotsState,
    ) {
        let ui = self.ui;

        // Tabs
        if let Some(_t) = TabBar::new(im_str!("mass_effect_1")).begin(ui) {
            // Plot
            if let Some(_t) = TabItem::new(im_str!("Plot")).begin(ui) {
                if let Some(me1_known_plot) = &known_plots.me1 {
                    self.draw_me1_known_plot(&mut save_game.state.plot, me1_known_plot).await;
                }
            }
            // Raw
            if let Some(_t) = TabItem::new(im_str!("Raw")).begin(ui) {
                self.set_next_item_open(true);
                if let Some(_t) = self.push_tree_node("Mass Effect 1") {
                    // Player
                    self.draw_raw_player(&save_game.player).await;
                    // State
                    save_game.state.draw_raw_ui(self, "State").await;
                }
            }
        }
    }

    pub async fn draw_me1_known_plot(
        &self, me1_plot_table: &mut Me1PlotTable, me1_known_plot: &Me1KnownPlot,
    ) {
        let ui = self.ui;
        let Me1KnownPlot { player_crew, missions } = me1_known_plot;

        // Tab bar
        let _t = match TabBar::new(im_str!("plot-tab")).begin(ui) {
            Some(t) => t,
            None => return,
        };

        let categories = [(im_str!("Player / Crew"), player_crew), (im_str!("Missions"), missions)];

        for (title, plot_map) in &categories {
            if let Some(_t) = TabItem::new(title).begin(ui) {
                for (category_name, known_plot) in plot_map.iter() {
                    if let Some(_t) = self.begin_table(&im_str!("{}-table", category_name), 1) {
                        self.table_next_row();
                        if let Some(_t) = self.push_tree_node(category_name) {
                            self.draw_me1_plot_category(me1_plot_table, known_plot).await;
                        }
                    }
                }
            }
        }
    }

    async fn draw_me1_plot_category(
        &self, plot_table: &mut Me1PlotTable, known_plot: &PlotCategory,
    ) {
        let PlotCategory { booleans, ints } = known_plot;

        if booleans.is_empty() && ints.is_empty() {
            return;
        }

        // Booleans
        for (plot_id, plot_desc) in booleans {
            let plot = plot_table.bool_variables.get_mut(*plot_id);
            if let Some(mut plot) = plot {
                self.table_next_row();
                plot.draw_raw_ui(self, plot_desc).await;
            }
        }
        // Integers
        for (plot_id, plot_desc) in ints {
            let plot = plot_table.int_variables.get_mut(*plot_id);
            if let Some(plot) = plot {
                self.table_next_row();
                plot.draw_raw_ui(self, plot_desc).await;
            }
        }
    }

    async fn draw_raw_player(&self, player: &Player) {
        for (i, _) in player.objects.iter().enumerate() {
            let object_id = i as i32 + 1;
            let object = player.get_object(object_id);
            let object_name = player.get_name(object.object_name_id);

            match object_name.to_str() {
                "CurrentGame" => self.draw_object(player, i, None, object_id).await,
                _ => continue,
            }
        }
    }

    async fn draw_object(
        &self, player: &Player, ident: usize, property_name: Option<&ImStr>, object_id: i32,
    ) {
        let object = player.get_object(object_id);
        let object_name = player.get_name(object.object_name_id);

        let property_name = match property_name {
            Option::Some(property_name) => im_str!("{} : {}", object_name, property_name),
            Option::None => object_name.to_owned(),
        };

        if let Some(_t) = self.push_tree_node(&format!("{}##{}", property_name, ident)) {
            if let Some(_t) = self.begin_table(im_str!("object-table"), 1) {
                let mut data = player.get_data(object_id).borrow_mut();
                for (i, property) in data.iter_mut().enumerate() {
                    self.draw_property(player, i, property).await;
                }
            }
        }
    }

    #[async_recursion(?Send)]
    async fn draw_property(&self, player: &Player, ident: usize, property: &mut Property) {
        match property {
            Property::Byte { .. } | Property::None { .. } => return,
            _ => {
                self.table_next_row();
            }
        }

        match property {
            Property::Array { name_id, array, .. } => {
                self.draw_array_property(
                    player,
                    &format!("{}##{}", player.get_name(*name_id), ident),
                    array,
                )
                .await
            }
            Property::Bool { name_id, value, .. } => {
                self.draw_edit_bool(
                    im_str!("{}##bool-{}", player.get_name(*name_id), ident).to_str(),
                    value,
                )
                .await
            }
            Property::Float { name_id, value, .. } => {
                self.draw_edit_f32(
                    im_str!("{}##f32-{}", player.get_name(*name_id), ident).to_str(),
                    value,
                )
                .await
            }
            Property::Int { name_id, value, .. } => {
                self.draw_edit_i32(
                    im_str!("{}##i32-{}", player.get_name(*name_id), ident).to_str(),
                    value,
                )
                .await
            }
            Property::Name { name_id, value_name_id, .. } => {
                self.draw_text(player.get_name(*value_name_id), Some(player.get_name(*name_id)))
                    .await;
            }
            Property::Object { name_id, object_id, .. } => {
                match (*object_id).cmp(&0) {
                    Ordering::Greater => {
                        // Object
                        let property_name = player.get_name(*name_id);
                        self.draw_object(player, ident, Some(property_name), *object_id).await;
                    }
                    Ordering::Less => {
                        // Class
                        let class = player.get_class(*object_id);
                        let class_name = player.get_name(class.class_name_id);
                        self.draw_text(class_name, Some(player.get_name(*name_id))).await;
                    }
                    Ordering::Equal => {
                        // Null => Nom de classe par défaut
                        self.draw_text(im_str!("Class"), Some(player.get_name(*name_id))).await
                    }
                };
            }
            Property::Str { name_id, string, .. } => {
                self.draw_edit_string(
                    im_str!("{}##string-{}", player.get_name(*name_id), ident).to_str(),
                    string,
                )
                .await
            }
            Property::StringRef { name_id, value, .. } => {
                self.draw_edit_i32(
                    im_str!("{}##string-ref-{}", player.get_name(*name_id), ident).to_str(),
                    value,
                )
                .await;
            }
            Property::Struct { name_id, struct_name_id, properties, .. } => {
                self.draw_struct_property(
                    player,
                    ident,
                    &im_str!(
                        "{} : {}",
                        player.get_name(*struct_name_id),
                        player.get_name(*name_id),
                    ),
                    properties,
                )
                .await
            }
            Property::Byte { .. } | Property::None { .. } => unreachable!(),
        }
    }

    async fn draw_array_property(&self, player: &Player, ident: &str, array: &mut [ArrayType]) {
        let ui = self.ui;

        // Tree node
        let _t = match self.push_tree_node(ident) {
            Some(t) => t,
            None => return,
        };

        // Table
        let _t = match self.begin_table(im_str!("array-table"), 1) {
            Some(t) => t,
            None => return,
        };

        if !array.is_empty() {
            for (i, property) in array.iter_mut().enumerate() {
                self.table_next_row();
                match property {
                    ArrayType::Int(int) => {
                        int.draw_raw_ui(self, im_str!("{}##int-{}", i, i).to_str()).await
                    }
                    ArrayType::Object(object_id) => {
                        if *object_id != 0 {
                            // Object
                            self.draw_object(player, i, None, *object_id).await;
                        } else {
                            // Null
                            self.draw_text(im_str!("Null"), None).await;
                        }
                    }
                    ArrayType::Vector(vector) => {
                        vector.draw_raw_ui(self, im_str!("{}##vector-{}", i, i).to_str()).await
                    }
                    ArrayType::String(string) => {
                        string.draw_raw_ui(self, im_str!("##string-{}", i).to_str()).await
                    }
                    ArrayType::Properties(properties) => {
                        if let Some(_t) = self.push_tree_node(&i.to_string()) {
                            if let Some(_t) = self.begin_table(im_str!("array-properties-table"), 1)
                            {
                                for (j, property) in properties.iter_mut().enumerate() {
                                    self.draw_property(player, j, property).await;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            ui.text(" Empty");
        }
    }

    async fn draw_struct_property(
        &self, player: &Player, ident: usize, label: &ImStr, struct_property: &mut StructType,
    ) {
        match struct_property {
            StructType::LinearColor(color) => {
                color.draw_raw_ui(self, &format!("{}##linear-color-{}", label, ident)).await
            }
            StructType::Vector(vector) => {
                vector.draw_raw_ui(self, &format!("{}##vector-{}", label, ident)).await
            }
            StructType::Rotator(rotator) => {
                rotator.draw_raw_ui(self, &format!("{}##rotator-{}", label, ident)).await
            }
            StructType::Properties(properties) => {
                if let Some(_t) = self.push_tree_node(&format!("{}##{}", label, ident)) {
                    if let Some(_t) = self.begin_table(im_str!("struct-properties-table"), 1) {
                        for (i, property) in properties.iter_mut().enumerate() {
                            self.draw_property(player, i, property).await;
                        }
                    }
                }
            }
        }
    }

    async fn draw_text(&self, text: &ImStr, label: Option<&ImStr>) {
        let ui = self.ui;
        ui.text(text);

        if let Some(label) = label {
            ui.same_line_with_pos(500.0);
            ui.text(label);
        }
    }
}
