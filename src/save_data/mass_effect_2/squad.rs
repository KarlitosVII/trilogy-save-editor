use derive_more::Display;
use serde::{Deserialize, Serialize};

use crate::save_data::shared::player::WeaponLoadout;

use super::player::Power;

#[rcize_fields]
#[derive(Deserialize, Serialize, Clone, Default, Display, RawUi)]
#[display(fmt = "{}", tag)]
pub struct Henchman {
    tag: String,
    powers: Vec<Power>,
    character_level: i32,
    talent_points: i32,
    weapon_loadout: WeaponLoadout,
    mapped_power: String,
}
