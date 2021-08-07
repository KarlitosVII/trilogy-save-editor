use derive_more::Display;
use serde::{Deserialize, Serialize};

use super::player::{ComplexTalent, Item, SimpleTalent};

#[rcize_fields]
#[derive(Deserialize, Serialize, Clone, Default, Display, RawUi)]
#[display(fmt = "{}", tag)]
pub struct Henchman {
    pub tag: String,
    simple_talents: Vec<SimpleTalent>,
    pub complex_talents: Vec<ComplexTalent>,
    pub equipment: Vec<Item>,
    pub quick_slots: Vec<Item>,
    pub talent_points: i32,
    talent_pool_points: i32,
    auto_levelup_template_id: i32,
    localized_last_name: i32,
    localized_class_name: i32,
    class_base: u8,
    health_per_level: f32,
    stability_current: f32,
    gender: u8,
    race: u8,
    toxic_current: f32,
    stamina: i32,
    focus: i32,
    precision: i32,
    coordination: i32,
    attribute_primary: u8,
    attribute_secondary: u8,
    health_current: f32,
    shield_current: f32,
    level: i32,
    helmet_shown: bool,
    current_quick_slot: u8,
    health_max: f32,
}
