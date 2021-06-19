use imgui::{im_str, ComboBox, ImStr, ImString, Selectable, TabBar, TabItem};

use crate::{
    databases::Database,
    event_handler::MainEvent,
    save_data::{
        mass_effect_2::{
            player::{Player, Power},
            plot_db::Me2PlotDb,
            Me2LeSaveGame, Me2SaveGame,
        },
        shared::{
            appearance::HeadMorph,
            player::{Notoriety, Origin},
            plot::{Me1PlotTable, PlotTable},
        },
        RawUi,
    },
};

use super::{
    imgui_utils::{TabScroll, Table, TreeNode},
    Gui, PlotFilterState, Theme,
};

enum Me2Type<'a> {
    Vanilla(&'a mut Me2SaveGame),
    Legendary(&'a mut Me2LeSaveGame),
}

impl<'ui> Gui<'ui> {
    pub fn draw_mass_effect_2(
        &self, save_game: &mut Me2SaveGame, plot_filter: &mut PlotFilterState,
    ) -> Option<()> {
        let ui = self.ui;

        // Tab bar
        let _tab_bar = TabBar::new(im_str!("mass_effect_2")).begin(ui)?;

        // General
        TabScroll::new(im_str!("General")).build(ui, || {
            self.draw_me2_general(Me2Type::Vanilla(save_game));
        });

        // Plot
        TabItem::new(im_str!("Plot")).build(ui, || {
            TabBar::new(im_str!("plot-tab")).build(ui, || {
                self.draw_me2_plot_db(&mut save_game.plot, &mut save_game.me1_plot);
            });
        });

        // Head Morph
        TabScroll::new(im_str!("Head Morph")).build(ui, || {
            self.draw_me2_head_morph(
                &mut save_game.player.appearance.head_morph,
                save_game.player.is_female,
            );
        });

        // Raw Data
        TabScroll::new(im_str!("Raw Data")).build(ui, || {
            self.set_next_item_open(true);
            save_game.draw_raw_ui(self, "Mass Effect 2");
        });

        // Raw Plot
        TabItem::new(im_str!("Raw Plot")).build(ui, || {
            if let Some(plot_db) = Database::me2_raw_plot() {
                let PlotTable { booleans, integers, floats, .. } = &mut save_game.plot;
                self.draw_raw_plot(booleans, integers, floats, plot_db, plot_filter);
            }
        });
        Some(())
    }

    pub fn draw_mass_effect_2_le(
        &self, save_game: &mut Me2LeSaveGame, plot_filter: &mut PlotFilterState,
    ) -> Option<()> {
        let ui = self.ui;

        // Tab bar
        let _tab_bar = TabBar::new(im_str!("mass_effect_2_le")).begin(ui)?;

        // General
        TabScroll::new(im_str!("General")).build(ui, || {
            self.draw_me2_general(Me2Type::Legendary(save_game));
        });

        // Plot
        TabItem::new(im_str!("Plot")).build(ui, || {
            TabBar::new(im_str!("plot-tab")).build(ui, || {
                self.draw_me2_plot_db(&mut save_game.plot, &mut save_game.me1_plot);
            });
        });

        // Head Morph
        TabScroll::new(im_str!("Head Morph")).build(ui, || {
            self.draw_head_morph(&mut save_game.player.appearance.head_morph);
        });

        // Raw Data
        TabScroll::new(im_str!("Raw Data")).build(ui, || {
            self.set_next_item_open(true);
            save_game.draw_raw_ui(self, "Mass Effect 2");
        });

        // Raw Plot
        TabItem::new(im_str!("Raw Plot")).build(ui, || {
            if let Some(plot_db) = Database::me2_raw_plot() {
                let PlotTable { booleans, integers, floats, .. } = &mut save_game.plot;
                self.draw_raw_plot(booleans, integers, floats, plot_db, plot_filter);
            }
        });
        Some(())
    }

    fn draw_me2_general(&self, save_game: Me2Type) -> Option<()> {
        let ui = self.ui;

        let (difficulty, end_game_state, player, plot, me1_plot) = match save_game {
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
            }) => (difficulty, end_game_state, player, plot, me1_plot),
        };

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
                }

                ui.same_line();
                self.draw_help_marker(
                                "If you change your gender, disable the head morph or import an appropriate one.\n\
                                Otherwise, the Collectors will be the least of your worries..."
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
                    if let Some(me1_origin) = me1_plot.integers.get_mut(1) {
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
                    if let Some(me1_notoriety) = me1_plot.integers.get_mut(2) {
                        *me1_notoriety = notoriety_idx as i32;
                    }
                }

                Table::next_row();
                face_code.draw_raw_ui(self, "Identity Code");
                ui.same_line();
                self.draw_help_marker("If you change this you can display whatever you want in the menus\nin place of your `Identity Code`, which is pretty cool !");
            });
        });

        // Morality
        Table::new(im_str!("morality-table"), 1).build(ui, || {
            Table::next_row();
            self.set_next_item_open(true);
            TreeNode::new("Morality").build(ui, || {
                if let Some(paragon) = plot.integers.get_mut(2) {
                    Table::next_row();
                    paragon.draw_raw_ui(self, "Paragon");
                }

                if let Some(renegade) = plot.integers.get_mut(3) {
                    Table::next_row();
                    renegade.draw_raw_ui(self, "Renegade");
                }
            });
        });

        // Gameplay
        Table::new(im_str!("gameplay-table"), 1).build(ui, || {
            Table::next_row();
            self.set_next_item_open(true);
            TreeNode::new("Gameplay").build(ui, || {
                Table::next_row();
                self.draw_me2_class(class_name);

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
            });
        });

        // Resources
        Table::new(im_str!("resources-table"), 1).build(ui, || {
            Table::next_row();
            self.set_next_item_open(true);
            TreeNode::new("Resources").build(ui, || {
                Table::next_row();
                eezo.draw_raw_ui(self, "Eezo");

                Table::next_row();
                iridium.draw_raw_ui(self, "Iridium");

                Table::next_row();
                palladium.draw_raw_ui(self, "Palladium");

                Table::next_row();
                platinum.draw_raw_ui(self, "Platinum");

                Table::next_row();
                probes.draw_raw_ui(self, "Probes");

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
                end_game_state.draw_raw_ui(self, "End Game State");
            });
        });

        // Bonus Powers
        self.set_next_item_open(true);
        self.draw_me2_bonus_powers(powers)
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
        let _table = Table::new(im_str!("gameplay-table"), 1).begin(ui)?;

        // Tree node
        Table::next_row();
        let _tree_node = TreeNode::new("Bonus Powers").push(ui)?;
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

    fn draw_me2_plot_db(
        &self, me2_plot_table: &mut PlotTable, me1_plot_table: &mut Me1PlotTable,
    ) -> Option<()> {
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
            imported_me1,
        } = Database::me2_plot()?;
        let PlotTable { booleans, integers, .. } = me2_plot_table;

        // Player
        TabScroll::new(im_str!("Player")).build(ui, || {
            Table::new(im_str!("plot-table"), 1).build(ui, || {
                self.draw_plot_category(booleans, integers, player);
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
                            self.draw_plot_category(booleans, integers, plot_db);
                        });
                    });
                }
            });
        }

        // Rewards
        TabScroll::new(im_str!("Rewards")).build(ui, || {
            Table::new(im_str!("plot-table"), 1).build(ui, || {
                self.draw_plot_category(booleans, integers, rewards);
            });
        });
        // Captain's cabin
        TabScroll::new(im_str!("Captain's cabin")).build(ui, || {
            Table::new(im_str!("plot-table"), 1).build(ui, || {
                self.draw_plot_category(booleans, integers, captains_cabin);
            });
        });

        // Mass Effect 1
        {
            let _colors = self.style_colors(Theme::MassEffect1);

            TabScroll::new(im_str!("Imported ME1")).build(ui, || {
                ui.text("For proper ME3 import change the same plot flags in `Mass Effect 1` tab. Conrad Verner bugfix :");
                ui.same_line();
                self.draw_help_marker(
                    "- Untick `[The Fan] Intimidated him`\n\
                        - Tick `[The Fan] Met Conrad Verner` and `[The Fan] Charmed him`\n\
                        - Only works if you didn't talk to Aethyta",
                );
                ui.separator();
                for (category_name, plot_db) in imported_me1.iter() {
                    Table::new(&im_str!("{}-table", category_name), 1).build(ui, || {
                        Table::next_row();
                        TreeNode::new(category_name).build(ui, || {
                            self.draw_plot_category(booleans, integers, plot_db);
                        });
                    });
                }
            });

            TabItem::new(im_str!("Mass Effect 1")).build(ui, ||  {
                if me1_plot_table.booleans.is_empty() {
                    ui.text("You cannot edit ME1 plot if you have not imported a ME1 save.");
                } else {
                    ui.text(
                        "If you change these plots this will ONLY take effect after an import.",
                    );
                    ui.same_line();
                    self.draw_help_marker("You have to change your end game state to `LiveToFightAgain` then import it to start a new game.\nOr you can start a New Game +.");
                    ui.separator();
                    self.draw_me1_plot_db(me1_plot_table);
                }
            });
        }
        Some(())
    }

    fn draw_me2_head_morph(&self, has_head_morph: &mut Option<HeadMorph>, is_female: bool) {
        let ui = self.ui;

        // Import
        if ui.button(im_str!("Import")) {
            let file =
                tinyfiledialogs::open_file_dialog("", "", Some((&["*.ron"], "Head Morph (*.ron)")));

            if let Some(path) = file {
                let _ = self.event_addr.send(MainEvent::ImportHeadMorph(path));
            }
        }

        if let Some(head_morph) = has_head_morph {
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
            // Remove head morph
            let mut remove = false;
            if !is_female {
                ui.same_line();
                ui.text("-");
                ui.same_line();
                remove = ui.button(im_str!("Remove head morph"));
            }
            ui.separator();

            // Raw
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

            Table::new(im_str!("head-morph-table"), 1).build(ui, || {
                Table::next_row();
                self.set_next_item_open(true);
                TreeNode::new("Raw").build(ui, || {
                    Table::next_row();
                    hair_mesh.draw_raw_ui(self, "Hair Mesh");
                    Table::next_row();
                    accessory_mesh.draw_raw_ui(self, "Accessory Mesh");
                    Table::next_row();
                    morph_features.draw_raw_ui(self, "Morph Features");
                    Table::next_row();
                    offset_bones.draw_raw_ui(self, "Offset Bones");
                    Table::next_row();
                    lod0_vertices.draw_raw_ui(self, "Lod0 Vertices");
                    Table::next_row();
                    lod1_vertices.draw_raw_ui(self, "Lod1 Vertices");
                    Table::next_row();
                    lod2_vertices.draw_raw_ui(self, "Lod2 Vertices");
                    Table::next_row();
                    lod3_vertices.draw_raw_ui(self, "Lod3 Vertices");
                    Table::next_row();
                    scalar_parameters.draw_raw_ui(self, "Scalar Parameters");
                    Table::next_row();
                    vector_parameters.draw_raw_ui(self, "Vector Parameters");
                    Table::next_row();
                    texture_parameters.draw_raw_ui(self, "Texture Parameters");
                });
            });

            // Remove
            if remove {
                *has_head_morph = None;
            }
        } else {
            ui.separator()
        }
    }
}
