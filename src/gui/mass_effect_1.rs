use if_chain::if_chain;
use imgui::{im_str, ChildWindow, ImStr, ImString, ListClipper, TabBar, TabItem};
use std::{
    cell::{RefCell, RefMut},
    cmp::Ordering,
};

use crate::save_data::{
    mass_effect_1::{
        data::{ArrayType, Data, Property, StructType},
        player::{Name, Player},
        plot_db::Me1PlotDb,
        Me1SaveGame,
    },
    shared::plot::{Me1PlotTable, PlotCategory},
    ImguiString, List, RawUi,
};

use super::{DatabasesState, Gui};

impl<'ui> Gui<'ui> {
    pub fn draw_mass_effect_1(
        &self, save_game: &mut Me1SaveGame, databases: &DatabasesState,
    ) -> Option<()> {
        let ui = self.ui;

        // Ajoute un Name dupliqué à la liste
        {
            let duplicate = &mut *save_game.player.duplicate.borrow_mut();
            if let Some(name) = duplicate.take() {
                save_game.player.names.push(RefCell::new(name));
            }
        }

        // Tab bar
        let _t = TabBar::new(im_str!("mass_effect_1")).begin(ui)?;

        // General
        if_chain! {
            if let Some(_t) = TabItem::new(im_str!("General")).begin(ui);
            if let Some(_t) = ChildWindow::new(im_str!("scroll")).begin(ui);
            then {
                self.draw_me1_general(save_game);
            }
        }
        // Plot
        if_chain! {
            if let Some(_t) = TabItem::new(im_str!("Plot")).begin(ui);
            if let Some(me1_plot_db) = &databases.me1_plot_db;
            then {
                self.draw_me1_plot_db(&mut save_game.state.plot, me1_plot_db);
            }
        }
        // Raw
        if_chain! {
            if let Some(_t) = TabItem::new(im_str!("Raw")).begin(ui);
            if let Some(_t) = ChildWindow::new(im_str!("scroll")).begin(ui);
            then {
                // Player
                self.set_next_item_open(true);
                self.draw_raw_player(&save_game.player);
                // State
                self.set_next_item_open(true);
                save_game.state.draw_raw_ui(self, "State");

            }
        }

        Some(())
    }

    fn draw_me1_general(&self, save_game: &mut Me1SaveGame) -> Option<()> {
        let player = &mut save_game.player;
        let plot = &mut save_game.state.plot;

        // Current Game
        let mut current_game = player.objects.iter().enumerate().find_map(|(i, object)| {
            match player.get_name(object.object_name_id).borrow().to_str() {
                "CurrentGame" => Some(player.get_data(i as i32 + 1).borrow_mut()),
                _ => None,
            }
        })?;

        // m_Player
        let mut m_player =
            Self::me1_find_object_property(player, &current_game.properties, "m_Player")?;

        // m_Squad
        let mut m_squad = Self::me1_find_object_property(player, &m_player.properties, "m_Squad")?;

        // m_GameOptions
        let m_game_options =
            Self::me1_find_struct_property(player, &mut current_game.properties, "m_GameOptions")?;

        // m_Inventory
        let mut m_inventory =
            Self::me1_find_object_property(player, &m_squad.properties, "m_Inventory")?;

        // 1ère colonne
        let _t = self.begin_columns(2)?;
        self.table_next_row();

        // Role Play
        if let Some(_t) = self.begin_table(im_str!("role-play-table"), 1) {
            self.table_next_row();
            self.set_next_item_open(true);
            if let Some(_t) = self.push_tree_node("Role-Play") {
                // Name
                if let Some(name) =
                    Self::me1_find_str_property(player, &mut m_player.properties, "m_FirstName")
                {
                    self.table_next_row();
                    name.draw_raw_ui(self, "Name");
                }

                // Gender
                if let Some(gender) =
                    Self::me1_find_name_property(player, &m_player.properties, "m_Gender")
                {
                    self.table_next_row();
                    let text = ImString::new(
                        gender.borrow().to_str().trim_start_matches("BIO_ATTRIBUTE_PAWN_GENDER_"),
                    );
                    self.draw_text(&text, Some(im_str!("Gender")));
                }

                // Origin
                if let Some(origin) =
                    Self::me1_find_name_property(player, &m_player.properties, "m_BackgroundOrigin")
                {
                    self.table_next_row();
                    let text = ImString::new(
                        origin
                            .borrow()
                            .to_str()
                            .trim_start_matches("BIO_PLAYER_CHARACTER_BACKGROUND_ORIGIN_"),
                    );
                    self.draw_text(&text, Some(im_str!("Origin")));
                }

                // Notoriety
                if let Some(notoriety) = Self::me1_find_name_property(
                    player,
                    &m_player.properties,
                    "m_BackgroundNotoriety",
                ) {
                    self.table_next_row();
                    let text = ImString::new(
                        notoriety
                            .borrow()
                            .to_str()
                            .trim_start_matches("BIO_PLAYER_CHARACTER_BACKGROUND_NOTORIETY_"),
                    );
                    self.draw_text(&text, Some(im_str!("Notoriety")));
                }
            }
        }

        // Gameplay
        if let Some(_t) = self.begin_table(im_str!("gameplay-table"), 1) {
            self.table_next_row();
            self.set_next_item_open(true);
            if let Some(_t) = self.push_tree_node("Gameplay") {
                // Class
                if let Some(class) =
                    Self::me1_find_name_property(player, &m_player.properties, "m_ClassBase")
                {
                    self.table_next_row();
                    let text = ImString::new(
                        class.borrow().to_str().trim_start_matches("BIO_PARTY_MEMBER_CLASS_BASE_"),
                    );
                    self.draw_text(&text, Some(im_str!("Class")));
                }

                // Level
                if let Some(level) =
                    Self::me1_find_int_property(player, &mut m_player.properties, "m_XPLevel")
                {
                    self.table_next_row();
                    level.draw_raw_ui(self, "Level");
                }

                // Current XP
                if let Some(current_xp) = Self::me1_find_int_property(
                    player,
                    &mut m_squad.properties,
                    "m_nSquadExperience",
                ) {
                    self.table_next_row();
                    current_xp.draw_raw_ui(self, "Current XP");
                }
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

        // 2ème colonne
        self.table_next_column();

        // General
        if let Some(_t) = self.begin_table(im_str!("general-table"), 1) {
            self.table_next_row();
            self.set_next_item_open(true);
            if let Some(_t) = self.push_tree_node("General") {
                // Difficulty
                if let Some(difficulty) =
                    Self::me1_find_int_property(player, m_game_options, "m_nCombatDifficulty")
                {
                    self.table_next_row();
                    const DIFFICULTY_LIST: [&ImStr; 5] = [
                        im_str!("Casual"),
                        im_str!("Normal"),
                        im_str!("Veteran"),
                        im_str!("Hardcore"),
                        im_str!("Insanity"),
                    ];

                    let mut index = *difficulty as usize;
                    if self.draw_edit_enum("Difficulty", &mut index, &DIFFICULTY_LIST) {
                        *difficulty = index as i32;
                    }
                }

                // New Game +
                if let Some(new_game_plus) = Self::me1_find_bool_property(
                    player,
                    &mut current_game.properties,
                    "m_bSecondPlaythrough",
                ) {
                    self.table_next_row();
                    new_game_plus.draw_raw_ui(self, "New Game +");
                }
            }
        }

        // Resources
        if let Some(_t) = self.begin_table(im_str!("resources-table"), 1) {
            self.table_next_row();
            self.set_next_item_open(true);
            if let Some(_t) = self.push_tree_node("Resources") {
                // Credits
                if let Some(credits) = Self::me1_find_int_property(
                    player,
                    &mut m_inventory.properties,
                    "m_nResourceCredits",
                ) {
                    self.table_next_row();
                    credits.draw_raw_ui(self, "Credits");
                }

                // Medigel
                if let Some(medigel) = Self::me1_find_float_property(
                    player,
                    &mut m_inventory.properties,
                    "m_fResourceMedigel",
                ) {
                    self.table_next_row();
                    medigel.draw_raw_ui(self, "Medigel");
                }

                // Grenades
                if let Some(grenades) = Self::me1_find_int_property(
                    player,
                    &mut m_inventory.properties,
                    "m_nResourceGrenades",
                ) {
                    self.table_next_row();
                    grenades.draw_raw_ui(self, "Grenades");
                }

                // Salvage
                if let Some(omnigel) = Self::me1_find_float_property(
                    player,
                    &mut m_inventory.properties,
                    "m_fResourceSalvage",
                ) {
                    self.table_next_row();
                    omnigel.draw_raw_ui(self, "Omnigel");
                }
            }
        }

        Some(())
    }

    fn me1_find_object_property<'a>(
        player: &'a Player, properties: &[Property], property_name: &str,
    ) -> Option<RefMut<'a, Data>> {
        properties.iter().find_map(|property| match property {
            Property::Object { name_id, object_id, .. }
                if player.get_name(*name_id).borrow().to_str() == property_name =>
            {
                Some(player.get_data(*object_id).borrow_mut())
            }
            _ => None,
        })
    }

    fn me1_find_struct_property<'a>(
        player: &Player, properties: &'a mut [Property], property_name: &str,
    ) -> Option<&'a mut List<Property>> {
        properties.iter_mut().find_map(|property| match property {
            Property::Struct {
                name_id, properties: StructType::Properties(properties), ..
            } if player.get_name(*name_id).borrow().to_str() == property_name => Some(properties),
            _ => None,
        })
    }

    fn me1_find_bool_property<'a>(
        player: &Player, properties: &'a mut [Property], property_name: &str,
    ) -> Option<&'a mut bool> {
        properties.iter_mut().find_map(|property| match property {
            Property::Bool { name_id, value, .. }
                if player.get_name(*name_id).borrow().to_str() == property_name =>
            {
                Some(value)
            }
            _ => None,
        })
    }

    fn me1_find_int_property<'a>(
        player: &Player, properties: &'a mut [Property], property_name: &str,
    ) -> Option<&'a mut i32> {
        properties.iter_mut().find_map(|property| match property {
            Property::Int { name_id, value, .. }
                if player.get_name(*name_id).borrow().to_str() == property_name =>
            {
                Some(value)
            }
            _ => None,
        })
    }

    fn me1_find_float_property<'a>(
        player: &Player, properties: &'a mut [Property], property_name: &str,
    ) -> Option<&'a mut f32> {
        properties.iter_mut().find_map(|property| match property {
            Property::Float { name_id, value, .. }
                if player.get_name(*name_id).borrow().to_str() == property_name =>
            {
                Some(value)
            }
            _ => None,
        })
    }

    fn me1_find_str_property<'a>(
        player: &Player, properties: &'a mut [Property], property_name: &str,
    ) -> Option<&'a mut ImguiString> {
        properties.iter_mut().find_map(|property| match property {
            Property::Str { name_id, string, .. }
                if player.get_name(*name_id).borrow().to_str() == property_name =>
            {
                Some(string)
            }
            _ => None,
        })
    }

    fn me1_find_name_property<'a>(
        player: &'a Player, properties: &[Property], property_name: &str,
    ) -> Option<&'a RefCell<Name>> {
        properties.iter().find_map(|property| match property {
            Property::Name { name_id, value_name_id, .. }
                if player.get_name(*name_id).borrow().to_str() == property_name =>
            {
                Some(player.get_name(*value_name_id))
            }
            _ => None,
        })
    }

    pub fn draw_me1_plot_db(
        &self, me1_plot_table: &mut Me1PlotTable, me1_plot_db: &Me1PlotDb,
    ) -> Option<()> {
        let ui = self.ui;
        let Me1PlotDb { player_crew, missions } = me1_plot_db;

        // Tab bar
        let _t = TabBar::new(im_str!("plot-tab")).begin(ui)?;

        let categories = [(im_str!("Player / Crew"), player_crew), (im_str!("Missions"), missions)];

        for (title, plot_map) in &categories {
            if_chain! {
                if let Some(_t) = TabItem::new(title).begin(ui);
                if let Some(_t) = ChildWindow::new(im_str!("scroll")).begin(ui);
                then {
                    for (category_name, plot_db) in plot_map.iter() {
                        if let Some(_t) = self.begin_table(&im_str!("{}-table", category_name), 1) {
                            self.table_next_row();
                            if let Some(_t) = self.push_tree_node(category_name) {
                                self.draw_me1_plot_category(me1_plot_table, plot_db);
                            }
                        }
                    }
                }
            }
        }

        Some(())
    }

    fn draw_me1_plot_category(&self, plot_table: &mut Me1PlotTable, plot_db: &PlotCategory) {
        let ui = self.ui;
        let PlotCategory { booleans, ints } = plot_db;

        if booleans.is_empty() && ints.is_empty() {
            return;
        }

        // Booleans
        let mut clipper = ListClipper::new(booleans.len() as i32).begin(ui);
        while clipper.step() {
            for i in clipper.display_start()..clipper.display_end() {
                let (plot_id, plot_desc) = booleans.get_index(i as usize).unwrap();
                let plot = plot_table.bool_variables.get_mut(*plot_id);
                if let Some(mut plot) = plot {
                    self.table_next_row();
                    plot.draw_raw_ui(self, &format!("{}##bool-{}", plot_desc, plot_desc));
                }
            }
        }

        // Integers
        let mut clipper = ListClipper::new(ints.len() as i32).begin(ui);
        while clipper.step() {
            for i in clipper.display_start()..clipper.display_end() {
                let (plot_id, plot_desc) = ints.get_index(i as usize).unwrap();
                let plot = plot_table.int_variables.get_mut(*plot_id);
                if let Some(plot) = plot {
                    self.table_next_row();
                    plot.draw_raw_ui(self, &format!("{}##int-{}", plot_desc, plot_desc));
                }
            }
        }
    }

    fn draw_raw_player(&self, player: &Player) -> Option<()> {
        let (i, _) = player.objects.iter().enumerate().find(|(_, object)| {
            player.get_name(object.object_name_id).borrow().to_str() == "CurrentGame"
        })?;

        let object_id = i as i32 + 1;
        self.draw_object(player, i, None, object_id);
        Some(())
    }

    fn draw_object(
        &self, player: &Player, ident: usize, property_name: Option<&ImStr>, object_id: i32,
    ) {
        let object = player.get_object(object_id);
        let object_name: &ImStr = &*player.get_name(object.object_name_id).borrow();

        let property_name = match property_name {
            Option::Some(property_name) => im_str!("{} : {}", object_name, property_name),
            Option::None => object_name.to_owned(),
        };

        if_chain! {
            if let Some(_t) = self.push_tree_node(&format!("{}##{}", property_name, ident));
            if let Some(_t) = self.begin_table(im_str!("object-table"), 1);
            then {
                let mut data = player.get_data(object_id).borrow_mut();
                for (i, property) in data.iter_mut().enumerate() {
                    self.draw_property(player, i, property);
                }
            }
        }
    }

    fn draw_property(&self, player: &Player, ident: usize, property: &mut Property) -> Option<()> {
        match property {
            Property::Byte { .. } | Property::None { .. } => return None,
            _ => {
                self.table_next_row();
            }
        }

        match property {
            Property::Array { name_id, array, .. } => {
                let name: &ImStr = &*player.get_name(*name_id).borrow();
                self.draw_array_property(player, &format!("{}##{}", name, ident), array)?;
            }
            Property::Bool { name_id, value, .. } => {
                let name: &ImStr = &*player.get_name(*name_id).borrow();
                self.draw_edit_bool(im_str!("{}##bool-{}", name, ident).to_str(), value)
            }
            Property::Float { name_id, value, .. } => {
                let name: &ImStr = &*player.get_name(*name_id).borrow();
                self.draw_edit_f32(im_str!("{}##f32-{}", name, ident).to_str(), value)
            }
            Property::Int { name_id, value, .. } => {
                let name: &ImStr = &*player.get_name(*name_id).borrow();
                self.draw_edit_i32(im_str!("{}##i32-{}", name, ident).to_str(), value)
            }
            Property::Name { name_id, value_name_id, .. } => {
                self.draw_name_property(player, ident, name_id, value_name_id);
            }
            Property::Object { name_id, object_id, .. } => {
                match (*object_id).cmp(&0) {
                    Ordering::Greater => {
                        // Object
                        let property_name: &ImStr = &*player.get_name(*name_id).borrow();
                        self.draw_object(player, ident, Some(property_name), *object_id);
                    }
                    Ordering::Less => {
                        // Class
                        let class = player.get_class(*object_id);
                        let class_name = &*player.get_name(class.class_name_id).borrow();
                        let label = &*player.get_name(*name_id).borrow();
                        self.draw_text(class_name, Some(label));
                    }
                    Ordering::Equal => {
                        // Null => Nom de classe par défaut
                        let label = &*player.get_name(*name_id).borrow();
                        self.draw_text(im_str!("Class"), Some(label))
                    }
                }
            }
            Property::Str { name_id, string, .. } => {
                let name: &ImStr = &*player.get_name(*name_id).borrow();
                self.draw_edit_string(im_str!("{}##string-{}", name, ident).to_str(), string)
            }
            Property::StringRef { name_id, value, .. } => {
                let name: &ImStr = &*player.get_name(*name_id).borrow();
                self.draw_edit_i32(im_str!("{}##string-ref-{}", name, ident).to_str(), value)
            }
            Property::Struct { name_id, struct_name_id, properties, .. } => {
                let name: &ImStr = &*player.get_name(*name_id).borrow();
                let struct_name: &ImStr = &*player.get_name(*struct_name_id).borrow();
                self.draw_struct_property(
                    player,
                    ident,
                    &im_str!("{} : {}", struct_name, name),
                    properties,
                )
            }
            Property::Byte { .. } | Property::None { .. } => unreachable!(),
        }
        Some(())
    }

    fn draw_name_property(
        &self, player: &Player, ident: usize, name_id: &u32, value_name_id: &mut u32,
    ) {
        let ui = self.ui;

        let is_duplicate = player.get_name(*value_name_id).borrow().is_duplicate;
        if !is_duplicate {
            let label = &*player.get_name(*name_id).borrow();
            let value = &*player.get_name(*value_name_id).borrow();

            self.draw_text(value, Some(label));
            ui.same_line();
            if ui.small_button(&im_str!("duplicate##dupe-{}", ident)) {
                // Duplicate name à la prochaine frame
                let mut new_value = value.clone();
                new_value.is_duplicate = true;
                *player.duplicate.borrow_mut() = Some(new_value);
                *value_name_id = player.names.len() as u32;
            }
            ui.same_line();
            self.draw_help_marker("In order to modify this string you have to duplicate it first.");
        } else {
            let ident = &*player.get_name(*name_id).borrow();
            let value = &mut *player.get_name(*value_name_id).borrow_mut();

            self.draw_edit_string(ident.to_str(), value);
        }
    }

    fn draw_array_property(
        &self, player: &Player, ident: &str, array: &mut [ArrayType],
    ) -> Option<()> {
        let ui = self.ui;

        // Tree node
        let _t = self.push_tree_node(ident)?;

        // Table
        let _t = self.begin_table(im_str!("array-table"), 1)?;

        if array.is_empty() {
            self.table_next_row();
            ui.text("Empty");
            return None;
        }

        for (i, property) in array.iter_mut().enumerate() {
            self.table_next_row();
            match property {
                ArrayType::Int(int) => int.draw_raw_ui(self, im_str!("{}##int-{}", i, i).to_str()),
                ArrayType::Object(object_id) => {
                    if *object_id != 0 {
                        // Object
                        self.draw_object(player, i, None, *object_id);
                    } else {
                        // Null
                        self.draw_text(im_str!("Null"), None);
                    }
                }
                ArrayType::Vector(vector) => {
                    vector.draw_raw_ui(self, im_str!("{}##vector-{}", i, i).to_str())
                }
                ArrayType::String(string) => {
                    string.draw_raw_ui(self, im_str!("##string-{}", i).to_str())
                }
                ArrayType::Properties(properties) => {
                    if_chain! {
                        if let Some(_t) = self.push_tree_node(&i.to_string());
                        if let Some(_t) = self.begin_table(im_str!("array-properties-table"), 1);
                        then {
                            for (j, property) in properties.iter_mut().enumerate() {
                                self.draw_property(player, j, property);
                            }
                        }
                    }
                }
            }
        }
        Some(())
    }

    fn draw_struct_property(
        &self, player: &Player, ident: usize, label: &ImStr, struct_property: &mut StructType,
    ) {
        match struct_property {
            StructType::LinearColor(color) => {
                color.draw_raw_ui(self, &format!("{}##linear-color-{}", label, ident))
            }
            StructType::Vector(vector) => {
                vector.draw_raw_ui(self, &format!("{}##vector-{}", label, ident))
            }
            StructType::Rotator(rotator) => {
                rotator.draw_raw_ui(self, &format!("{}##rotator-{}", label, ident))
            }
            StructType::Properties(properties) => {
                if_chain! {
                    if let Some(_t) = self.push_tree_node(&format!("{}##{}", label, ident));
                    if let Some(_t) = self.begin_table(im_str!("struct-properties-table"), 1);
                    then {
                        for (i, property) in properties.iter_mut().enumerate() {
                            self.draw_property(player, i, property);
                        }
                    }
                }
            }
        }
    }

    fn draw_text(&self, text: &ImStr, label: Option<&ImStr>) {
        let ui = self.ui;

        if let Some(label) = label {
            ui.label_text(label, text)
        } else {
            ui.text(text);
        }
    }
}
