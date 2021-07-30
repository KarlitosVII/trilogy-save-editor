use serde::{Deserialize, Serialize};

use crate::save_data::shared::player::WeaponLoadout;

use super::player::Power;

#[rc_ize_fields_derive(RawUi)]
#[derive(Deserialize, Serialize, Clone, Default)]
pub struct Henchman {
    tag: String,
    powers: Vec<Power>,
    character_level: i32,
    talent_points: i32,
    weapon_loadout: WeaponLoadout,
    mapped_power: String,
}
