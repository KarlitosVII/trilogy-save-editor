use if_chain::if_chain;
use imgui::{
    im_str, ChildWindow, ComboBox, ImStr, ImString, ListClipper, Selectable, TabBar, TabItem,
};

use crate::{
    event_handler::MainEvent,
    save_data::{
        mass_effect_2::{
            player::{Player, Power},
            plot::PlotTable,
            plot_db::Me2PlotDb,
            Me2LeSaveGame, Me2SaveGame,
        },
        shared::{
            appearance::{HasHeadMorph, HeadMorph},
            player::{Notoriety, Origin},
            plot::{Me1PlotTable, PlotCategory},
        },
        RawUi,
    },
};

use super::{DatabasesState, Gui, Theme};

enum Me2Type<'a> {
    Vanilla(&'a mut Me2SaveGame),
    Legendary(&'a mut Me2LeSaveGame),
}

impl<'ui> Gui<'ui> {
    pub fn draw_mass_effect_2(
        &self, save_game: &mut Me2SaveGame, databases: &DatabasesState,
    ) -> Option<()> {
        let ui = self.ui;

        // Tab bar
        let _t = TabBar::new(im_str!("mass_effect_2")).begin(ui)?;

        // General
        if_chain! {
            if let Some(_t) = TabItem::new(im_str!("General")).begin(ui);
            if let Some(_t) = ChildWindow::new(im_str!("scroll")).begin(ui);
            then {
                self.draw_me2_general(Me2Type::Vanilla(save_game));
            }
        }
        // Plot
        if_chain! {
            if let Some(_t) = TabItem::new(im_str!("Plot")).begin(ui);
            if let Some(_t) = TabBar::new(im_str!("plot-tab")).begin(ui);
            then {
                self.draw_me2_plot_db(&mut save_game.plot, &mut save_game.me1_plot, &databases);
            }
        }
        // Head Morph
        if_chain! {
            if let Some(_t) = TabItem::new(im_str!("Head Morph")).begin(ui);
            if let Some(_t) = ChildWindow::new(im_str!("scroll")).begin(ui);
            then {
                self.draw_me2_head_morph(&mut save_game.player.appearance.head_morph, save_game.player.is_female);
            }
        }
        // Raw
        if_chain! {
            if let Some(_t) = TabItem::new(im_str!("Raw")).begin(ui);
            if let Some(_t) = ChildWindow::new(im_str!("scroll")).begin(ui);
            then {
                self.set_next_item_open(true);
                save_game.draw_raw_ui(self, "Mass Effect 2");
            }
        }
        Some(())
    }

    pub fn draw_mass_effect_2_le(
        &self, save_game: &mut Me2LeSaveGame, databases: &DatabasesState,
    ) -> Option<()> {
        let ui = self.ui;

        // Tab bar
        let _t = TabBar::new(im_str!("mass_effect_2")).begin(ui)?;

        // General
        if_chain! {
            if let Some(_t) = TabItem::new(im_str!("General")).begin(ui);
            if let Some(_t) = ChildWindow::new(im_str!("scroll")).begin(ui);
            then {
                self.draw_me2_general(Me2Type::Legendary(save_game));
            }
        }
        // Plot
        if_chain! {
            if let Some(_t) = TabItem::new(im_str!("Plot")).begin(ui);
            if let Some(_t) = TabBar::new(im_str!("plot-tab")).begin(ui);
            then {
                self.draw_me2_plot_db(&mut save_game.plot, &mut save_game.me1_plot, &databases);
            }
        }
        // Head Morph
        if_chain! {
            if let Some(_t) = TabItem::new(im_str!("Head Morph")).begin(ui);
            if let Some(_t) = ChildWindow::new(im_str!("scroll")).begin(ui);
            then {
                self.draw_me3_and_le_head_morph(&mut save_game.player.appearance.head_morph);
            }
        }
        // Raw
        if_chain! {
            if let Some(_t) = TabItem::new(im_str!("Raw")).begin(ui);
            if let Some(_t) = ChildWindow::new(im_str!("scroll")).begin(ui);
            then {
                self.set_next_item_open(true);
                save_game.draw_raw_ui(self, "Mass Effect 2");
            }
        }
        Some(())
    }

    fn draw_me2_general(&self, save_game: Me2Type) -> Option<()> {
        let ui = self.ui;

        match save_game {
            Me2Type::Vanilla(Me2SaveGame {
                difficulty,
                end_game_state,
                player,
                plot,
                me1_plot,
                ..
            })
            | Me2Type::Legendary(Me2LeSaveGame {
                difficulty,
                end_game_state,
                player,
                plot,
                me1_plot,
                ..
            }) => {
                let Player {
                    is_female,
                    class_name,
                    level,
                    current_xp,
                    first_name,
                    origin,
                    notoriety,
                    talent_points,
                    powers,
                    credits,
                    medigel,
                    eezo,
                    iridium,
                    palladium,
                    platinum,
                    probes,
                    current_fuel,
                    face_code,
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
                                Otherwise, the Collectors will be the least of your worries..."
                            );
                        }

                        self.table_next_row();
                        let mut origin_idx = origin.clone() as usize;
                        const ORIGIN_LIST: [&ImStr; 4] = [
                            im_str!("None"),
                            im_str!("Spacer"),
                            im_str!("Colonist"),
                            im_str!("Earthborn"),
                        ];

                        if self.draw_edit_enum("Origin", &mut origin_idx, &ORIGIN_LIST) {
                            // Enum
                            *origin = match origin_idx {
                                0 => Origin::None,
                                1 => Origin::Spacer,
                                2 => Origin::Colonist,
                                3 => Origin::Earthborn,
                                _ => unreachable!(),
                            };

                            // ME1 imported
                            match origin {
                                Origin::None => {}
                                Origin::Spacer => {
                                    if let Some(mut spacer) = plot.bool_variables.get_mut(1533) {
                                        *spacer = true;
                                    }
                                    if let Some(mut colonist) = plot.bool_variables.get_mut(1535) {
                                        *colonist = false;
                                    }
                                    if let Some(mut eathborn) = plot.bool_variables.get_mut(1534) {
                                        *eathborn = false;
                                    }
                                }
                                Origin::Colonist => {
                                    if let Some(mut spacer) = plot.bool_variables.get_mut(1533) {
                                        *spacer = false;
                                    }
                                    if let Some(mut colonist) = plot.bool_variables.get_mut(1535) {
                                        *colonist = true;
                                    }
                                    if let Some(mut eathborn) = plot.bool_variables.get_mut(1534) {
                                        *eathborn = false;
                                    }
                                }
                                Origin::Earthborn => {
                                    if let Some(mut spacer) = plot.bool_variables.get_mut(1533) {
                                        *spacer = false;
                                    }
                                    if let Some(mut colonist) = plot.bool_variables.get_mut(1535) {
                                        *colonist = false;
                                    }
                                    if let Some(mut eathborn) = plot.bool_variables.get_mut(1534) {
                                        *eathborn = true;
                                    }
                                }
                            }

                            // ME1 plot
                            if let Some(me1_origin) = me1_plot.int_variables.get_mut(1) {
                                *me1_origin = origin_idx as i32;
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

                        if self.draw_edit_enum("Notoriety", &mut notoriety_idx, &NOTORIETY_LIST) {
                            // Enum
                            *notoriety = match notoriety_idx {
                                0 => Notoriety::None,
                                1 => Notoriety::Survivor,
                                2 => Notoriety::Warhero,
                                3 => Notoriety::Ruthless,
                                _ => unreachable!(),
                            };

                            // ME1 imported
                            match notoriety {
                                Notoriety::None => {}
                                Notoriety::Survivor => {
                                    if let Some(mut survivor) = plot.bool_variables.get_mut(1537) {
                                        *survivor = true;
                                    }
                                    if let Some(mut war_hero) = plot.bool_variables.get_mut(1538) {
                                        *war_hero = false;
                                    }
                                    if let Some(mut ruthless) = plot.bool_variables.get_mut(1539) {
                                        *ruthless = false;
                                    }
                                }
                                Notoriety::Warhero => {
                                    if let Some(mut survivor) = plot.bool_variables.get_mut(1537) {
                                        *survivor = false;
                                    }
                                    if let Some(mut war_hero) = plot.bool_variables.get_mut(1538) {
                                        *war_hero = true;
                                    }
                                    if let Some(mut ruthless) = plot.bool_variables.get_mut(1539) {
                                        *ruthless = false;
                                    }
                                }
                                Notoriety::Ruthless => {
                                    if let Some(mut survivor) = plot.bool_variables.get_mut(1537) {
                                        *survivor = false;
                                    }
                                    if let Some(mut war_hero) = plot.bool_variables.get_mut(1538) {
                                        *war_hero = false;
                                    }
                                    if let Some(mut ruthless) = plot.bool_variables.get_mut(1539) {
                                        *ruthless = true;
                                    }
                                }
                            }

                            // ME1 plot
                            if let Some(me1_notoriety) = me1_plot.int_variables.get_mut(2) {
                                *me1_notoriety = notoriety_idx as i32;
                            }
                        }

                        self.table_next_row();
                        face_code.draw_raw_ui(self, "Identity Code");
                        ui.same_line();
                        self.draw_help_marker("If you change this you can display whatever you want in the menus\nin place of your `Identity Code`, which is pretty cool !");
                    }
                }

                // Morality
                if let Some(_t) = self.begin_table(im_str!("morality-table"), 1) {
                    self.table_next_row();
                    self.set_next_item_open(true);
                    if let Some(_t) = self.push_tree_node("Morality") {
                        if let Some(paragon) = plot.int_variables.get_mut(2) {
                            self.table_next_row();
                            paragon.draw_raw_ui(self, "Paragon");
                        }

                        if let Some(renegade) = plot.int_variables.get_mut(3) {
                            self.table_next_row();
                            renegade.draw_raw_ui(self, "Renegade");
                        }
                    }
                }

                // Gameplay
                if let Some(_t) = self.begin_table(im_str!("gameplay-table"), 1) {
                    self.table_next_row();
                    self.set_next_item_open(true);
                    if let Some(_t) = self.push_tree_node("Gameplay") {
                        self.table_next_row();
                        self.draw_me2_class(class_name);

                        self.table_next_row();
                        level.draw_raw_ui(self, "Level");

                        self.table_next_row();
                        current_xp.draw_raw_ui(self, "Current XP");

                        self.table_next_row();
                        talent_points.draw_raw_ui(self, "Talent Points");

                        self.table_next_row();
                        credits.draw_raw_ui(self, "Credits");

                        self.table_next_row();
                        medigel.draw_raw_ui(self, "Medi-gel");
                    }
                }

                // Resources
                if let Some(_t) = self.begin_table(im_str!("resources-table"), 1) {
                    self.table_next_row();
                    self.set_next_item_open(true);
                    if let Some(_t) = self.push_tree_node("Resources") {
                        self.table_next_row();
                        eezo.draw_raw_ui(self, "Eezo");

                        self.table_next_row();
                        iridium.draw_raw_ui(self, "Iridium");

                        self.table_next_row();
                        palladium.draw_raw_ui(self, "Palladium");

                        self.table_next_row();
                        platinum.draw_raw_ui(self, "Platinum");

                        self.table_next_row();
                        probes.draw_raw_ui(self, "Probes");

                        self.table_next_row();
                        current_fuel.draw_raw_ui(self, "Current Fuel");
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
                        self.table_next_row();
                        end_game_state.draw_raw_ui(self, "End Game State");
                    }
                }

                // Bonus Powers
                self.set_next_item_open(true);
                self.draw_me2_bonus_powers(powers)
            }
        }
    }

    fn draw_me2_class(&self, class_name: &mut ImString) {
        let ui = self.ui;
        const CLASS_LIST: [(&ImStr, &ImStr); 6] = [
            (im_str!("SFXGame.SFXPawn_PlayerAdept"), im_str!("Adept")),
            (im_str!("SFXGame.SFXPawn_PlayerEngineer"), im_str!("Engineer")),
            (im_str!("SFXGame.SFXPawn_PlayerInfiltrator"), im_str!("Infiltrator")),
            (im_str!("SFXGame.SFXPawn_PlayerSentinel"), im_str!("Sentinel")),
            (im_str!("SFXGame.SFXPawn_PlayerSoldier"), im_str!("Soldier")),
            (im_str!("SFXGame.SFXPawn_PlayerVanguard"), im_str!("Vanguard")),
        ];

        if let Some(mut class_id) = CLASS_LIST.iter().enumerate().find_map(|(i, &name)| {
            if unicase::eq(name.0, class_name) {
                Some(i)
            } else {
                None
            }
        }) {
            let width = ui.push_item_width(200.0);
            if ComboBox::new(im_str!("Class")).build_simple(
                self.ui,
                &mut class_id,
                &CLASS_LIST,
                &|&(_, name)| name.into(),
            ) {
                *class_name = CLASS_LIST[class_id].0.to_owned();
            }
            width.pop(ui);
        }
    }

    fn draw_me2_bonus_powers(&self, powers: &mut Vec<Power>) -> Option<()> {
        let ui = self.ui;

        // Table
        let _t = self.begin_table(im_str!("gameplay-table"), 1)?;

        // Tree node
        self.table_next_row();
        let _t = self.push_tree_node("Bonus Powers")?;
        ui.same_line();
        self.draw_help_marker(
            "You can use as many bonus powers as you want and customize your\n\
            build to your liking.\n\
            The only restriction is the size of your screen !\n\
            If you want to remove a bonus power you need to reset your\n\
            talents `before` or you will lose some talent points.\n\
            Unlike Mass Effect 3, the game will never recalculate your points.\n\
            At level 30 you have `51` points to spend.",
        );

        const POWER_LIST: [(&ImStr, &ImStr); 14] = [
            (im_str!("SFXGameContent_Powers.SFXPower_Crush_Player"), im_str!("Slam")),
            (im_str!("SFXGameContent_Powers.SFXPower_Barrier_Player"), im_str!("Barrier")),
            (im_str!("SFXGameContent_Powers.SFXPower_WarpAmmo_Player"), im_str!("Warp Ammo")),
            (
                im_str!("SFXGameContent_Powers.SFXPower_Fortification_Player"),
                im_str!("Fortification"),
            ),
            (
                im_str!("SFXGameContent_Powers.SFXPower_ArmorPiercingAmmo_Player"),
                im_str!("Armor Piercing Ammo"),
            ),
            (im_str!("SFXGameContent_Powers.SFXPower_NeuralShock_Player"), im_str!("Neural Shock")),
            (im_str!("SFXGameContent_Powers.SFXPower_ShieldJack_Player"), im_str!("Energy Drain")),
            (im_str!("SFXGameContent_Powers.SFXPower_Reave_Player"), im_str!("Reave")),
            (im_str!("SFXGameContent_Powers.SFXPower_Dominate_Player"), im_str!("Dominate")),
            (
                im_str!("SFXGameContent_Powers.SFXPower_AntiOrganicAmmo_Player"),
                im_str!("Shredder Ammo"),
            ),
            (
                im_str!("SFXGameContent_Powers.SFXPower_GethShieldBoost_Player"),
                im_str!("Geth Shield Boost"),
            ),
            (
                im_str!("SFXGameContentDLC_HEN_VT.SFXPower_ZaeedUnique_Player"),
                im_str!("Inferno Grenade"),
            ),
            (
                im_str!("SFXGameContentKasumi.SFXPower_KasumiUnique_Player"),
                im_str!("Flashbang Grenade"),
            ),
            (im_str!("SFXGameContentLiara.SFXPower_StasisNew"), im_str!("Stasis")),
        ];

        for &(power_class_name, power_name) in &POWER_LIST {
            let mut selected = powers
                .iter()
                .any(|power| unicase::eq(power.power_class_name.as_ref(), power_class_name));

            self.table_next_row();
            ui.align_text_to_frame_padding();
            if Selectable::new(power_name).build_with_ref(ui, &mut selected) {
                if selected {
                    let mut power = Power::default();
                    power.power_class_name = power_class_name.to_owned().into();
                    powers.push(power);
                } else if let Some((i, _)) = powers.iter().enumerate().find(|(_, power)| {
                    unicase::eq(power.power_class_name.as_ref(), power_class_name)
                }) {
                    powers.remove(i);
                }
            }
        }
        Some(())
    }

    fn draw_me2_plot_db(
        &self, me2_plot_table: &mut PlotTable, me1_plot_table: &mut Me1PlotTable,
        databases: &DatabasesState,
    ) -> Option<()> {
        let ui = self.ui;
        let me2_plot_db = databases.me2_plot_db.as_ref()?;

        let Me2PlotDb {
            player,
            crew,
            romance,
            missions,
            loyalty_missions,
            research_upgrades,
            rewards,
            captains_cabin,
            imported_me1,
        } = me2_plot_db;

        // Player
        if_chain! {
            if let Some(_t) = TabItem::new(im_str!("Player")).begin(ui);
            if let Some(_t) = ChildWindow::new(im_str!("scroll")).begin(ui);
            if let Some(_t) = self.begin_table(im_str!("plot-table"), 1);
            then {
                self.draw_me2_plot_category(me2_plot_table, player);
            }
        }

        let categories = [
            (im_str!("Crew"), crew),
            (im_str!("Romance"), romance),
            (im_str!("Missions"), missions),
            (im_str!("Loyalty missions"), loyalty_missions),
            (im_str!("Research / Upgrades"), research_upgrades),
        ];

        for (title, plot_map) in &categories {
            if_chain! {
                if let Some(_t) = TabItem::new(title).begin(ui);
                if let Some(_t) = ChildWindow::new(im_str!("scroll")).begin(ui);
                then {
                    for (category_name, plot_db) in plot_map.iter() {
                        if let Some(_t) = self.begin_table(&im_str!("{}-table", category_name), 1) {
                            self.table_next_row();
                            if let Some(_t) = self.push_tree_node(category_name) {
                                self.draw_me2_plot_category(me2_plot_table, plot_db);
                            }
                        }
                    }
                }
            }
        }

        // Rewards
        if_chain! {
            if let Some(_t) = TabItem::new(im_str!("Rewards")).begin(ui);
            if let Some(_t) = ChildWindow::new(im_str!("scroll")).begin(ui);
            if let Some(_t) = self.begin_table(im_str!("plot-table"), 1);
            then {
                self.draw_me2_plot_category(me2_plot_table, rewards);
            }
        }
        // Captain's cabin
        if_chain! {
            if let Some(_t) = TabItem::new(im_str!("Captain's cabin")).begin(ui);
            if let Some(_t) = ChildWindow::new(im_str!("scroll")).begin(ui);
            if let Some(_t) = self.begin_table(im_str!("plot-table"), 1);
            then {
                self.draw_me2_plot_category(me2_plot_table, captains_cabin);
            }
        }

        // Mass Effect 1
        {
            let _colors = self.style_colors(Theme::MassEffect1);
            if_chain! {
                if let Some(_t) = TabItem::new(im_str!("Imported ME1")).begin(ui);
                if let Some(_t) = ChildWindow::new(im_str!("scroll")).begin(ui);
                then {
                    ui.text("For proper ME3 import change the same plot flags in `Mass Effect 1` tab. Conrad Verner bugfix :");
                    ui.same_line();
                    self.draw_help_marker(
                        "- Untick `[The Fan] Intimidated him`\n\
                         - Tick `[The Fan] Met Conrad Verner` and `[The Fan] Charmed him`\n\
                         - Only works if you didn't talk to Aethyta"
                    );
                    ui.separator();
                    for (category_name, plot_db) in imported_me1.iter() {
                        if let Some(_t) = self.begin_table(&im_str!("{}-table", category_name), 1) {
                            self.table_next_row();
                            if let Some(_t) = self.push_tree_node(category_name) {
                                self.draw_me2_plot_category(me2_plot_table, plot_db);
                            }
                        }
                    }
                }
            }

            if_chain! {
                if let Some(_t) = TabItem::new(im_str!("Mass Effect 1")).begin(ui);
                if let Some(me1_plot_db) = &databases.me1_plot_db;
                then {
                    if me1_plot_table.bool_variables.is_empty() {
                        ui.text("You cannot edit ME1 plot if you have not imported a ME1 save.");
                    } else {
                        ui.text("If you change these plots this will ONLY take effect after an import.");
                        ui.same_line();
                        self.draw_help_marker("You have to change your end game state to `LiveToFightAgain` then import it to start a new game.\nOr you can start a New Game +.");
                        ui.separator();
                        self.draw_me1_plot_db(me1_plot_table, me1_plot_db);
                    }
                }
            }
        }
        Some(())
    }

    fn draw_me2_plot_category(&self, plot_table: &mut PlotTable, plot_db: &PlotCategory) {
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

    fn draw_me2_head_morph(&self, head_morph: &mut HasHeadMorph, is_female: bool) {
        let ui = self.ui;
        let HasHeadMorph { has_head_morph, head_morph } = head_morph;

        // Import
        if ui.button(im_str!("Import")) {
            let file =
                tinyfiledialogs::open_file_dialog("", "", Some((&["*.ron"], "Head Morph (*.ron)")));

            if let Some(path) = file {
                let _ = self.event_addr.send(MainEvent::ImportHeadMorph(path));
            }
        }
        match head_morph {
            Some(head_morph) => {
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
                // Toggle head morph
                if !is_female {
                    ui.same_line();
                    has_head_morph.draw_raw_ui(self, "Enable head morph");
                }
                ui.separator();

                // Raw
                if *has_head_morph {
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

                    if let Some(_t) = self.begin_table(im_str!("plot-table"), 1) {
                        self.table_next_row();
                        self.set_next_item_open(true);
                        if let Some(_t) = self.push_tree_node("Raw") {
                            self.table_next_row();
                            hair_mesh.draw_raw_ui(self, "Hair Mesh");
                            self.table_next_row();
                            accessory_mesh.draw_raw_ui(self, "Accessory Mesh");
                            self.table_next_row();
                            morph_features.draw_raw_ui(self, "Morph Features");
                            self.table_next_row();
                            offset_bones.draw_raw_ui(self, "Offset Bones");
                            self.table_next_row();
                            lod0_vertices.draw_raw_ui(self, "Lod0 Vertices");
                            self.table_next_row();
                            lod1_vertices.draw_raw_ui(self, "Lod1 Vertices");
                            self.table_next_row();
                            lod2_vertices.draw_raw_ui(self, "Lod2 Vertices");
                            self.table_next_row();
                            lod3_vertices.draw_raw_ui(self, "Lod3 Vertices");
                            self.table_next_row();
                            scalar_parameters.draw_raw_ui(self, "Scalar Parameters");
                            self.table_next_row();
                            vector_parameters.draw_raw_ui(self, "Vector Parameters");
                            self.table_next_row();
                            texture_parameters.draw_raw_ui(self, "Texture Parameters");
                        }
                    }
                }
            }
            None => ui.separator(),
        }
    }
}
