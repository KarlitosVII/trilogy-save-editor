use serde::{Deserialize, Serialize};

use crate::save_data::{common::player::WeaponLoadout, ImguiString};

use super::player::Power;

#[derive(Deserialize, Serialize, SaveData, Default, Clone)]
pub struct Henchman {
    tag: ImguiString,
    powers: Vec<Power>,
    character_level: i32,
    talent_points: i32,
    weapon_loadout: WeaponLoadout,
    mapped_power: ImguiString,
}
