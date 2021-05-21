use serde::{Deserialize, Serialize};

use crate::save_data::{
    shared::{
        appearance::HasHeadMorph,
        player::{Notoriety, Origin},
    },
    Dummy, ImguiString,
};

#[derive(Deserialize, Serialize, RawUi, Clone)]
pub struct Player {
    pub is_female: bool,
    localized_class_name: i32,
    _unknown1: Dummy<1>,
    pub level: i32,
    pub current_xp: f32,
    pub first_name: ImguiString,
    localized_last_name: i32,
    pub origin: Origin,
    pub notoriety: Notoriety,
    _unknown2: Dummy<13>,
    _unknown3: ImguiString,
    pub head_morph: HasHeadMorph,
    simple_talents: Vec<SimpleTalent>,
    complex_talents: Vec<ComplexTalent>,
}

#[derive(Deserialize, Serialize, RawUi, Clone, Default)]
pub struct SimpleTalent {
    talent_id: i32,
    ranks: i32,
}

#[derive(Deserialize, Serialize, RawUi, Clone, Default)]
pub struct ComplexTalent {
    talent_id: i32,
    ranks: i32,
    max_rank: i32,
    level_offset: i32,
    levels_per_rank: i32,
    visual_order: i32,
    prereq_talent_id_array: Vec<i32>,
    prereq_talent_rank_array: Vec<i32>,
}
