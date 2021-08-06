use derive_more::Display;
use serde::{Deserialize, Serialize};

use crate::save_data::shared::player::WeaponLoadout;

use super::player::{Power, Weapon, WeaponMod};

#[rcize_fields_derive(RawUi)]
#[derive(Deserialize, Serialize, Clone, Default, Display)]
#[display(fmt = "{}", tag)]
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
