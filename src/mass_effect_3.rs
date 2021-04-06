use anyhow::Result;
use imgui::ImString;
use indexmap::IndexMap;

use crate::{
    save_data::{SaveCursor, SaveData},
    ui::Ui,
};

mod guid;
use guid::*;

mod player;
use player::*;

mod squad;
use squad::*;

mod variables;
use variables::*;

mod galaxy_map;
use galaxy_map::*;

mod appearance;

#[derive(SaveData, Debug)]
pub struct Me3SaveGame {
    version: i32,
    debug_name: Vec<u8>,
    seconds_played: f32,
    disc: [u8; 4],
    base_level_name: ImString,
    base_level_name_display_override_as_read: ImString,
    difficulty: Difficulty,
    end_game_state: EndGameState,
    timestamp: SaveTimeStamp,
    location: Vector,
    rotation: Rotation,
    current_loading_tip: [u8; 4],
    levels: Vec<Level>,
    streaming_records: Vec<StreamingRecord>,
    kismet_records: Vec<[u8; 20]>,
    doors: Vec<[u8; 18]>,
    placeables: Vec<[u8; 18]>,
    pawns: Vec<[u8; 16]>,
    player: Player,
    powers: Vec<Power>,
    gaw_assets: Vec<GawAsset>,
    weapons: Vec<Weapon>,
    weapons_mods: Vec<WeaponMod>,
    weapons_loadout: WeaponLoadout,
    primary_weapon: ImString,
    secondary_weapon: ImString,
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
    face_code: ImString,
    class_friendly_name: i32,
    character_guid: Guid,
    henchmen: Vec<Henchman>,
    plot: PlotTable,
    me1_plot: Me1PlotTable,
    player_variables: IndexMap<ImString, i32>,
    galaxy_map: GalaxyMap,
    dependant_dlcs: Vec<DependentDlc>,
    treasures: Vec<LevelTreasure>,
    use_modules: Vec<[u8; 16]>,
    conversation_mode: AutoReplyModeOptions,
    objectice_markers: Vec<ObjectiveMarker>,
    saved_objective_text: [u8; 4],
    checksum: Checksum,
}

#[derive(Debug)]
struct Checksum(u32);

impl SaveData for Checksum {
    fn deserialize(input: &mut SaveCursor) -> Result<Self> {
        Ok(Self(Self::deserialize_from(input)?))
    }

    fn draw_raw_ui(&mut self, _ui: &Ui, _ident: &str) {}
}

#[derive(FromPrimitive, ToPrimitive, SaveData, Debug)]
enum Difficulty {
    Narrative = 0,
    Casual = 1,
    Normal = 2,
    Hardcore = 3,
    Insanity = 4,
    WhatIsBeyondInsanity = 5,
}

#[derive(FromPrimitive, ToPrimitive, SaveData, Debug)]
enum EndGameState {
    NotFinished = 0,
    OutInABlazeOfGlory = 1,
    LivedToFightAgain = 2,
}

#[derive(SaveData, Debug)]
struct SaveTimeStamp {
    seconds_since_midnight: i32,
    day: i32,
    month: i32,
    year: i32,
}

#[derive(SaveData, Debug)]
struct Vector {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(SaveData, Debug)]
struct Rotation {
    pitch: i32,
    yaw: i32,
    roll: i32,
}

#[derive(SaveData, Debug)]
struct Level {
    name: ImString,
    should_be_loaded: bool,
    should_be_visible: bool,
}

#[derive(SaveData, Debug)]
struct StreamingRecord {
    name: ImString,
    is_active: bool,
}

#[derive(SaveData, Debug)]
struct GawAsset {
    id: i32,
    strength: i32,
}

#[derive(SaveData, Debug)]
struct DependentDlc {
    id: i32,
    name: ImString,
    canonical_name: ImString,
}

#[derive(SaveData, Debug)]
struct LevelTreasure {
    level_name: ImString,
    credits: i32,
    xp: i32,
    items: Vec<ImString>,
}

#[allow(clippy::enum_variant_names)]
#[derive(FromPrimitive, ToPrimitive, SaveData, Debug)]
enum AutoReplyModeOptions {
    AllDecisions = 0,
    MajorDecisions = 1,
    NoDecisions = 2,
}

#[derive(SaveData, Debug)]
struct ObjectiveMarker {
    marker_owned_data: ImString,
    marker_offset: Vector,
    marker_label: i32,
    bone_to_attach_to: ImString,
    marker_icon_type: ObjectiveMarkerIconType,
}

#[derive(FromPrimitive, ToPrimitive, SaveData, Debug)]
enum ObjectiveMarkerIconType {
    None = 0,
    Attack = 1,
    Supply = 2,
    Alert = 3,
}
