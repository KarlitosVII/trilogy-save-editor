use imgui::ImString;

use super::player::{Power, Weapon, WeaponLoadout, WeaponMod};

#[derive(SaveData, Default, Clone)]
pub(super) struct Henchman {
    tag: ImString,
    powers: Vec<Power>,
    character_level: i32,
    talent_points: i32,
    weapon_loadout: WeaponLoadout,
    mapped_power: ImString,
    weapon_mods: Vec<WeaponMod>,
    grenades: i32,
    weapons: Vec<Weapon>,
}
