use serde::{Deserialize, Serialize};
use derive_more::Display;

use crate::save_data::shared::player::WeaponLoadout;

use super::player::Power;

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
}
