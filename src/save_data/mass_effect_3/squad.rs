use serde::{Deserialize, Serialize};

use crate::save_data::{shared::player::WeaponLoadout, String};

use super::player::{Power, Weapon, WeaponMod};

#[derive(Deserialize, Serialize, RawUi, Default, Clone)]
pub struct Henchman {
    tag: String,
    powers: Vec<Power>,
    character_level: i32,
    talent_points: i32,
    weapon_loadout: WeaponLoadout,
    mapped_power: String,
    weapon_mods: Vec<WeaponMod>,
    grenades: i32,
    weapons: Vec<Weapon>,
}
