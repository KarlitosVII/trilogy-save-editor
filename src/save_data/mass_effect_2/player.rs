use imgui::ImString;

use crate::save_data::common::{
    appearance::Appearance,
    player::{Notoriety, Origin, WeaponLoadout},
};

#[derive(SaveData, Clone)]
pub struct Player {
    pub is_female: bool,
    pub class_name: ImString,
    pub level: i32,
    pub current_xp: f32,
    pub first_name: ImString,
    last_name: i32,
    pub origin: Origin,
    pub notoriety: Notoriety,
    pub talent_points: i32,
    mapped_power_1: ImString,
    mapped_power_2: ImString,
    mapped_power_3: ImString,
    appearance: Appearance,
    pub powers: Vec<Power>,
    weapons: Vec<Weapon>,
    weapons_loadout: WeaponLoadout,
    hotkeys: Vec<Hotkey>,
    pub credits: i32,
    pub medigel: i32,
    pub eezo: i32,
    pub iridium: i32,
    pub palladium: i32,
    pub platinum: i32,
    pub probes: i32,
    pub current_fuel: f32,
    pub face_code: ImString,
    class_friendly_name: i32,
}

#[derive(SaveData, Default, Clone)]
pub struct Power {
    name: ImString,
    rank: f32,
    pub power_class_name: ImString,
    wheel_display_index: i32,
}

#[derive(SaveData, Default, Clone)]
struct Weapon {
    class_name: ImString,
    ammo_used_count: i32,
    ammo_total: i32,
    current_weapon: bool,
    was_last_weapon: bool,
    ammo_power_name: ImString,
}

#[derive(SaveData, Default, Clone)]
struct Hotkey {
    pawn_name: ImString,
    power_id: i32,
}
