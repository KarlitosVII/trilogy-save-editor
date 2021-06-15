use std::ops::IndexMut;

use if_chain::if_chain;
use imgui::{
    im_str, ChildWindow, ComboBox, ImStr, ImString, ListClipper, Selectable, TabBar, TabItem,
};

use crate::{
    databases::Database,
    save_data::{
        mass_effect_1::item_db::DbItem,
        mass_effect_1_le::{
            player::{ComplexTalent, Item, ItemLevel, Player},
            squad::Henchman,
            Me1LeSaveData,
        },
        shared::player::{Notoriety, Origin},
        RawUi,
    },
};

use super::Gui;

impl<'ui> Gui<'ui> {
    pub fn draw_mass_effect_1_le(&self, save_game: &mut Me1LeSaveData) -> Option<()> {
        let ui = self.ui;

        // Tab bar
        let _t = TabBar::new(im_str!("mass_effect_1_le")).begin(ui)?;

        // General
        if_chain! {
            if let Some(_t) = TabItem::new(im_str!("General")).begin(ui);
            if let Some(_t) = ChildWindow::new(im_str!("scroll")).begin(ui);
            then {
                self.draw_me1_le_general(save_game);
            }
        }

        // Plot
        if let Some(_t) = TabItem::new(im_str!("Plot")).begin(ui) {
            self.draw_me1_plot_db(&mut save_game.plot);
        }

        // Inventory
        if_chain! {
            if let Some(_t) = TabItem::new(im_str!("Inventory")).begin(ui);
            if let Some(_t) = ChildWindow::new(im_str!("scroll")).begin(ui);
            then {
                self.draw_me1_le_inventory_tab(save_game);
            }
        }
        // Head Morph
        if_chain! {
            if let Some(_t) = TabItem::new(im_str!("Head Morph")).begin(ui);
            if let Some(_t) = ChildWindow::new(im_str!("scroll")).begin(ui);
            then {
                self.draw_me3_and_le_head_morph(&mut save_game.player.head_morph);
            }
        }
        // Raw
        if_chain! {
            if let Some(_t) = TabItem::new(im_str!("Raw")).begin(ui);
            if let Some(_t) = ChildWindow::new(im_str!("scroll")).begin(ui);
            then {
                self.set_next_item_open(true);
                save_game.draw_raw_ui(self, "Mass Effect 1");
            }
        }
        Some(())
    }

    fn draw_me1_le_general(&self, save_game: &mut Me1LeSaveData) -> Option<()> {
        let ui = self.ui;
        let Me1LeSaveData { plot, player, difficulty, squad, .. } = save_game;
        let Player {
            is_female,
            level,
            current_xp,
            first_name,
            origin,
            notoriety,
            talent_points,
            credits,
            medigel,
            grenades,
            omnigel,
            face_code,
            complex_talents,
            ..
        } = player;

        // 1ère colonne
        let _t = self.begin_columns(2)?;
        self.table_next_row();

        // Role Play
        if let Some(_t) = self.begin_table(im_str!("role-play-table"), 1) {
            self.table_next_row();
            self.set_next_item_open(true);
            if let Some(_t) = self.push_tree_node("Role-Play") {
                self.table_next_row();
                first_name.draw_raw_ui(self, "Name");

                // Gender
                self.table_next_row();
                let mut gender = *is_female as usize;
                const GENDER_LIST: [&ImStr; 2] = [im_str!("Male"), im_str!("Female")];
                {
                    if self.draw_edit_enum("Gender", &mut gender, &GENDER_LIST) {
                        *is_female = gender != 0;
                    }

                    ui.same_line();
                    self.draw_help_marker(
                        "If you change your gender, disable the head morph or import an appropriate one.\n\
                        Otherwise, Saren and his Geths will be the least of your worries..."
                    );
                }

                self.table_next_row();
                let mut origin_idx = origin.clone() as usize;
                const ORIGIN_LIST: [&ImStr; 4] =
                    [im_str!("None"), im_str!("Spacer"), im_str!("Colonist"), im_str!("Earthborn")];
                {
                    if self.draw_edit_enum("Origin", &mut origin_idx, &ORIGIN_LIST) {
                        // Enum
                        *origin = match origin_idx {
                            0 => Origin::None,
                            1 => Origin::Spacer,
                            2 => Origin::Colonist,
                            3 => Origin::Earthborn,
                            _ => unreachable!(),
                        };

                        // ME1 plot
                        if let Some(me1_origin) = plot.int_variables.get_mut(1) {
                            *me1_origin = origin_idx as i32;
                        }
                    }
                }

                self.table_next_row();
                let mut notoriety_idx = notoriety.clone() as usize;
                const NOTORIETY_LIST: [&ImStr; 4] = [
                    im_str!("None"),
                    im_str!("Survivor"),
                    im_str!("War Hero"),
                    im_str!("Ruthless"),
                ];
                {
                    if self.draw_edit_enum("Notoriety", &mut notoriety_idx, &NOTORIETY_LIST) {
                        // Enum
                        *notoriety = match notoriety_idx {
                            0 => Notoriety::None,
                            1 => Notoriety::Survivor,
                            2 => Notoriety::Warhero,
                            3 => Notoriety::Ruthless,
                            _ => unreachable!(),
                        };

                        // ME1 plot
                        if let Some(me1_notoriety) = plot.int_variables.get_mut(2) {
                            *me1_notoriety = notoriety_idx as i32;
                        }
                    }
                }

                self.table_next_row();
                face_code.draw_raw_ui(self, "Identity Code");
                ui.same_line();
                self.draw_help_marker(
                    "If you change this you can display whatever you want in the menus\n\
                    in place of your `Identity Code`, which is pretty cool !",
                );
            }
        }

        // Morality
        if let Some(_t) = self.begin_table(im_str!("morality-table"), 1) {
            self.table_next_row();
            self.set_next_item_open(true);
            if let Some(_t) = self.push_tree_node("Morality") {
                if let Some(paragon) = plot.int_variables.get_mut(47) {
                    self.table_next_row();
                    paragon.draw_raw_ui(self, "Paragon");
                }

                if let Some(renegade) = plot.int_variables.get_mut(46) {
                    self.table_next_row();
                    renegade.draw_raw_ui(self, "Renegade");
                }
            }
        }

        // Resources
        if let Some(_t) = self.begin_table(im_str!("resources-table"), 1) {
            self.table_next_row();
            self.set_next_item_open(true);
            if let Some(_t) = self.push_tree_node("Resources") {
                self.table_next_row();
                credits.draw_raw_ui(self, "Credits");
                self.table_next_row();
                medigel.draw_raw_ui(self, "Medigel");
                self.table_next_row();
                grenades.draw_raw_ui(self, "Grenades");
                self.table_next_row();
                omnigel.draw_raw_ui(self, "Omnigel");
            }
        }

        // 2ème colonne
        self.table_next_column();

        // General
        if let Some(_t) = self.begin_table(im_str!("general-table"), 1) {
            self.table_next_row();
            self.set_next_item_open(true);
            if let Some(_t) = self.push_tree_node("General") {
                self.table_next_row();
                difficulty.draw_raw_ui(self, "Difficulty");
            }
        }

        // Gameplay
        if let Some(_t) = self.begin_table(im_str!("gameplay-table"), 1) {
            self.table_next_row();
            self.set_next_item_open(true);
            if let Some(_t) = self.push_tree_node("Gameplay") {
                self.table_next_row();
                level.draw_raw_ui(self, "Level");
                ui.same_line();
                self.draw_help_marker("Classic mode (1 - 60)");

                self.table_next_row();
                current_xp.draw_raw_ui(self, "Current XP");
                self.table_next_row();
                talent_points.draw_raw_ui(self, "Talent Points");
                self.table_next_row();
                self.draw_me1_le_reset_talents("player", talent_points, complex_talents);
            }
        }

        // Squad
        if let Some(_t) = self.begin_table(im_str!("squad-table"), 1) {
            self.table_next_row();
            self.set_next_item_open(true);
            if let Some(_t) = self.push_tree_node("Squad") {
                for Henchman { tag, talent_points, complex_talents, .. } in squad {
                    let character_name = match tag.to_str() {
                        "hench_asari" => "Liara",
                        "hench_humanfemale" => "Ashley",
                        "hench_humanmale" => "Kaidan",
                        "hench_krogan" => "Wrex",
                        "hench_quarian" => "Tali'Zorah",
                        "hench_turian" => "Garrus",
                        _ => continue,
                    };

                    self.table_next_row();
                    self.draw_me1_le_reset_talents(character_name, talent_points, complex_talents);
                }
            }
        }

        Some(())
    }

    fn draw_me1_le_reset_talents(
        &self, character_name: &str, talent_points: &mut i32, complex_talents: &mut [ComplexTalent],
    ) {
        let ui = self.ui;

        if ui.button(&im_str!("Reset {} talents", character_name)) {
            for talent in complex_talents {
                *talent_points += talent.ranks;
                talent.ranks = 0;
            }
        }
    }

    fn draw_me1_le_inventory_tab(&self, savegame: &mut Me1LeSaveData) -> Option<()> {
        // 1ère colonne
        let _t = self.begin_columns(2)?;
        self.table_next_row();

        let Me1LeSaveData { player, squad, .. } = savegame;
        let Player { inventory, .. } = player;

        // Player
        self.draw_me1_le_equipped_items("Player Equipped", &mut inventory.equipped);
        self.draw_me1_le_equipped_items("Player Quick Slots", &mut inventory.quick_slots);

        // Squad

        for Henchman { tag, equipped, quick_slots, .. } in squad {
            let (character_equipped, character_quick_slots) = match tag.to_str() {
                "hench_asari" => ("Liara Equipped", "Liara Quick Slots"),
                "hench_humanfemale" => ("Ashley Equipped", "Ashley Quick Slots"),
                "hench_humanmale" => ("Kaidan Equipped", "Kaidan Quick Slots"),
                "hench_krogan" => ("Wrex Equipped", "Wrex Quick Slots"),
                "hench_quarian" => ("Tali'Zorah Equipped", "Tali'Zorah Quick Slots"),
                "hench_turian" => ("Garrus Equipped", "Garrus Quick Slots"),
                _ => continue,
            };

            self.draw_me1_le_equipped_items(character_equipped, equipped);
            self.draw_me1_le_equipped_items(character_quick_slots, quick_slots);
        }

        // 2ème colonne
        self.table_next_column();

        self.draw_me1_le_inventory(&mut inventory.inventory);

        Some(())
    }

    fn draw_me1_le_equipped_items(&self, label: &str, items: &mut Vec<Item>) -> Option<()> {
        let ui = self.ui;

        let _t = self.begin_table(&im_str!("{}-table", label), 1)?;
        self.table_next_row();
        self.set_next_item_open(true);
        let _t = self.push_tree_node(label)?;

        if !items.is_empty() {
            let mut clipper = ListClipper::new(items.len() as i32).begin(ui);
            while clipper.step() {
                for i in clipper.display_start()..clipper.display_end() {
                    self.table_next_row();

                    let current_item = items.index_mut(i as usize);

                    let width = ui.push_item_width(375.0);
                    self.draw_me1_le_item(i, current_item);
                    width.pop(ui);
                }
            }
        } else {
            self.table_next_row();
            ui.text("Empty");
        }

        Some(())
    }

    fn draw_me1_le_inventory(&self, inventory: &mut Vec<Item>) -> Option<()> {
        let ui = self.ui;

        let _t = self.begin_table(im_str!("inventory-table"), 1)?;
        self.table_next_row();
        self.set_next_item_open(true);
        let _t = self.push_tree_node("Inventory")?;

        if !inventory.is_empty() {
            let mut clipper = ListClipper::new(inventory.len() as i32).begin(ui);
            let mut remove = None;
            while clipper.step() {
                for i in clipper.display_start()..clipper.display_end() {
                    self.table_next_row();

                    ui.align_text_to_frame_padding();
                    if ui.small_button(&im_str!("remove##remove-{}", i)) {
                        remove = Some(i);
                    }
                    ui.same_line();

                    let current_item = inventory.index_mut(i as usize);

                    let width = ui.push_item_width(318.0);
                    self.draw_me1_le_item(i, current_item);
                    width.pop(ui);
                }
            }

            // Remove
            if let Some(i) = remove {
                inventory.remove(i as usize);
            }
        } else {
            self.table_next_row();
            ui.text("Empty");
        }

        // Add
        self.table_next_row();
        if ui.button(im_str!("add")) {
            inventory.push(Item::default());
        }
        Some(())
    }

    fn draw_me1_le_item(&self, ident: i32, current_item: &mut Item) -> Option<()> {
        let ui = self.ui;
        let item_db = Database::me1_item()?;

        // Find name of item
        let current_item_name: &str = item_db
            .get(&DbItem {
                item_id: current_item.item_id,
                manufacturer_id: current_item.manufacturer_id,
            })
            .map(|i| i.as_str())
            .unwrap_or("Unknown item");

        // Item name
        let label = im_str!("##item-name-{}", ident);
        let preview_value = ImString::new(current_item_name);
        let cb = ComboBox::new(&label).preview_value(&preview_value);

        if let Some(_t) = cb.begin(ui) {
            for (k, item_name) in item_db.iter() {
                let text = ImString::new(item_name);
                let selected = item_name == current_item_name;
                if Selectable::new(&text).selected(selected).build(ui) {
                    current_item.item_id = k.item_id;
                    current_item.manufacturer_id = k.manufacturer_id;
                }
            }
        }

        ui.same_line();

        // Item level
        let mut item_level_idx = current_item.item_level.clone() as usize;
        const ITEM_LEVEL_LIST: [&ImStr; 11] = [
            im_str!("None"),
            im_str!("I"),
            im_str!("II"),
            im_str!("III"),
            im_str!("IV"),
            im_str!("V"),
            im_str!("VI"),
            im_str!("VII"),
            im_str!("VIII"),
            im_str!("IX"),
            im_str!("X"),
        ];
        let width = ui.push_item_width(60.0);
        if ComboBox::new(&im_str!("##item-level-{}", ident)).build_simple_string(
            ui,
            &mut item_level_idx,
            &ITEM_LEVEL_LIST,
        ) {
            // Enum
            current_item.item_level = match item_level_idx {
                0 => ItemLevel::None,
                1 => ItemLevel::I,
                2 => ItemLevel::II,
                3 => ItemLevel::III,
                4 => ItemLevel::IV,
                5 => ItemLevel::V,
                6 => ItemLevel::VI,
                7 => ItemLevel::VII,
                8 => ItemLevel::VIII,
                9 => ItemLevel::IX,
                10 => ItemLevel::X,
                _ => unreachable!(),
            };
        }
        width.pop(ui);

        Some(())
    }
}
