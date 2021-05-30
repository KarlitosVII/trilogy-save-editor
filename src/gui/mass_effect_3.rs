use if_chain::if_chain;
use imgui::{
    im_str, ChildWindow, ComboBox, ImStr, ImString, ListClipper, Selectable, TabBar, TabItem,
};
use indexmap::IndexMap;

use crate::{
    event_handler::MainEvent,
    save_data::{
        mass_effect_1::known_plot::Me1KnownPlot,
        mass_effect_2::known_plot::Me2KnownPlot,
        mass_effect_3::{
            known_plot::{Me3KnownPlot, PlotVariable},
            player::{Player, Power},
            plot::PlotTable,
            Me3SaveGame,
        },
        shared::{
            appearance::{HasHeadMorph, HeadMorph},
            player::{Notoriety, Origin},
            plot::PlotCategory,
        },
        ImguiString, RawUi,
    },
};

use super::{Gui, KnownPlotsState, Theme};

impl<'ui> Gui<'ui> {
    pub fn draw_mass_effect_3(
        &self, save_game: &mut Me3SaveGame, known_plots: &KnownPlotsState,
    ) -> Option<()> {
        let ui = self.ui;

        // Tab bar
        let _t = TabBar::new(im_str!("mass_effect_3")).begin(ui)?;

        // General
        if_chain! {
            if let Some(_t) = TabItem::new(im_str!("General")).begin(ui);
            if let Some(_t) = ChildWindow::new(im_str!("scroll")).begin(ui);
            then {
                self.draw_me3_general(save_game);
            }
        }
        // Plot
        if let Some(_t) = TabItem::new(im_str!("Plot")).begin(ui) {
            self.draw_me3_known_plot(
                &mut save_game.plot,
                &mut save_game.player_variables,
                known_plots,
            );
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
                save_game.draw_raw_ui(self, "Mass Effect 3");
            }
        }
        Some(())
    }

    fn draw_me3_general(&self, save_game: &mut Me3SaveGame) -> Option<()> {
        let ui = self.ui;
        let Me3SaveGame { difficulty, end_game_state, conversation_mode, player, plot, .. } =
            save_game;
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
            current_fuel,
            grenades,
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

                        // Plot
                        if let Some(mut is_female) = plot.bool_variables.get_mut(17662) {
                            *is_female = gender != 0;
                        }

                        // Loco / Lola
                        let is_loco = match plot.bool_variables.get(19578) {
                            Some(val) => *val,
                            None => false,
                        };
                        let is_lola = match plot.bool_variables.get(19579) {
                            Some(val) => *val,
                            None => false,
                        };

                        if is_loco || is_lola {
                            if let Some(mut is_loco) = plot.bool_variables.get_mut(19578) {
                                *is_loco = gender == 0;
                            }
                            if let Some(mut is_lola) = plot.bool_variables.get_mut(19579) {
                                *is_lola = gender != 0;
                            }
                        }
                    }

                    ui.same_line();
                    self.draw_help_marker(
                        "If you change your gender, disable the head morph or import an appropriate one.\n\
                        Otherwise, the Reapers will be the least of your worries..."
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
                        if let Some(me1_origin) = plot.int_variables.get_mut(&10001) {
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
                        if let Some(me1_notoriety) = plot.int_variables.get_mut(&10002) {
                            *me1_notoriety = notoriety_idx as i32;
                        }
                    }
                }

                self.table_next_row();
                face_code.draw_raw_ui(self, "Identity Code");
                ui.same_line();
                self.draw_help_marker(
                    "If you change this you can display whatever you want in the menus\nin place of your `Identity Code`, which is pretty cool !"
                );
            }
        }

        // Morality
        if let Some(_t) = self.begin_table(im_str!("morality-table"), 1) {
            self.table_next_row();
            self.set_next_item_open(true);
            if let Some(_t) = self.push_tree_node("Morality") {
                if let Some(paragon) = plot.int_variables.get_mut(&10159) {
                    self.table_next_row();
                    paragon.draw_raw_ui(self, "Paragon");
                }

                if let Some(renegade) = plot.int_variables.get_mut(&10160) {
                    self.table_next_row();
                    renegade.draw_raw_ui(self, "Renegade");
                }

                if let Some(reputation) = plot.int_variables.get_mut(&10297) {
                    self.table_next_row();
                    reputation.draw_raw_ui(self, "Reputation");
                }

                if let Some(reputation_points) = plot.int_variables.get_mut(&10380) {
                    self.table_next_row();
                    reputation_points.draw_raw_ui(self, "Reputation Points");
                }
            }
        }

        // Gameplay
        if let Some(_t) = self.begin_table(im_str!("gameplay-table"), 1) {
            self.table_next_row();
            self.set_next_item_open(true);
            if let Some(_t) = self.push_tree_node("Gameplay") {
                self.table_next_row();
                self.draw_me3_class(class_name);

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

                self.table_next_row();
                grenades.draw_raw_ui(self, "Grenades");

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
                conversation_mode.draw_raw_ui(self, "Conversation Mode");
                self.table_next_row();
                end_game_state.draw_raw_ui(self, "End Game State");
            }
        }

        // Bonus Powers
        self.set_next_item_open(true);
        self.draw_me3_bonus_powers(powers)
    }

    fn draw_me3_class(&self, class_name: &mut ImString) {
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

    fn draw_me3_bonus_powers(&self, powers: &mut Vec<Power>) -> Option<()> {
        let ui = self.ui;

        // Table
        let _t = self.begin_table(im_str!("gameplay-table"), 1)?;

        // Tree node
        self.table_next_row();
        let _t = self.push_tree_node("Bonus Powers")?;
        ui.same_line();
        self.draw_help_marker(
            "You can use as many bonus powers as you want\nand customize your build to your liking.\nThe only restriction is the size of your screen !"
        );

        const POWER_LIST: [(&ImStr, &ImStr); 19] = [
            (im_str!("SFXGameContent.SFXPowerCustomAction_EnergyDrain"), im_str!("Energy Drain")),
            (
                im_str!("SFXGameContent.SFXPowerCustomAction_ProtectorDrone"),
                im_str!("Defense Drone"),
            ),
            (
                im_str!("SFXGameContent.SFXPowerCustomAction_GethShieldBoost"),
                im_str!("Defense Matrix"),
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
            (im_str!("SFXGameContent.SFXPowerCustomAction_DarkChannel"), im_str!("Dark Channel")),
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

    fn draw_me3_known_plot(
        &self, plot_table: &mut PlotTable, player_variables: &mut IndexMap<ImguiString, i32>,
        known_plots: &KnownPlotsState,
    ) -> Option<()> {
        let ui = self.ui;
        let me3_known_plot = known_plots.me3.as_ref()?;

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
        let _t = TabBar::new(im_str!("plot-tab")).begin(ui)?;

        // Mass Effect 3
        if_chain! {
            if let Some(_t) = TabItem::new(im_str!("General")).begin(ui);
            if let Some(_t) = ChildWindow::new(im_str!("scroll")).begin(ui);
            if let Some(_t) = self.begin_table(im_str!("plot-table"), 1);
            then {
                self.draw_me3_plot_category(plot_table, general);
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
            if_chain! {
                if let Some(_t) = TabItem::new(title).begin(ui);
                if let Some(_t) = ChildWindow::new(im_str!("scroll")).begin(ui);
                then {
                    for (category_name, known_plot) in plot_map.iter() {
                        if let Some(_t) = self.begin_table(&im_str!("{}-table", category_name), 1) {
                            self.table_next_row();
                            if let Some(_t) = self.push_tree_node(category_name) {
                                self.draw_me3_plot_category(plot_table, known_plot);
                            }
                        }
                    }
                }
            }
        }

        if_chain! {
            if let Some(_t) = TabItem::new(im_str!("Intel")).begin(ui);
            if let Some(_t) = ChildWindow::new(im_str!("scroll")).begin(ui);
            if let Some(_t) = self.begin_table(im_str!("plot-table"), 1);
            then {
                self.draw_me3_plot_category(plot_table, intel);
            }
        }

        // Weapons / Powers
        if_chain! {
            if let Some(_t) = TabItem::new(im_str!("Weapons / Powers")).begin(ui);
            if let Some(_t) = ChildWindow::new(im_str!("scroll")).begin(ui);
            then {
                for (category_name, known_plot) in weapons_powers {
                    if let Some(_t) = self.begin_table(&im_str!("{}-table", category_name), 1) {
                        self.table_next_row();
                        if let Some(_t) = self.push_tree_node(category_name) {
                            self.draw_me3_plot_variable(plot_table, player_variables, known_plot);
                        }
                    }
                }
            }
        }

        // Mass Effect 2
        let me2_known_plot = known_plots.me2.as_ref()?;

        let _colors = self.style_colors(Theme::MassEffect2);
        if let Some(_t) = TabItem::new(im_str!("Mass Effect 2")).begin(ui) {
            self.draw_me2_imported_known_plot(plot_table, me2_known_plot);
        }
        // Mass Effect 1
        {
            let _colors = self.style_colors(Theme::MassEffect1);
            if let Some(_t) = TabItem::new(im_str!("Mass Effect 1")).begin(ui) {
                self.draw_me1_imported_known_plot(plot_table, me1_imported);
            }
        }
        Some(())
    }

    pub fn draw_me2_imported_known_plot(
        &self, me3_plot_table: &mut PlotTable, me2_known_plot: &Me2KnownPlot,
    ) -> Option<()> {
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
            imported_me1: _,
        } = me2_known_plot;

        // Tab bar
        let _t = TabBar::new(im_str!("plot-tab")).begin(ui)?;

        // Player
        if_chain! {
            if let Some(_t) = TabItem::new(im_str!("Player")).begin(ui);
            if let Some(_t) = ChildWindow::new(im_str!("scroll")).begin(ui);
            if let Some(_t) = self.begin_table(im_str!("plot-table"), 1);
            then {
                self.draw_me3_plot_category(me3_plot_table, player);
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
                                self.draw_me3_plot_category(me3_plot_table, known_plot);
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
                self.draw_me3_plot_category(me3_plot_table, rewards);
            }
        }

        // Captain's cabin
        if_chain! {
            if let Some(_t) = TabItem::new(im_str!("Captain's cabin")).begin(ui);
            if let Some(_t) = ChildWindow::new(im_str!("scroll")).begin(ui);
            if let Some(_t) = self.begin_table(im_str!("plot-table"), 1);
            then {
                self.draw_me3_plot_category(me3_plot_table, captains_cabin);
            }
        }
        Some(())
    }

    pub fn draw_me1_imported_known_plot(
        &self, me1_plot_table: &mut PlotTable, me1_imported: &Me1KnownPlot,
    ) -> Option<()> {
        let ui = self.ui;
        let Me1KnownPlot { player_crew, missions } = me1_imported;

        // Tab bar
        let _t = TabBar::new(im_str!("plot-tab")).begin(ui)?;

        let categories = [(im_str!("Player / Crew"), player_crew), (im_str!("Missions"), missions)];

        for (title, plot_map) in &categories {
            if_chain! {
                if let Some(_t) = TabItem::new(title).begin(ui);
                if let Some(_t) = ChildWindow::new(im_str!("scroll")).begin(ui);
                then {
                    for (category_name, known_plot) in plot_map.iter() {
                        if let Some(_t) = self.begin_table(&im_str!("{}-table", category_name), 1) {
                            self.table_next_row();
                            if let Some(_t) = self.push_tree_node(category_name) {
                                self.draw_me3_plot_category(me1_plot_table, known_plot);
                            }
                        }
                    }
                }
            }
        }
        Some(())
    }

    fn draw_me3_plot_category(&self, plot_table: &mut PlotTable, known_plot: &PlotCategory) {
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
                let plot = plot_table.int_variables.entry(*plot_id as i32).or_default();

                self.table_next_row();
                plot.draw_raw_ui(self, &format!("{}##int-{}", plot_desc, plot_desc));
            }
        }
    }

    fn draw_me3_plot_variable(
        &self, plot_table: &mut PlotTable, player_variables: &mut IndexMap<ImguiString, i32>,
        known_plot: &PlotVariable,
    ) {
        let ui = self.ui;
        let PlotVariable { booleans, variables } = known_plot;

        if booleans.is_empty() && player_variables.is_empty() {
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
        // Variables
        let mut clipper = ListClipper::new(variables.len() as i32).begin(ui);
        while clipper.step() {
            for i in clipper.display_start()..clipper.display_end() {
                let (variable_id, variable_desc) = variables.get_index(i as usize).unwrap();
                let variable = player_variables
                    .iter_mut()
                    .find(|(key, _)| unicase::eq(key.to_str(), variable_id));

                let value = match variable {
                    Some((_, value)) => value,
                    None => player_variables.entry(ImString::new(variable_id).into()).or_default(),
                };

                self.table_next_row();
                value.draw_raw_ui(self, &format!("{}##var-{}", variable_desc, variable_desc));
            }
        }
    }

    pub fn draw_me3_and_le_head_morph(&self, head_morph: &mut HasHeadMorph) {
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
                ui.same_line();
                has_head_morph.draw_raw_ui(self, "Enable head morph");
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
