use if_chain::if_chain;
use imgui::*;

use crate::save_data::{
    common::plot::PlotCategory,
    mass_effect_2::{
        player::{Player, Power},
        plot::PlotTable,
        Me2SaveGame,
    },
};

use super::*;

impl<'ui> Gui<'ui> {
    pub fn draw_mass_effect_2(&self, save_game: &mut Me2SaveGame, known_plots: &KnownPlotsState) {
        let ui = self.ui;

        // Tab bar
        let _t = match TabBar::new(im_str!("mass_effect_2")).begin(ui) {
            Some(t) => t,
            None => return,
        };

        // General
        if_chain! {
            if let Some(_t) = TabItem::new(im_str!("General")).begin(ui);
            if let Some(_t) = ChildWindow::new(im_str!("scroll")).begin(ui);
            then {
                self.draw_me2_general(save_game);
            }
        }
        // Plot
        if_chain! {
            if let Some(_t) = TabItem::new(im_str!("Plot")).begin(ui);
            if let Some(_t) = TabBar::new(im_str!("plot-tab")).begin(ui);
            then {
                // Mass Effect 2
                self.draw_me2_known_plot(&mut save_game.plot, &known_plots);
                // Mass Effect 1
                {
                    let _colors = self.style_colors(Theme::MassEffect1);
                    if_chain! {
                        if let Some(_t) = TabItem::new(im_str!("Mass Effect 1")).begin(ui);
                        if let Some(me1_known_plot) = &known_plots.me1;
                        then {
                            self.draw_me1_known_plot(&mut save_game.me1_plot, &me1_known_plot);
                        }
                    }
                }
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
    }

    fn draw_me2_general(&self, save_game: &mut Me2SaveGame) {
        let ui = self.ui;
        let Me2SaveGame { difficulty, end_game_state, player, plot, .. } = save_game;
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
        let _t = match self.begin_columns(2) {
            Some(t) => t,
            None => return,
        };
        self.table_next_row();

        // Role Play
        if let Some(_t) = self.begin_table(im_str!("role-play-table"), 1) {
            self.table_next_row();
            self.set_next_item_open(true);
            if let Some(_t) = self.push_tree_node("Role-Play") {
                self.table_next_row();
                ui.input_text(im_str!("Name"), first_name).resize_buffer(true).build();

                // Gender
                self.table_next_row();
                let mut gender = *is_female as usize;
                const GENDER_LIST: [&ImStr; 2] = [im_str!("Male"), im_str!("Female")];
                {
                    let width = ui.push_item_width(200.0);
                    if ComboBox::new(im_str!("Gender")).build_simple_string(
                        ui,
                        &mut gender,
                        &GENDER_LIST,
                    ) {
                        *is_female = gender != 0;
                    }
                    width.pop(ui);

                    ui.same_line();
                    self.draw_help_marker("If you change your gender, disable head morph or import one appropriate\nor the Collectors will be the least of your worries...");
                }

                self.table_next_row();
                origin.draw_raw_ui(self, "Origin");

                self.table_next_row();
                notoriety.draw_raw_ui(self, "Notoriety");

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

        // Ressources
        if let Some(_t) = self.begin_table(im_str!("ressources-table"), 1) {
            self.table_next_row();
            self.set_next_item_open(true);
            if let Some(_t) = self.push_tree_node("Ressources") {
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
        self.draw_me2_bonus_powers(powers);
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

    fn draw_me2_bonus_powers(&self, powers: &mut Vec<Power>) {
        let ui = self.ui;

        // Table
        let _t = match self.begin_table(im_str!("gameplay-table"), 1) {
            Some(t) => t,
            None => return,
        };

        // Tree node
        self.table_next_row();
        let _t = match self.push_tree_node("Bonus Powers") {
            Some(t) => t,
            None => return,
        };
        ui.same_line();
        self.draw_help_marker("You can use as many bonus powers as you want and customize your\nbuild to your liking.\nThe only restriction is the size of your screen !\nIf you want to remove a bonus power you need to reset your\ntalents `before` or you will loose some talent points.\nUnlike Mass Effect 3 the game will never recalculate your points.\nAt level 30 you have `51` points to spend.");

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
                    power.power_class_name = power_class_name.to_owned();
                    powers.push(power);
                } else if let Some((i, _)) = powers.iter().enumerate().find(|(_, power)| {
                    unicase::eq(power.power_class_name.as_ref(), power_class_name)
                }) {
                    powers.remove(i);
                }
            }
        }
    }

    fn draw_me2_known_plot(&self, me2_plot_table: &mut PlotTable, known_plots: &KnownPlotsState) {
        let ui = self.ui;
        let me2_known_plot = match &known_plots.me2 {
            Some(me2) => me2,
            None => return,
        };

        let Me2KnownPlot {
            player,
            crew,
            romance,
            missions,
            loyalty_missions,
            research_upgrades,
            rewards,
            captains_cabin,
        } = me2_known_plot;

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
                    for (category_name, known_plot) in plot_map.iter() {
                        if let Some(_t) = self.begin_table(&im_str!("{}-table", category_name), 1) {
                            self.table_next_row();
                            if let Some(_t) = self.push_tree_node(category_name) {
                                self.draw_me2_plot_category(me2_plot_table, known_plot);
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
    }

    fn draw_me2_plot_category(&self, plot_table: &mut PlotTable, known_plot: &PlotCategory) {
        let ui = self.ui;
        let PlotCategory { booleans, ints } = known_plot;

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
}
