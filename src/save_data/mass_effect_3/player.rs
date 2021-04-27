use serde::{Deserialize, Serialize};

use crate::save_data::{
    common::{
        appearance::Appearance,
        player::{Notoriety, Origin, WeaponLoadout},
    },
    Dummy, ImguiString,
};

#[derive(Deserialize, Serialize, RawUi, Clone)]
pub struct Player {
    pub is_female: bool,
    pub class_name: ImguiString,
    is_combat_pawn: bool,
    is_injured_pawn: bool,
    use_casual_appearance: bool,
    pub level: i32,
    pub current_xp: f32,
    pub first_name: ImguiString,
    last_name: i32,
    pub origin: Origin,
    pub notoriety: Notoriety,
    pub talent_points: i32,
    mapped_power_1: ImguiString,
    mapped_power_2: ImguiString,
    mapped_power_3: ImguiString,
    pub appearance: Appearance,
    emissive_id: i32,
    pub powers: Vec<Power>,
    gaw_assets: Vec<GawAsset>,
    weapons: Vec<Weapon>,
    weapons_mods: Vec<WeaponMod>,
    weapons_loadout: WeaponLoadout,
    primary_weapon: ImguiString,
    secondary_weapon: ImguiString,
    loadout_weapon_group: Vec<i32>,
    hotkeys: Vec<Hotkey>,
    current_health: f32,
    pub credits: i32,
    pub medigel: i32,
    eezo: i32,
    iridium: i32,
    palladium: i32,
    platinum: i32,
    probes: i32,
    pub current_fuel: f32,
    pub grenades: i32,
    pub face_code: ImguiString,
    class_friendly_name: i32,
    _character_guid: Dummy<16>,
}

#[derive(Deserialize, Serialize, RawUi, Default, Clone)]
pub struct Power {
    name: ImguiString,
    rank: f32,
    evolved_choice_0: i32,
    evolved_choice_1: i32,
    evolved_choice_2: i32,
    evolved_choice_3: i32,
    evolved_choice_4: i32,
    evolved_choice_5: i32,
    pub power_class_name: ImguiString,
    wheel_display_index: i32,
}

#[derive(Deserialize, Serialize, RawUi, Default, Clone)]
pub struct Weapon {
    class_name: ImguiString,
    ammo_used_count: i32,
    ammo_total: i32,
    current_weapon: bool,
    was_last_weapon: bool,
    ammo_power_name: ImguiString,
    ammo_power_source_tag: ImguiString,
}

#[derive(Deserialize, Serialize, RawUi, Default, Clone)]
pub struct WeaponMod {
    weapon_class_name: ImguiString,
    weapon_mod_class_names: Vec<ImguiString>,
}

#[derive(Deserialize, Serialize, RawUi, Default, Clone)]
struct GawAsset {
    id: i32,
    strength: i32,
}

#[derive(Deserialize, Serialize, RawUi, Default, Clone)]
struct Hotkey {
    pawn_name: ImguiString,
    power_name: ImguiString,
}
