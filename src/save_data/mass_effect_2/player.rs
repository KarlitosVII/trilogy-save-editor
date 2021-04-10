use imgui::ImString;

use crate::save_data::common::{
    appearance::Appearance,
    player::{Notoriety, Origin, WeaponLoadout},
};

#[derive(SaveData, Clone)]
pub struct Player {
    is_female: bool,
    class_name: ImString,
    level: i32,
    current_xp: f32,
    first_name: ImString,
    last_name: i32,
    origin: Origin,
    notoriety: Notoriety,
    talent_points: i32,
    mapped_power_1: ImString,
    mapped_power_2: ImString,
    mapped_power_3: ImString,
    appearance: Appearance,
    powers: Vec<Power>,
    weapons: Vec<Weapon>,
    weapons_loadout: WeaponLoadout,
    hotkeys: Vec<Hotkey>,
    credits: i32,
    medigel: i32,
    eezo: i32,
    iridium: i32,
    palladium: i32,
    platinum: i32,
    probes: i32,
    current_fuel: f32,
    face_code: ImString,
    class_friendly_name: i32,
}

#[derive(SaveData, Default, Clone)]
pub struct Power {
    name: ImString,
    rank: f32,
    power_class_name: ImString,
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
