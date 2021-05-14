use serde::{Deserialize, Serialize};

use crate::save_data::{shared::player::WeaponLoadout, ImguiString};

use super::player::Power;

#[derive(Deserialize, Serialize, RawUi, Default, Clone)]
pub struct Henchman {
    tag: ImguiString,
    powers: Vec<Power>,
    character_level: i32,
    talent_points: i32,
    weapon_loadout: WeaponLoadout,
    mapped_power: ImguiString,
}
