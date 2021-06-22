use imgui::{im_str, ComboBox, ImStr, ImString, ListClipper, Selectable, TabBar, TabItem};
use indexmap::IndexMap;

use crate::{
    databases::Database,
    gui::shared::PlotType,
    save_data::{
        mass_effect_1::plot_db::Me1PlotDb,
        mass_effect_2::plot_db::Me2PlotDb,
        mass_effect_3::{
            player::{Player, Power},
            plot::PlotTable,
            plot_db::{Me3PlotDb, PlotVariable},
            Me3SaveGame,
        },
        shared::{
            player::{Notoriety, Origin},
            plot::PlotCategory,
        },
        ImguiString, RawUi,
    },
};

use super::{
    imgui_utils::{TabScroll, Table, TreeNode},
    Gui, PlotFilterState, Theme,
};

impl<'ui> Gui<'ui> {
    pub fn draw_mass_effect_3(
        &self, save_game: &mut Me3SaveGame, plot_filter: &mut PlotFilterState,
    ) -> Option<()> {
        let ui = self.ui;

        // Tab bar
        let _tab_bar = TabBar::new(im_str!("mass_effect_3")).begin(ui)?;

        // General
        TabScroll::new(im_str!("General")).build(ui, || {
            self.draw_me3_general(save_game);
        });

        // Plot
        TabItem::new(im_str!("Plot")).build(ui, || {
            self.draw_me3_plot(&mut save_game.plot, &mut save_game.player_variables);
        });

        // Head Morph
        TabScroll::new(im_str!("Head Morph")).build(ui, || {
            self.draw_head_morph(&mut save_game.player.appearance.head_morph);
        });

        // Raw Data
        TabScroll::new(im_str!("Raw Data")).build(ui, || {
            self.set_next_item_open(true);
            save_game.draw_raw_ui(self, "Mass Effect 3");
        });

        // Raw Plot
        TabItem::new(im_str!("Raw Plot")).build(ui, || {
            if let Some(plot_db) = Database::me3_raw_plot() {
                let PlotTable { booleans, integers, floats, .. } = &mut save_game.plot;
                self.draw_raw_plot(
                    booleans,
                    PlotType::IndexMap(integers),
                    PlotType::IndexMap(floats),
                    plot_db,
                    plot_filter,
                );
            }
        });
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
        let _columns = Table::begin_columns(2, ui)?;
        Table::next_row();

        // Role Play
        Table::new(im_str!("role-play-table"), 1).build(ui, || {
            Table::next_row();
            self.set_next_item_open(true);
            TreeNode::new("Role-Play").build(ui, || {
                Table::next_row();
                first_name.draw_raw_ui(self, "Name");

                // Gender
                Table::next_row();
                const GENDER_LIST: [&ImStr; 2] = [im_str!("Male"), im_str!("Female")];
                let mut gender = *is_female as usize;
                if self.draw_edit_enum("Gender", &mut gender, &GENDER_LIST) {
                    *is_female = gender != 0;

                    // Plot
                    // FIXME: ME1
                    // ME2
                    if let Some(mut is_female) = plot.booleans.get_mut(66) {
                        *is_female = gender != 0;
                    }
                    // ME3
                    if let Some(mut is_female) = plot.booleans.get_mut(17662) {
                        *is_female = gender != 0;
                    }

                    // Loco / Lola
                    let is_loco = match plot.booleans.get(19578) {
                        Some(val) => *val,
                        None => false,
                    };
                    let is_lola = match plot.booleans.get(19579) {
                        Some(val) => *val,
                        None => false,
                    };

                    if is_loco || is_lola {
                        if let Some(mut is_loco) = plot.booleans.get_mut(19578) {
                            *is_loco = gender == 0;
                        }
                        if let Some(mut is_lola) = plot.booleans.get_mut(19579) {
                            *is_lola = gender != 0;
                        }
                    }
                }

                ui.same_line();
                self.draw_help_marker(
                        "If you change your gender, disable the head morph or import an appropriate one.\n\
                        Otherwise, the Reapers will be the least of your worries..."
                    );

                // Origin
                Table::next_row();
                const ORIGIN_LIST: [&ImStr; 4] =
                    [im_str!("None"), im_str!("Spacer"), im_str!("Colonist"), im_str!("Earthborn")];
                let mut origin_idx = origin.clone() as usize;
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
                            if let Some(mut spacer) = plot.booleans.get_mut(1533) {
                                *spacer = true;
                            }
                            if let Some(mut colonist) = plot.booleans.get_mut(1535) {
                                *colonist = false;
                            }
                            if let Some(mut eathborn) = plot.booleans.get_mut(1534) {
                                *eathborn = false;
                            }
                        }
                        Origin::Colonist => {
                            if let Some(mut spacer) = plot.booleans.get_mut(1533) {
                                *spacer = false;
                            }
                            if let Some(mut colonist) = plot.booleans.get_mut(1535) {
                                *colonist = true;
                            }
                            if let Some(mut eathborn) = plot.booleans.get_mut(1534) {
                                *eathborn = false;
                            }
                        }
                        Origin::Earthborn => {
                            if let Some(mut spacer) = plot.booleans.get_mut(1533) {
                                *spacer = false;
                            }
                            if let Some(mut colonist) = plot.booleans.get_mut(1535) {
                                *colonist = false;
                            }
                            if let Some(mut eathborn) = plot.booleans.get_mut(1534) {
                                *eathborn = true;
                            }
                        }
                    }

                    // ME1 plot
                    if let Some(me1_origin) = plot.integers.get_mut(&10001) {
                        *me1_origin = origin_idx as i32;
                    }
                }

                // Notoriety
                Table::next_row();
                const NOTORIETY_LIST: [&ImStr; 4] = [
                    im_str!("None"),
                    im_str!("Survivor"),
                    im_str!("War Hero"),
                    im_str!("Ruthless"),
                ];
                let mut notoriety_idx = notoriety.clone() as usize;
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
                            if let Some(mut survivor) = plot.booleans.get_mut(1537) {
                                *survivor = true;
                            }
                            if let Some(mut war_hero) = plot.booleans.get_mut(1538) {
                                *war_hero = false;
                            }
                            if let Some(mut ruthless) = plot.booleans.get_mut(1539) {
                                *ruthless = false;
                            }
                        }
                        Notoriety::Warhero => {
                            if let Some(mut survivor) = plot.booleans.get_mut(1537) {
                                *survivor = false;
                            }
                            if let Some(mut war_hero) = plot.booleans.get_mut(1538) {
                                *war_hero = true;
                            }
                            if let Some(mut ruthless) = plot.booleans.get_mut(1539) {
                                *ruthless = false;
                            }
                        }
                        Notoriety::Ruthless => {
                            if let Some(mut survivor) = plot.booleans.get_mut(1537) {
                                *survivor = false;
                            }
                            if let Some(mut war_hero) = plot.booleans.get_mut(1538) {
                                *war_hero = false;
                            }
                            if let Some(mut ruthless) = plot.booleans.get_mut(1539) {
                                *ruthless = true;
                            }
                        }
                    }

                    // ME1 plot
                    if let Some(me1_notoriety) = plot.integers.get_mut(&10002) {
                        *me1_notoriety = notoriety_idx as i32;
                    }
                }

                Table::next_row();
                face_code.draw_raw_ui(self, "Identity Code");
                ui.same_line();
                self.draw_help_marker(
                    "If you change this you can display whatever you want in the menus\nin place of your `Identity Code`, which is pretty cool !"
                );
            });
        });

        // Morality
        Table::new(im_str!("morality-table"), 1).build(ui, || {
            Table::next_row();
            self.set_next_item_open(true);
            TreeNode::new("Morality").build(ui, || {
                if let Some(paragon) = plot.integers.get_mut(&10159) {
                    Table::next_row();
                    paragon.draw_raw_ui(self, "Paragon");
                }

                if let Some(renegade) = plot.integers.get_mut(&10160) {
                    Table::next_row();
                    renegade.draw_raw_ui(self, "Renegade");
                }

                if let Some(reputation) = plot.integers.get_mut(&10297) {
                    Table::next_row();
                    reputation.draw_raw_ui(self, "Reputation");
                }

                if let Some(reputation_points) = plot.integers.get_mut(&10380) {
                    Table::next_row();
                    reputation_points.draw_raw_ui(self, "Reputation Points");
                }
            });
        });

        // Gameplay
        Table::new(im_str!("gameplay-table"), 1).build(ui, || {
            Table::next_row();
            self.set_next_item_open(true);
            TreeNode::new("Gameplay").build(ui, || {
                Table::next_row();
                self.draw_me3_class(class_name);

                Table::next_row();
                level.draw_raw_ui(self, "Level");

                Table::next_row();
                current_xp.draw_raw_ui(self, "Current XP");

                Table::next_row();
                talent_points.draw_raw_ui(self, "Talent Points");

                Table::next_row();
                credits.draw_raw_ui(self, "Credits");

                Table::next_row();
                medigel.draw_raw_ui(self, "Medi-gel");

                Table::next_row();
                grenades.draw_raw_ui(self, "Grenades");

                Table::next_row();
                current_fuel.draw_raw_ui(self, "Current Fuel");
            });
        });

        // 2ème colonne
        Table::next_column();

        // General
        Table::new(im_str!("general-table"), 1).build(ui, || {
            Table::next_row();
            self.set_next_item_open(true);
            TreeNode::new("General").build(ui, || {
                Table::next_row();
                difficulty.draw_raw_ui(self, "Difficulty");
                Table::next_row();
                conversation_mode.draw_raw_ui(self, "Conversation Mode");
                Table::next_row();
                end_game_state.draw_raw_ui(self, "End Game State");
            });
        });

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
        let _table = Table::new(im_str!("gameplay-table"), 1).begin(ui)?;

        // Tree node
        Table::next_row();
        let _tree_node = TreeNode::new("Bonus Powers").push(ui)?;
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
            // FIXME: Stim pack
        ];

        for &(power_class_name, power_name) in &POWER_LIST {
            let mut selected = powers
                .iter()
                .any(|power| unicase::eq(power.power_class_name.as_ref(), power_class_name));

            Table::next_row();
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

    fn draw_me3_plot(
        &self, plot_table: &mut PlotTable, player_variables: &mut IndexMap<ImguiString, i32>,
    ) -> Option<()> {
        let ui = self.ui;

        let Me3PlotDb {
            general,
            appearances,
            crew,
            romance,
            missions,
            citadel_dlc,
            intel,
            normandy,
            weapons_powers,
        } = Database::me3_plot()?;
        let PlotTable { booleans, integers, .. } = plot_table;

        // Tab bar
        let _tab_bar = TabBar::new(im_str!("plot-tab")).begin(ui)?;

        // Mass Effect 3
        TabScroll::new(im_str!("General")).build(ui, || {
            Table::new(im_str!("plot-table"), 1).build(ui, || {
                self.draw_plot_category(booleans, PlotType::IndexMap(integers), general);
            });
        });

        let categories = [
            (im_str!("Appearances"), appearances),
            (im_str!("Crew"), crew),
            (im_str!("Romance"), romance),
            (im_str!("Missions"), missions),
            (im_str!("Normandy"), normandy),
            (im_str!("Citadel DLC"), citadel_dlc),
        ];

        for (title, plot_map) in &categories {
            TabScroll::new(title).build(ui, || {
                for (category_name, plot_db) in plot_map.iter() {
                    Table::new(&im_str!("{}-table", category_name), 1).build(ui, || {
                        Table::next_row();
                        TreeNode::new(category_name).build(ui, || {
                            self.draw_plot_category(
                                booleans,
                                PlotType::IndexMap(integers),
                                plot_db,
                            );
                        });
                    });
                }
            });
        }

        TabScroll::new(im_str!("Intel")).build(ui, || {
            Table::new(im_str!("plot-table"), 1).build(ui, || {
                self.draw_plot_category(booleans, PlotType::IndexMap(integers), intel);
            });
        });

        // Weapons / Powers
        TabScroll::new(im_str!("Weapons / Powers")).build(ui, || {
            for (category_name, plot_db) in weapons_powers {
                Table::new(&im_str!("{}-table", category_name), 1).build(ui, || {
                    Table::next_row();
                    TreeNode::new(category_name).build(ui, || {
                        self.draw_me3_plot_variable(plot_table, player_variables, plot_db);
                    });
                });
            }
        });

        // Mass Effect 2
        let _colors = self.style_colors(Theme::MassEffect2);
        TabItem::new(im_str!("Mass Effect 2")).build(ui, || {
            self.draw_me2_imported_plot(plot_table);
        });

        // Mass Effect 1
        {
            let _colors = self.style_colors(Theme::MassEffect1);
            TabItem::new(im_str!("Mass Effect 1")).build(ui, || {
                self.draw_me1_imported_plot_db(plot_table);
            });
        }
        Some(())
    }

    pub fn draw_me2_imported_plot(&self, me3_plot_table: &mut PlotTable) -> Option<()> {
        let ui = self.ui;
        let Me2PlotDb {
            player,
            crew,
            romance,
            missions,
            loyalty_missions,
            research_upgrades,
            rewards,
            captains_cabin,
            imported_me1: _,
        } = Database::me2_plot()?;
        let PlotTable { booleans, integers, .. } = me3_plot_table;

        // Tab bar
        let _tab_bar = TabBar::new(im_str!("plot-tab")).begin(ui)?;

        // Player
        TabScroll::new(im_str!("Player")).build(ui, || {
            Table::new(im_str!("plot-table"), 1).build(ui, || {
                self.draw_plot_category(booleans, PlotType::IndexMap(integers), player);
            });
        });

        let categories = [
            (im_str!("Crew"), crew),
            (im_str!("Romance"), romance),
            (im_str!("Missions"), missions),
            (im_str!("Loyalty missions"), loyalty_missions),
            (im_str!("Research / Upgrades"), research_upgrades),
        ];

        for (title, plot_map) in &categories {
            TabScroll::new(title).build(ui, || {
                for (category_name, plot_db) in plot_map.iter() {
                    Table::new(&im_str!("{}-table", category_name), 1).build(ui, || {
                        Table::next_row();
                        TreeNode::new(category_name).build(ui, || {
                            self.draw_plot_category(
                                booleans,
                                PlotType::IndexMap(integers),
                                plot_db,
                            );
                        });
                    });
                }
            });
        }

        // Rewards
        TabScroll::new(im_str!("Rewards")).build(ui, || {
            Table::new(im_str!("plot-table"), 1).build(ui, || {
                self.draw_plot_category(booleans, PlotType::IndexMap(integers), rewards);
            });
        });

        // Captain's cabin
        TabScroll::new(im_str!("Captain's cabin")).build(ui, || {
            Table::new(im_str!("plot-table"), 1).build(ui, || {
                self.draw_plot_category(booleans, PlotType::IndexMap(integers), captains_cabin);
            });
        });
        Some(())
    }

    pub fn draw_me1_imported_plot_db(&self, plot_table: &mut PlotTable) -> Option<()> {
        let ui = self.ui;
        let Me1PlotDb { player_crew, missions } = Database::me1_plot()?;

        // Tab bar
        let _tab_bar = TabBar::new(im_str!("plot-tab")).begin(ui)?;

        let categories = [(im_str!("Player / Crew"), player_crew), (im_str!("Missions"), missions)];

        for (title, plot_map) in &categories {
            TabScroll::new(title).build(ui, || {
                for (category_name, plot_db) in plot_map.iter() {
                    Table::new(&im_str!("{}-table", category_name), 1).build(ui, || {
                        Table::next_row();
                        TreeNode::new(category_name).build(ui, || {
                            self.draw_me3_imported_me1_plot_category(plot_table, plot_db);
                        });
                    });
                }
            });
        }
        Some(())
    }

    fn draw_me3_imported_me1_plot_category(
        &self, plot_table: &mut PlotTable, plot_db: &PlotCategory,
    ) {
        let PlotTable { booleans, integers, .. } = plot_table;
        let PlotCategory { booleans: bool_db, integers: int_db } = plot_db;

        self.draw_plot_bools(booleans, bool_db, true);
        self.draw_plot_ints(PlotType::IndexMap(integers), int_db, true);
    }

    fn draw_me3_plot_variable(
        &self, plot_table: &mut PlotTable, variables: &mut IndexMap<ImguiString, i32>,
        plot_db: &PlotVariable,
    ) {
        let ui = self.ui;
        let PlotTable { booleans, .. } = plot_table;
        let PlotVariable { booleans: bool_db, variables: var_db } = plot_db;

        if bool_db.is_empty() && variables.is_empty() {
            return;
        }

        // Booleans
        self.draw_plot_bools(booleans, bool_db, false);

        // Variables
        let mut clipper = ListClipper::new(var_db.len() as i32).begin(ui);
        while clipper.step() {
            for i in clipper.display_start()..clipper.display_end() {
                let (var_id, var_desc) = var_db.get_index(i as usize).unwrap();
                let variable =
                    variables.iter_mut().find(|(key, _)| unicase::eq(key.to_str(), var_id));

                let value = match variable {
                    Some((_, value)) => value,
                    None => variables.entry(ImString::new(var_id).into()).or_default(),
                };

                Table::next_row();
                value.draw_raw_ui(self, &format!("{}##var-{}", var_desc, var_desc));
            }
        }
        clipper.end();
    }
}
