use anyhow::{bail, Result};
use imgui::ImString;
use indexmap::IndexMap;

use crate::{
    save_data::{Dummy, SaveCursor, SaveData},
    ui::Ui,
};

mod player;
use player::*;

mod squad;
use squad::*;

pub mod variables;
use variables::*;

mod galaxy_map;
use galaxy_map::*;

mod appearance;

#[derive(SaveData, Clone)]
pub struct Me3SaveGame {
    version: Version,
    debug_name: Vec<Dummy<1>>,
    seconds_played: f32,
    disc: Dummy<4>,
    base_level_name: ImString,
    base_level_name_display_override_as_read: ImString,
    difficulty: Difficulty,
    end_game_state: EndGameState,
    timestamp: SaveTimeStamp,
    location: Vector,
    rotation: Rotation,
    current_loading_tip: Dummy<4>,
    levels: Vec<Level>,
    streaming_records: Vec<StreamingRecord>,
    kismet_records: Vec<Dummy<20>>,
    doors: Vec<Dummy<18>>,
    placeables: Vec<Dummy<18>>,
    pawns: Vec<Dummy<16>>,
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
    character_guid: Dummy<16>,
    henchmen: Vec<Henchman>,
    plot: PlotTable,
    me1_plot: Me1PlotTable,
    player_variables: IndexMap<ImString, i32>,
    galaxy_map: GalaxyMap,
    dependant_dlcs: Vec<DependentDlc>,
    treasures: Vec<LevelTreasure>,
    use_modules: Vec<Dummy<16>>,
    conversation_mode: AutoReplyModeOptions,
    objectice_markers: Vec<ObjectiveMarker>,
    saved_objective_text: Dummy<4>,
    checksum: Checksum,
}

#[derive(Clone)]
struct Version(i32);

impl SaveData for Version {
    fn deserialize(input: &mut SaveCursor) -> Result<Self> {
        let version = Self::deserialize_from(input)?;

        if version != 59 {
            bail!("Wrong save version, please use a save from the last version of the game")
        }

        Ok(Self(version))
    }

    fn serialize(&self, output: &mut Vec<u8>) -> Result<()> {
        Self::serialize_to(&self.0, output)
    }

    fn draw_raw_ui(&mut self, _: &Ui, _: &str) {}
}

#[derive(Clone)]
struct Checksum(u32);

impl SaveData for Checksum {
    fn deserialize(input: &mut SaveCursor) -> Result<Self> {
        Ok(Self(Self::deserialize_from(input)?))
    }

    fn serialize(&self, output: &mut Vec<u8>) -> Result<()> {
        // FIXME: Calculer le checksum
        Self::serialize_to(&self.0, output)
    }

    fn draw_raw_ui(&mut self, _: &Ui, _: &str) {}
}

#[derive(FromPrimitive, ToPrimitive, SaveData, Clone)]
enum Difficulty {
    Narrative,
    Casual,
    Normal,
    Hardcore,
    Insanity,
    WhatIsBeyondInsanity,
}

#[derive(FromPrimitive, ToPrimitive, SaveData, Clone)]
enum EndGameState {
    NotFinished,
    OutInABlazeOfGlory,
    LivedToFightAgain,
}

#[derive(SaveData, Clone)]
struct SaveTimeStamp {
    seconds_since_midnight: i32,
    day: i32,
    month: i32,
    year: i32,
}

#[derive(SaveData, Default, Clone)]
struct Vector {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(SaveData, Clone)]
struct Rotation {
    pitch: i32,
    yaw: i32,
    roll: i32,
}

#[derive(SaveData, Default, Clone)]
struct Level {
    name: ImString,
    should_be_loaded: bool,
    should_be_visible: bool,
}

#[derive(SaveData, Default, Clone)]
struct StreamingRecord {
    name: ImString,
    is_active: bool,
}

#[derive(SaveData, Default, Clone)]
struct GawAsset {
    id: i32,
    strength: i32,
}

#[derive(SaveData, Default, Clone)]
struct DependentDlc {
    id: i32,
    name: ImString,
    canonical_name: ImString,
}

#[derive(SaveData, Default, Clone)]
struct LevelTreasure {
    level_name: ImString,
    credits: i32,
    xp: i32,
    items: Vec<ImString>,
}

#[allow(clippy::enum_variant_names)]
#[derive(FromPrimitive, ToPrimitive, SaveData, Clone)]
enum AutoReplyModeOptions {
    AllDecisions,
    MajorDecisions,
    NoDecisions,
}

#[derive(SaveData, Default, Clone)]
struct ObjectiveMarker {
    marker_owned_data: ImString,
    marker_offset: Vector,
    marker_label: i32,
    bone_to_attach_to: ImString,
    marker_icon_type: ObjectiveMarkerIconType,
}

#[derive(FromPrimitive, ToPrimitive, SaveData, Clone)]
enum ObjectiveMarkerIconType {
    None,
    Attack,
    Supply,
    Alert,
}

impl Default for ObjectiveMarkerIconType {
    fn default() -> Self {
        ObjectiveMarkerIconType::None
    }
}
