use crate::save_data::{common::player::WeaponLoadout, ImguiString};

use super::player::{Power, Weapon, WeaponMod};

#[derive(SaveData, Default, Clone)]
pub struct Henchman {
    tag: ImguiString,
    powers: Vec<Power>,
    character_level: i32,
    talent_points: i32,
    weapon_loadout: WeaponLoadout,
    mapped_power: ImguiString,
    weapon_mods: Vec<WeaponMod>,
    grenades: i32,
    weapons: Vec<Weapon>,
}
