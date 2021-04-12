use imgui::ImString;

use crate::save_data::{
    common::{
        appearance::Appearance,
        player::{Notoriety, Origin, WeaponLoadout},
    },
    Dummy,
};

#[derive(SaveData, Clone)]
pub struct Player {
    is_female: bool,
    class_name: ImString,
    is_combat_pawn: bool,
    is_injured_pawn: bool,
    use_casual_appearance: bool,
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
    emissive_id: i32,
    powers: Vec<Power>,
    gaw_assets: Vec<GawAsset>,
    weapons: Vec<Weapon>,
    weapons_mods: Vec<WeaponMod>,
    weapons_loadout: WeaponLoadout,
    primary_weapon: ImString,
    secondary_weapon: ImString,
    loadout_weapon_group: Vec<i32>,
    hotkeys: Vec<Hotkey>,
    current_health: f32,
    credits: i32,
    medigel: i32,
    eezo: i32,
    iridium: i32,
    palladium: i32,
    platinum: i32,
    probes: i32,
    current_fuel: f32,
    grenades: i32,
    face_code: ImString,
    class_friendly_name: i32,
    _character_guid: Dummy<16>,
}

#[derive(SaveData, Default, Clone)]
pub struct Power {
    name: ImString,
    rank: f32,
    evolved_choice_0: i32,
    evolved_choice_1: i32,
    evolved_choice_2: i32,
    evolved_choice_3: i32,
    evolved_choice_4: i32,
    evolved_choice_5: i32,
    power_class_name: ImString,
    wheel_display_index: i32,
}

#[derive(SaveData, Default, Clone)]
pub struct Weapon {
    class_name: ImString,
    ammo_used_count: i32,
    ammo_total: i32,
    current_weapon: bool,
    was_last_weapon: bool,
    ammo_power_name: ImString,
    ammo_power_source_tag: ImString,
}

#[derive(SaveData, Default, Clone)]
pub struct WeaponMod {
    weapon_class_name: ImString,
    weapon_mod_class_names: Vec<ImString>,
}

#[derive(SaveData, Default, Clone)]
struct GawAsset {
    id: i32,
    strength: i32,
}

#[derive(SaveData, Default, Clone)]
struct Hotkey {
    pawn_name: ImString,
    power_name: ImString,
}
