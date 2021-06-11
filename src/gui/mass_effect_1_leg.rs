use if_chain::if_chain;
use imgui::{im_str, ChildWindow, ImStr, TabBar, TabItem};

use crate::save_data::{
    mass_effect_1_leg::{self, player::ComplexTalent, squad::Henchman, Me1LegSaveData},
    shared::player::{Notoriety, Origin},
    RawUi,
};

use super::{Gui, PlotDbsState};

impl<'ui> Gui<'ui> {
    pub fn draw_mass_effect_1_leg(
        &self, save_game: &mut Me1LegSaveData, plot_dbs: &PlotDbsState,
    ) -> Option<()> {
        let ui = self.ui;

        // Tab bar
        let _t = TabBar::new(im_str!("mass_effect_1_leg")).begin(ui)?;

        // General
        if_chain! {
            if let Some(_t) = TabItem::new(im_str!("General")).begin(ui);
            if let Some(_t) = ChildWindow::new(im_str!("scroll")).begin(ui);
            then {
                self.draw_me1_leg_general(save_game);
            }
        }
        // Plot
        if_chain! {
            if let Some(_t) = TabItem::new(im_str!("Plot")).begin(ui);
            if let Some(me1_plot_db) = &plot_dbs.me1;
            then {
                self.draw_me1_plot_db(&mut save_game.plot, me1_plot_db);
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

    fn draw_me1_leg_general(&self, save_game: &mut Me1LegSaveData) -> Option<()> {
        let ui = self.ui;
        let Me1LegSaveData { plot, player, difficulty, squad, .. } = save_game;
        let mass_effect_1_leg::player::Player {
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
                self.draw_me1_reset_talents("player", talent_points, complex_talents);
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
                    self.draw_me1_reset_talents(character_name, talent_points, complex_talents);
                }
            }
        }

        Some(())
    }

    fn draw_me1_reset_talents(
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
}
