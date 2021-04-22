use imgui::*;

use crate::save_data::{
    common::plot::PlotCategory,
    mass_effect_3::{
        known_plot::PlotVariable,
        player::{Player, Power},
        plot::PlotTable,
        Me3SaveGame,
    },
};

use super::*;

impl<'ui> Gui<'ui> {
    pub async fn draw_mass_effect_3(
        &self, save_game: &mut Me3SaveGame, known_plots: &KnownPlotsState,
    ) {
        let ui = self.ui;

        // Tabs
        if let Some(_t) = TabBar::new(im_str!("mass_effect_3")).begin(ui) {
            // General
            if let Some(_t) = TabItem::new(im_str!("General")).begin(ui) {
                self.draw_me3_general(save_game).await;
            }
            // Plot
            if let Some(_t) = TabItem::new(im_str!("Plot")).begin(ui) {
                self.draw_me3_known_plot(
                    &mut save_game.plot,
                    &mut save_game.player_variables,
                    known_plots,
                )
                .await;
            }
            // Raw
            if let Some(_t) = TabItem::new(im_str!("Raw")).begin(ui) {
                self.set_next_item_open(true);
                save_game.draw_raw_ui(self, "Mass Effect 3").await;
            }
        }
    }

    async fn draw_me3_general(&self, save_game: &mut Me3SaveGame) {
        let ui = self.ui;
        let Me3SaveGame { difficulty, end_game_state, conversation_mode, player, .. } = save_game;
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
            grenades,
            face_code,
            ..
        } = player;

        // 1ère colonne
        let _t = match self.begin_columns(2) {
            Some(t) => t,
            None => return,
        };
        self.table_next_row();

        // General
        if let Some(_t) = self.begin_table(im_str!("general-table"), 1) {
            self.table_next_row();
            self.set_next_item_open(true);
            if let Some(_t) = self.push_tree_node("General") {
                self.table_next_row();
                difficulty.draw_raw_ui(self, "Difficulty").await;
                self.table_next_row();
                conversation_mode.draw_raw_ui(self, "Conversation Mode").await;
                self.table_next_row();
                end_game_state.draw_raw_ui(self, "End Game State").await;
            }
        }

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
                    self.draw_help_marker("If you change your gender, disable head morph or import one appropriate\nor the Reapers will be the least of your worries...").await;
                }

                self.table_next_row();
                origin.draw_raw_ui(self, "Origin").await;

                self.table_next_row();
                notoriety.draw_raw_ui(self, "Notoriety").await;

                self.table_next_row();
                face_code.draw_raw_ui(self, "Identity Code").await;
                ui.same_line();
                self.draw_help_marker("If you change this you can display whatever you want in the menus\nin place of your `Identity Code`, which is pretty cool !").await;
            }
        }

        // Gameplay
        if let Some(_t) = self.begin_table(im_str!("gameplay-table"), 1) {
            self.table_next_row();
            self.set_next_item_open(true);
            if let Some(_t) = self.push_tree_node("Gameplay") {
                self.table_next_row();
                self.draw_me3_class(class_name).await;

                self.table_next_row();
                level.draw_raw_ui(self, "Level").await;

                self.table_next_row();
                current_xp.draw_raw_ui(self, "Current XP").await;

                self.table_next_row();
                talent_points.draw_raw_ui(self, "Talent Points").await;

                self.table_next_row();
                credits.draw_raw_ui(self, "Credits").await;

                self.table_next_row();
                medigel.draw_raw_ui(self, "Medi-gel").await;

                self.table_next_row();
                grenades.draw_raw_ui(self, "Grenades").await;
            }
        }

        // 2ème colonne
        self.table_next_column();
        self.set_next_item_open(true);
        self.draw_me3_bonus_powers(powers).await;
    }

    async fn draw_me3_bonus_powers(&self, powers: &mut Vec<Power>) {
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
        self.draw_help_marker("You can use as many bonus powers as you want\nand customize your build to your liking.\nThe only restriction is the size of your screen !").await;

        const POWER_LIST: [(&ImStr, &ImStr); 19] = [
            (im_str!("SFXGameContent.SFXPowerCustomAction_EnergyDrain"), im_str!("Energy Drain")),
            (
                im_str!("SFXGameContent.SFXPowerCustomAction_ProtectorDrone"),
                im_str!("Protector Drone"),
            ),
            (
                im_str!("SFXGameContent.SFXPowerCustomAction_GethShieldBoost"),
                im_str!("Geth Shield Boost"),
            ),
            (im_str!("SFXGameContent.SFXPowerCustomAction_Decoy"), im_str!("Decoy")),
            (
                im_str!("SFXGameContent.SFXPowerCustomAction_ArmorPiercingAmmo"),
                im_str!("Armor Piercing Ammo"),
            ),
            (
                im_str!("SFXGameContent.SFXPowerCustomAction_ProximityMine"),
                im_str!("Proximity Mine"),
            ),
            (im_str!("SFXGameContent.SFXPowerCustomAction_Barrier"), im_str!("Barrier")),
            (im_str!("SFXGameContent.SFXPowerCustomAction_Reave"), im_str!("Reave")),
            (
                im_str!("SFXGameContent.SFXPowerCustomAction_InfernoGrenade"),
                im_str!("Inferno Grenade"),
            ),
            (im_str!("SFXGameContent.SFXPowerCustomAction_Marksman"), im_str!("Marksman")),
            (im_str!("SFXGameContent.SFXPowerCustomAction_WarpAmmo"), im_str!("Warp Ammo")),
            (im_str!("SFXGameContent.SFXPowerCustomAction_Stasis"), im_str!("Stasis")),
            (
                im_str!("SFXGameContent.SFXPowerCustomAction_Fortification"),
                im_str!("Fortification"),
            ),
            (im_str!("SFXGameContent.SFXPowerCustomAction_Carnage"), im_str!("Carnage")),
            (im_str!("SFXGameContent.SFXPowerCustomAction_Slam"), im_str!("Slam")),
            (im_str!("SFXGameContent.SFXPowerCustomAction_DarkChannel"), im_str!("DarkChannel")),
            (
                im_str!("SFXGameContentDLC_Exp_Pack001.SFXPowerCustomAction_Dominate"),
                im_str!("Dominate"),
            ),
            (
                im_str!("SFXGameContentDLC_Exp_Pack002.SFXPowerCustomAction_AriaLash"),
                im_str!("Lash"),
            ),
            (
                im_str!("SFXGameContentDLC_Exp_Pack002.SFXPowerCustomAction_BioticFlare"),
                im_str!("Flare"),
            ),
        ];

        for &(power_class_name, power_name) in &POWER_LIST {
            let mut selected = powers
                .iter_mut()
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

    async fn draw_me3_class(&self, class_name: &mut ImString) {
        let ui = self.ui;
        const CLASS_LIST: [(&ImStr, &ImStr); 12] = [
            (im_str!("SFXGame.SFXPawn_PlayerAdept"), im_str!("Adept")),
            (im_str!("SFXGame.SFXPawn_PlayerAdeptNonCombat"), im_str!("Adept (out of combat)")),
            (im_str!("SFXGame.SFXPawn_PlayerEngineer"), im_str!("Engineer")),
            (
                im_str!("SFXGame.SFXPawn_PlayerEngineerNonCombat"),
                im_str!("Engineer (out of combat)"),
            ),
            (im_str!("SFXGame.SFXPawn_PlayerInfiltrator"), im_str!("Infiltrator")),
            (
                im_str!("SFXGame.SFXPawn_PlayerInfiltratorNonCombat"),
                im_str!("Infiltrator (out of combat)"),
            ),
            (im_str!("SFXGame.SFXPawn_PlayerSentinel"), im_str!("Sentinel")),
            (
                im_str!("SFXGame.SFXPawn_PlayerSentinelNonCombat"),
                im_str!("Sentinel (out of combat)"),
            ),
            (im_str!("SFXGame.SFXPawn_PlayerSoldier"), im_str!("Soldier")),
            (im_str!("SFXGame.SFXPawn_PlayerSoldierNonCombat"), im_str!("Soldier (out of combat)")),
            (im_str!("SFXGame.SFXPawn_PlayerVanguard"), im_str!("Vanguard")),
            (
                im_str!("SFXGame.SFXPawn_PlayerVanguardNonCombat"),
                im_str!("Vanguard (out of combat)"),
            ),
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

    async fn draw_me3_known_plot(
        &self, plot_table: &mut PlotTable, player_variables: &mut IndexMap<ImString, i32>,
        known_plots: &KnownPlotsState,
    ) {
        let ui = self.ui;
        let me3_known_plot = match &known_plots.me3 {
            Some(me3) => me3,
            None => return,
        };

        let Me3KnownPlot {
            general,
            appearances,
            crew,
            romance,
            missions,
            citadel_dlc,
            intel,
            normandy,
            weapons_powers,
            me1_imported,
        } = me3_known_plot;

        // Tab bar
        let _t = match TabBar::new(im_str!("plot-tab")).begin(ui) {
            Some(t) => t,
            None => return,
        };

        // Mass Effect 3
        if let Some(_t) = TabItem::new(im_str!("General")).begin(ui) {
            if let Some(_t) = self.begin_table(im_str!("plot-table"), 1) {
                self.draw_me3_plot_category(plot_table, general).await;
            }
        }

        let categories = [
            (im_str!("Appearances"), appearances),
            (im_str!("Crew"), crew),
            (im_str!("Romance"), romance),
            (im_str!("Missions"), missions),
            (im_str!("Normandy"), normandy),
            (im_str!("Citadel DLC"), citadel_dlc),
        ];

        for (title, plot_map) in &categories {
            if let Some(_t) = TabItem::new(title).begin(ui) {
                for (category_name, known_plot) in plot_map.iter() {
                    if let Some(_t) = self.begin_table(&im_str!("{}-table", category_name), 1) {
                        self.table_next_row();
                        if let Some(_t) = self.push_tree_node(category_name) {
                            self.draw_me3_plot_category(plot_table, known_plot).await;
                        }
                    }
                }
            }
        }

        if let Some(_t) = TabItem::new(im_str!("Intel")).begin(ui) {
            if let Some(_t) = self.begin_table(im_str!("plot-table"), 1) {
                self.draw_me3_plot_category(plot_table, intel).await;
            }
        }

        // Weapons / Powers
        if let Some(_t) = TabItem::new(im_str!("Weapons / Powers")).begin(ui) {
            for (category_name, known_plot) in weapons_powers {
                if let Some(_t) = self.begin_table(&im_str!("{}-table", category_name), 1) {
                    self.table_next_row();
                    if let Some(_t) = self.push_tree_node(category_name) {
                        self.draw_me3_plot_variable(plot_table, player_variables, known_plot).await;
                    }
                }
            }
        }

        // Mass Effect 2
        let me2_known_plot = match &known_plots.me2 {
            Some(me2_known_plot) => me2_known_plot,
            None => return,
        };

        let _colors = self.style_colors(Theme::MassEffect2).await;
        if let Some(_t) = TabItem::new(im_str!("Mass Effect 2")).begin(ui) {
            self.draw_me2_imported_known_plot(plot_table, me2_known_plot).await;
        }
        // Mass Effect 1
        {
            let _colors = self.style_colors(Theme::MassEffect1).await;
            if let Some(_t) = TabItem::new(im_str!("Mass Effect 1")).begin(ui) {
                self.draw_me1_imported_known_plot(plot_table, me1_imported).await;
            }
        }
    }

    pub async fn draw_me2_imported_known_plot(
        &self, me3_plot_table: &mut PlotTable, me2_known_plot: &Me2KnownPlot,
    ) {
        let ui = self.ui;
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

        // Tab bar
        let _t = match TabBar::new(im_str!("plot-tab")).begin(ui) {
            Some(t) => t,
            None => return,
        };

        // Player
        if let Some(_t) = TabItem::new(im_str!("Player")).begin(ui) {
            if let Some(_t) = self.begin_table(im_str!("plot-table"), 1) {
                self.draw_me3_plot_category(me3_plot_table, player).await;
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
            if let Some(_t) = TabItem::new(title).begin(ui) {
                for (category_name, known_plot) in plot_map.iter() {
                    if let Some(_t) = self.begin_table(&im_str!("{}-table", category_name), 1) {
                        self.table_next_row();
                        if let Some(_t) = self.push_tree_node(category_name) {
                            self.draw_me3_plot_category(me3_plot_table, known_plot).await;
                        }
                    }
                }
            }
        }

        // Rewards
        if let Some(_t) = TabItem::new(im_str!("Rewards")).begin(ui) {
            if let Some(_t) = self.begin_table(im_str!("plot-table"), 1) {
                self.draw_me3_plot_category(me3_plot_table, rewards).await;
            }
        }
        // Captain's cabin
        if let Some(_t) = TabItem::new(im_str!("Captain's cabin")).begin(ui) {
            if let Some(_t) = self.begin_table(im_str!("plot-table"), 1) {
                self.draw_me3_plot_category(me3_plot_table, captains_cabin).await;
            }
        }
    }

    pub async fn draw_me1_imported_known_plot(
        &self, me1_plot_table: &mut PlotTable, me1_imported: &Me1KnownPlot,
    ) {
        let ui = self.ui;
        let Me1KnownPlot { player_crew, missions } = me1_imported;

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
                            self.draw_me3_plot_category(me1_plot_table, known_plot).await;
                        }
                    }
                }
            }
        }
    }

    async fn draw_me3_plot_category(&self, plot_table: &mut PlotTable, known_plot: &PlotCategory) {
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
            let plot = plot_table.int_variables.entry(*plot_id as i32).or_default();

            self.table_next_row();
            plot.draw_raw_ui(self, plot_desc).await;
        }
    }

    async fn draw_me3_plot_variable(
        &self, plot_table: &mut PlotTable, player_variables: &mut IndexMap<ImString, i32>,
        known_plot: &PlotVariable,
    ) {
        let PlotVariable { booleans, variables } = known_plot;

        if booleans.is_empty() && player_variables.is_empty() {
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
        // Variables
        for (variable_id, variable_desc) in variables {
            let variable =
                player_variables.iter_mut().find(|(key, _)| unicase::eq(key.to_str(), variable_id));

            let value = match variable {
                Some((_, value)) => value,
                None => player_variables.entry(ImString::new(variable_id)).or_default(),
            };

            self.table_next_row();
            value.draw_raw_ui(self, variable_desc).await;
        }
    }
}
