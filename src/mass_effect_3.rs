use anyhow::Result;
use indexmap::IndexMap;

use crate::serializer::{SaveCursor, Serializable};

mod guid;
use guid::*;

mod uninteresting;
use uninteresting::*;

mod player;
use player::*;

mod squad;
use squad::*;

mod variables;
use variables::*;

mod galaxy_map;
use galaxy_map::*;

mod appearance;

#[derive(Serializable, Debug)]
pub struct Me3SaveGame {
    version: i32,
    debug_name: String,
    seconds_played: f32,
    disc: i32,
    base_level_name: String,
    base_level_name_display_override_as_read: String,
    difficulty: Difficulty,
    end_game_state: EndGameState,
    timestamp: SaveTimeStamp,
    location: Vector,
    rotation: Rotation,
    current_loading_tip: i32,
    levels: Vec<Level>,
    streaming_records: Vec<StreamingRecord>,
    kismet_records: Vec<Dummy20Bytes>,
    doors: Vec<Dummy18Bytes>,
    placeables: Vec<Dummy18Bytes>,
    pawns: Vec<Dummy16Bytes>,
    player: Player,
    powers: Vec<Power>,
    gaw_assets: Vec<GawAsset>,
    weapons: Vec<Weapon>,
    weapons_mods: Vec<WeaponMod>,
    weapons_loadout: WeaponLoadout,
    primary_weapon: String,
    secondary_weapon: String,
    loadout_weapon_group: Vec<i32>,
    hotkeys: Vec<Hotkey>,
    current_health: f32,
    credits: i32,
    medigel: i32,
    eezo: i32,
    iridium: i32,
    palladium: i32,
    platinum: i32,
    probes: i32,
    current_fuel: f32,
    grenades: i32,
    face_code: String,
    class_friendly_name: i32,
    character_guid: Guid,
    henchmen: Vec<Henchman>,
    plot: PlotTable,
    me1_plot: Me1PlotTable,
    player_variables: IndexMap<String, i32>,
    galaxy_map: GalaxyMap,
    dependant_dlcs: Vec<DependentDlc>,
    treasures: Vec<LevelTreasure>,
    use_modules: Vec<Guid>,
    conversation_mode: AutoReplyModeOptions,
    objectice_markers: Vec<ObjectiveMarker>,
    saved_objective_text: i32,
    checksum: u32,
}

#[derive(FromPrimitive, ToPrimitive, Serializable, Debug)]
enum Difficulty {
    Narrative = 0,
    Casual = 1,
    Normal = 2,
    Hardcore = 3,
    Insanity = 4,
    WhatIsBeyondInsanity = 5,
}

#[derive(FromPrimitive, ToPrimitive, Debug)]
enum EndGameState {
    NotFinished = 0,
    OutInABlazeOfGlory = 1,
    LivedToFightAgain = 2,
}

impl Serializable for EndGameState {
    fn deserialize(input: &mut SaveCursor) -> Result<Self> {
        Self::deserialize_enum_from_u32(input)
    }
}

#[derive(Serializable, Debug)]
struct SaveTimeStamp {
    seconds_since_midnight: i32,
    day: i32,
    month: i32,
    year: i32,
}

#[derive(Serializable, Debug)]
struct Vector {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Serializable, Debug)]
struct Rotation {
    pitch: i32,
    yaw: i32,
    roll: i32,
}

#[derive(Serializable, Debug)]
struct GawAsset {
    id: i32,
    strength: i32,
}

#[derive(Serializable, Debug)]
struct DependentDlc {
	id: i32,
	name: String,
	canonical_name: String,
}

#[derive(Serializable, Debug)]
struct LevelTreasure {
	level_name: String,
	credits: i32,
	xp: i32,
	items: Vec<String>,
}

#[derive(FromPrimitive, ToPrimitive, Serializable, Debug)]
enum AutoReplyModeOptions {
	AllDecisions = 0,
	MajorDecisions = 1,
	NoDecisions = 2,
}

#[derive(Serializable, Debug)]
struct ObjectiveMarker {
	marker_owned_data: String,
	marker_offset: Vector,
	marker_label: i32,
	bone_to_attach_to: String,
	marker_icin_type: ObjectiveMarkerIconType,
}

#[derive(FromPrimitive, ToPrimitive, Serializable, Debug)]
enum ObjectiveMarkerIconType {
	None = 0,
	Attack = 1,
	Supply = 2,
	Alert = 3,
}
