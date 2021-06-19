use serde::{Deserialize, Serialize};

use crate::save_data::{
    shared::{Rotator, Vector},
    Dummy, ImguiString,
};

use super::BaseObject;

#[derive(Deserialize, Serialize, RawUiMe1Legacy, Clone)]
pub struct PawnBehavior {
    is_dead: bool,
    generated_treasure: bool,
    challenge_scaled: bool,
    owner: Box<Option<BaseObject>>,
    health_current: f32,
    shield_current: f32,
    first_name: ImguiString,
    localized_last_name: i32,
    health_max: f32,
    health_regen_rate: f32,
    radar_range: f32,
    level: i32,
    health_per_level: f32,
    stability_current: f32,
    gender: u8,
    race: u8,
    toxic_current: f32,
    stamina: i32,
    focus: i32,
    precision: i32,
    coordination: i32,
    quick_slot_current: u8,
    squad: Box<Option<BaseObject>>,
    inventory: Box<Option<BaseObject>>,
    _unknown: Dummy<3>,
    experience: i32,
    talent_points: i32,
    talent_pool_points: i32,
    attribute_primary: u8,
    attribute_secondary: u8,
    class_base: u8,
    localized_class_name: i32,
    auto_level_up_template_id: i32,
    spectre_rank: u8,
    background_origin: u8,
    background_notoriety: u8,
    specialization_bonus_id: u8,
    skill_charm: f32,
    skill_intimidate: f32,
    skill_haggle: f32,
    audibility: f32,
    blindness: f32,
    damage_duration_mult: f32,
    deafness: f32,
    unlootable_grenade_count: i32,
    head_gear_visible_preference: bool,
    simple_talents: Vec<SimpleTalent>,
    complex_talents: Vec<ComplexTalent>,
    quick_slots: Vec<Option<BaseObject>>,
    equipment: Vec<Option<BaseObject>>,
}

#[derive(Deserialize, Serialize, RawUi, Clone, Default)]
struct SimpleTalent {
    talent_id: i32,
    current_rank: i32,
}

#[derive(Deserialize, Serialize, RawUi, Clone, Default)]
struct ComplexTalent {
    talent_id: i32,
    current_rank: i32,
    max_rank: i32,
    level_offset: i32,
    levels_per_rank: i32,
    visual_order: i32,
    prereq_talent_ids: Vec<i32>,
    prereq_talent_ranks: Vec<i32>,
}

#[derive(Deserialize, Serialize, RawUiMe1Legacy, Clone)]
pub struct Pawn {
    location: Vector,
    rotation: Rotator,
    velocity: Vector,
    acceleration: Vector,
    script_initialized: bool,
    hidden: bool,
    stasis: bool,
    grime_level: f32,
    grime_dirt_level: f32,
    talked_to_count: i32,
    head_gear_visible_preference: bool,
}

#[derive(Deserialize, Serialize, RawUiMe1Legacy, Clone)]
pub struct BaseSquad {
    inventory: Box<Option<BaseObject>>,
}
