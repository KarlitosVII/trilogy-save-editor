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
    unknown_string: ImguiString,
    pub head_morph: HasHeadMorph,
    simple_talents: Vec<SimpleTalent>,
    complex_talents: Vec<ComplexTalent>,
    inventory: Inventory,
    pub credits: i32,
    pub medigel: i32,
    pub grenades: f32,
    pub salvage: f32,
    pub face_code: ImguiString,
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

#[derive(Deserialize, Serialize, RawUi, Clone, Default)]
pub struct Inventory {
    equipped: Vec<Item>,
    quick_slots: Vec<Item>,
    inventory: Vec<Item>,
    saved_backpack_items: Vec<Item>,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Deserialize, Serialize, RawUi, Clone)]
pub enum ItemLevel {
    None,
    I,
    II,
    III,
    IV,
    V,
    VI,
    VII,
    VIII,
    IX,
    X,
}

impl Default for ItemLevel {
    fn default() -> Self {
        ItemLevel::None
    }
}

#[derive(Deserialize, Serialize, RawUi, Clone, Default)]
struct Item {
    item_id: i32,
    item_level: ItemLevel,
    manufacturer_id: i32,
    _osef: Dummy<4>,
    unknown_bool: bool,
    is_junk: bool,
    attached_mods: Vec<ItemMod>,
}

#[derive(Deserialize, Serialize, RawUi, Clone, Default)]
struct ItemMod {
    item_id: i32,
    item_level: ItemLevel,
    manufacturer_id: i32,
    _osef: Dummy<4>,
}
