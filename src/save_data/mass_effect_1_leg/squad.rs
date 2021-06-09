use serde::{Deserialize, Serialize};

use crate::save_data::{Dummy, ImguiString};

use super::player::{ComplexTalent, Item, SimpleTalent};

#[derive(Deserialize, Serialize, RawUi, Default, Clone)]
pub struct Henchman {
    pub tag: ImguiString,
    simple_talents: Vec<SimpleTalent>,
    pub complex_talents: Vec<ComplexTalent>,
    equipped: Vec<Item>,
    quick_slots: Vec<Item>,
    pub talent_points: i32,
    _unknown1: Dummy<4>,
    auto_levelup_template_id: i32,
    localized_last_name: i32,
    localized_class_name: i32,
    _unknown2: Dummy<1>,
    health_per_level: f32,
    stability_current: f32,
    _unknown3: Dummy<6>,
    stamina: i32,
    focus: i32,
    precision: i32,
    coordination: i32,
    _unknown4: Dummy<2>,
    health_current: f32,
    _unknown5: Dummy<4>,
    level: i32,
    _unknown6: Dummy<4>,
    helmet_visible: bool,
    _unknown7: Dummy<1>,
}
