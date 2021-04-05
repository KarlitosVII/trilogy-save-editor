use anyhow::Result;

use crate::serializer::{SaveCursor, Serializable};

use super::appearance::Appearance;

#[derive(Serializable, Debug)]
pub(super) struct Player {
    is_female: bool,
    class_name: String,
    is_combat_pawn: bool,
    is_injured_pawn: bool,
    use_casual_appearance: bool,
    level: i32,
    current_xp: f32,
    first_name: String,
    last_name: i32,
    origin: Origin,
    notoriety: Notoriety,
    talent_points: i32,
    mapped_power_1: String,
    mapped_power_2: String,
    mapped_power_3: String,
    appearance: Appearance,
    emissive_id: i32,
}

#[derive(FromPrimitive, ToPrimitive, Serializable, Debug)]
enum Origin {
    None = 0,
    Spacer = 1,
    Colony = 2,
    Earthborn = 3,
}

#[derive(FromPrimitive, ToPrimitive, Serializable, Debug)]
enum Notoriety {
    None = 0,
    Survivor = 1,
    Warhero = 2,
    Ruthless = 3,
}

#[derive(Serializable, Debug)]
pub(super) struct Power {
    name: String,
    current_rank: f32,
    evolved_choice_0: i32,
    evolved_choice_1: i32,
    evolved_choice_2: i32,
    evolved_choice_3: i32,
    evolved_choice_4: i32,
    evolved_choice_5: i32,
    power_class_name: String,
    wheel_display_index: i32,
}

#[derive(Serializable, Debug)]
pub(super) struct Weapon {
    class_name: String,
    ammo_used_count: i32,
    ammo_total: i32,
    current_weapon: bool,
    was_last_weapon: bool,
    ammo_power_name: String,
    ammo_power_source_tag: String,
}

#[derive(Serializable, Debug)]
pub(super) struct WeaponMod {
    weapon_class_name: String,
    weapon_mod_class_names: Vec<String>,
}

#[derive(Serializable, Debug)]
pub(super) struct WeaponLoadout {
	assaul_rifle: String,
	shotgun: String,
	sniper_rifle: String,
	submachine_gun: String,
	pistol: String,
	heavy_weapon: String,
}

#[derive(Serializable, Debug)]
pub(super) struct Hotkey {
	pawn_name: String,
	power_name: String,
}
