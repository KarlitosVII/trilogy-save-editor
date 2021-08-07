use derive_more::Display;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::save_data::{
    shared::{
        appearance::Appearance,
        player::{Notoriety, Origin, WeaponLoadout},
    },
    Dummy,
};

#[rcize_fields]
#[derive(Deserialize, Serialize, Clone, RawUi)]
pub struct Player {
    pub is_female: bool,
    pub class_name: String,
    is_combat_pawn: bool,
    is_injured_pawn: bool,
    use_casual_appearance: bool,
    pub level: i32,
    pub current_xp: f32,
    pub first_name: String,
    localized_last_name: i32,
    pub origin: Origin,
    pub notoriety: Notoriety,
    pub talent_points: i32,
    mapped_power_1: String,
    mapped_power_2: String,
    mapped_power_3: String,
    pub appearance: Appearance,
    emissive_id: i32,
    pub powers: Vec<Power>,
    war_assets: IndexMap<i32, i32>,
    weapons: Vec<Weapon>,
    weapons_mods: Vec<WeaponMod>,
    weapons_loadout: WeaponLoadout,
    primary_weapon: String,
    secondary_weapon: String,
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
    pub face_code: String,
    localized_class_name: i32,
    _character_guid: Dummy<16>,
}

#[rcize_fields]
#[derive(Deserialize, Serialize, Clone, Default, Display, RawUi)]
#[display(fmt = "{}", name)]
pub struct Power {
    name: String,
    rank: f32,
    evolved_choice_0: i32,
    evolved_choice_1: i32,
    evolved_choice_2: i32,
    evolved_choice_3: i32,
    evolved_choice_4: i32,
    evolved_choice_5: i32,
    pub power_class_name: String,
    wheel_display_index: i32,
}

#[rcize_fields]
#[derive(Deserialize, Serialize, Clone, Default, Display, RawUi)]
#[display(fmt = "{}", class_name)]
pub struct Weapon {
    class_name: String,
    ammo_used_count: i32,
    ammo_total: i32,
    current_weapon: bool,
    was_last_weapon: bool,
    ammo_power_name: String,
    ammo_power_source_tag: String,
}

#[rcize_fields]
#[derive(Deserialize, Serialize, Clone, Default, Display, RawUi)]
#[display(fmt = "{}", weapon_class_name)]
pub struct WeaponMod {
    weapon_class_name: String,
    weapon_mod_class_names: Vec<String>,
}

#[rcize_fields]
#[derive(Deserialize, Serialize, Clone, Default, Display, RawUi)]
#[display(fmt = "")]
struct Hotkey {
    pawn_name: String,
    power_name: String,
}
