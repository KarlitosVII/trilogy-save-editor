use serde::{Deserialize, Serialize};

use crate::save_data::{
    shared::{
        appearance::HeadMorph,
        player::{Notoriety, Origin},
    },
    Dummy, ImguiString,
};

// TODO: Tous les u8
#[derive(Deserialize, Serialize, RawUi, Clone)]
pub struct Player {
    pub is_female: bool,
    localized_class_name: i32,
    player_class: u8,
    pub level: i32,
    pub current_xp: f32,
    pub first_name: ImguiString,
    localized_last_name: i32,
    pub origin: Origin,
    pub notoriety: Notoriety,
    specialization_bonus_id: i32,
    spectre_rank: u8,
    pub talent_points: i32,
    talent_pool_points: i32,
    mapped_talent: ImguiString,
    pub head_morph: Option<HeadMorph>,
    simple_talents: Vec<SimpleTalent>,
    pub complex_talents: Vec<ComplexTalent>,
    pub inventory: Inventory,
    pub credits: i32,
    pub medigel: i32,
    pub grenades: f32,
    pub omnigel: f32,
    pub face_code: ImguiString,
    armor_overridden: bool,
    auto_levelup_template_id: i32,
    health_per_level: f32,
    stability_current: f32,
    race: u8,
    toxic_current: f32,
    stamina: i32,
    focus: i32,
    precision: i32,
    coordination: i32,
    attribute_primary: u8,
    attribute_secondary: u8,
    skill_charm: f32,
    skill_intimidate: f32,
    skill_haggle: f32,
    health_current: f32,
    shield_current: f32,
    xp_level: i32,
    is_driving: bool,
    pub game_options: Vec<i32>,
    helmet_shown: bool,
    _unknown: Dummy<5>,
    last_power: ImguiString,
    health_max: f32,
    hotkeys: Vec<Hotkey>,
    primary_weapon: ImguiString,
    secondary_weapon: ImguiString,
}

#[derive(Deserialize, Serialize, RawUi, Clone, Default)]
pub struct SimpleTalent {
    talent_id: i32,
    current_rank: i32,
}

#[derive(Deserialize, Serialize, RawUi, Clone, Default)]
pub struct ComplexTalent {
    talent_id: i32,
    pub current_rank: i32,
    max_rank: i32,
    level_offset: i32,
    levels_per_rank: i32,
    visual_order: i32,
    prereq_talent_ids: Vec<i32>,
    prereq_talent_ranks: Vec<i32>,
}

#[derive(Deserialize, Serialize, RawUi, Clone, Default)]
pub struct Inventory {
    pub equipment: Vec<Item>,
    pub quick_slots: Vec<Item>,
    pub inventory: Vec<Item>,
    buy_pack_items: Vec<Item>, // TODO: Inventory tab
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
pub struct Item {
    pub item_id: i32,
    pub item_level: ItemLevel,
    pub manufacturer_id: i32,
    plot_conditional_id: i32,
    new_item: bool,
    junk: bool,
    attached_mods: Vec<ItemMod>,
}

#[derive(Deserialize, Serialize, RawUi, Clone, Default)]
struct ItemMod {
    item_id: i32,
    item_level: ItemLevel,
    manufacturer_id: i32,
    plot_conditional_id: i32,
}

#[derive(Deserialize, Serialize, RawUi, Clone, Default)]
struct Hotkey {
    pawn: i32,
    event: i32,
}
