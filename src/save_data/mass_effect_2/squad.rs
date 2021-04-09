use imgui::ImString;

use crate::save_data::common::player::WeaponLoadout;

use super::player::Power;

#[derive(SaveData, Default, Clone)]
pub struct Henchman {
    tag: ImString,
    powers: Vec<Power>,
    character_level: i32,
    talent_points: i32,
    weapon_loadout: WeaponLoadout,
    mapped_power: ImString,
}
