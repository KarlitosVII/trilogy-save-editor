use anyhow::Result;
use indexmap::IndexMap;
use serde::{de, Deserialize, Serialize};

use super::shared::{
    plot::{Me1PlotTable, PlotTable},
    Door, EndGameState, Guid, KismetRecord, Level, Rotator, SaveTimeStamp, Vector,
};

pub mod player;
use player::*;

mod squad;
use squad::*;

pub mod plot_db;

mod galaxy_map;
use galaxy_map::*;

#[rc_ize_fields_derive(RawUi)]
#[derive(Deserialize, Serialize, Clone)]
pub struct Me2SaveGame {
    _version: Me2Version,
    debug_name: String,
    seconds_played: f32,
    disc: i32,
    base_level_name: String,
    pub difficulty: Difficulty,
    pub end_game_state: EndGameState,
    timestamp: SaveTimeStamp,
    location: Vector,
    rotation: Rotator,
    current_loading_tip: i32,
    levels: Vec<Level>,
    streaming_records: IndexMap<String, bool>,
    kismet_records: Vec<KismetRecord>,
    doors: Vec<Door>,
    pawns: Vec<Guid>,
    pub player: Player,
    squad: Vec<Henchman>,
    pub plot: PlotTable,
    pub me1_plot: Me1PlotTable,
    galaxy_map: GalaxyMap,
    dependant_dlcs: Vec<DependentDlc>,
}

#[derive(Serialize, Clone)]
pub struct Me2Version(i32);

impl<'de> serde::Deserialize<'de> for Me2Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let version: i32 = serde::Deserialize::deserialize(deserializer)?;

        if version != 29 {
            return Err(de::Error::custom(
                "Wrong save version, please use a save from the latest version of the game",
            ));
        }

        Ok(Self(version))
    }
}

#[rc_ize_fields_derive(RawUi)]
#[derive(Deserialize, Serialize, Clone)]
pub struct Me2LeSaveGame {
    _version: Me2LeVersion,
    debug_name: String,
    seconds_played: f32,
    disc: i32,
    base_level_name: String,
    pub difficulty: Difficulty,
    pub end_game_state: EndGameState,
    timestamp: SaveTimeStamp,
    location: Vector,
    rotation: Rotator,
    current_loading_tip: i32,
    levels: Vec<Level>,
    streaming_records: IndexMap<String, bool>,
    kismet_records: Vec<KismetRecord>,
    doors: Vec<Door>,
    pawns: Vec<Guid>,
    pub player: Player,
    me1_import_rewards: Me1ImportRewards,
    squad: Vec<Henchman>,
    pub plot: PlotTable,
    pub me1_plot: Me1PlotTable,
    galaxy_map: GalaxyMap,
    dependant_dlcs: Vec<DependentDlc>,
}

#[rc_ize_fields_derive(RawUi)]
#[derive(Deserialize, Serialize, Clone)]
struct Me1ImportRewards {
    me1_level: i32,
    me2_level: i32,
    experience: f32,
    credits: f32,
    resources: f32,
    paragon: f32,
    renegade: f32,
}

#[derive(Serialize, Clone)]
pub struct Me2LeVersion(i32);

impl<'de> serde::Deserialize<'de> for Me2LeVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let version: i32 = serde::Deserialize::deserialize(deserializer)?;

        if version != 30 {
            return Err(de::Error::custom(
                "Wrong save version, please use a save from the latest version of the game",
            ));
        }

        Ok(Self(version))
    }
}

#[derive(Deserialize, Serialize, Clone, PartialEq, RawUi)]
pub enum Difficulty {
    Casual,
    Normal,
    Veteran,
    Hardcore,
    Insanity,
}

#[rc_ize_fields_derive(RawUi)]
#[derive(Deserialize, Serialize, Default, Clone)]
struct DependentDlc {
    id: i32,
    name: String,
}

#[rc_ize_fields_derive(RawUi)]
#[derive(Deserialize, Serialize, Default, Clone)]
struct LevelTreasure {
    level_name: String,
    credits: i32,
    xp: i32,
    items: Vec<String>,
}

#[allow(clippy::enum_variant_names)]
#[derive(Deserialize, Serialize, Clone, PartialEq, RawUi)]
enum AutoReplyModeOptions {
    AllDecisions,
    MajorDecisions,
    NoDecisions,
}

#[rc_ize_fields_derive(RawUi)]
#[derive(Deserialize, Serialize, Default, Clone)]
struct ObjectiveMarker {
    marker_owned_data: String,
    marker_offset: Vector,
    marker_label: i32,
    bone_to_attach_to: String,
    marker_icon_type: ObjectiveMarkerIconType,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, RawUi)]
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

#[cfg(test)]
mod test {
    use anyhow::Result;
    use crc::{Crc, CRC_32_BZIP2};
    use std::{fs, time::Instant};

    use crate::unreal;

    use super::*;

    #[test]
    fn deserialize_serialize_vanilla() -> Result<()> {
        let input = fs::read("test/ME2Save.pcsav")?;

        let now = Instant::now();

        // Deserialize
        let me2_save_game: Me2SaveGame = unreal::Deserializer::from_bytes(&input.clone())?;

        println!("Deserialize : {:?}", Instant::now() - now);
        let now = Instant::now();

        // Serialize
        let mut output = unreal::Serializer::to_byte_buf(&me2_save_game)?;

        // Checksum
        let crc = Crc::<u32>::new(&CRC_32_BZIP2);
        let checksum = crc.checksum(&output);
        output.extend(&u32::to_le_bytes(checksum));

        println!("Serialize : {:?}", Instant::now() - now);

        // // Check serialized = input
        // let cmp = input.chunks(4).zip(output.chunks(4));
        // for (i, (a, b)) in cmp.enumerate() {
        //     if a != b {
        //         panic!("0x{:02x?} : {:02x?} != {:02x?}", i * 4, a, b);
        //     }
        // }

        // Check serialized = input
        assert_eq!(input, output);

        Ok(())
    }

    #[test]
    fn deserialize_serialize_legendary() -> Result<()> {
        let input = fs::read("test/ME2LeSave.pcsav")?;

        let now = Instant::now();

        // Deserialize
        let me2_save_game: Me2LeSaveGame = unreal::Deserializer::from_bytes(&input.clone())?;

        println!("Deserialize : {:?}", Instant::now() - now);
        let now = Instant::now();

        // Serialize
        let mut output = unreal::Serializer::to_byte_buf(&me2_save_game)?;

        // Checksum
        let crc = Crc::<u32>::new(&CRC_32_BZIP2);
        let checksum = crc.checksum(&output);
        output.extend(&u32::to_le_bytes(checksum));

        println!("Serialize : {:?}", Instant::now() - now);

        // // Check serialized = input
        // let cmp = input.chunks(4).zip(output.chunks(4));
        // for (i, (a, b)) in cmp.enumerate() {
        //     if a != b {
        //         panic!("0x{:02x?} : {:02x?} != {:02x?}", i * 4, a, b);
        //     }
        // }

        // Check serialized = input
        assert_eq!(input, output);

        Ok(())
    }
}
