use anyhow::Result;
use derive_more::Display;
use indexmap::IndexMap;
use serde::{de, Deserialize, Deserializer, Serialize};

use super::{
    shared::{
        plot::Me1PlotTable, Door, EndGameState, Kismet, Level, Rotator, SaveTimeStamp,
        StreamingState, Vector,
    },
    Guid,
};

pub mod player;
use player::*;

mod squad;
use squad::*;

pub mod plot;
use plot::*;

pub mod plot_db;

mod galaxy_map;
use galaxy_map::*;

#[rcize_fields_derive(RawUiRoot)]
#[derive(Deserialize, Serialize, Clone)]
pub struct Me3SaveGame {
    _version: Me3Version,
    debug_name: String,
    seconds_played: f32,
    disc: i32,
    base_level_name: String,
    base_level_name_display_override_as_read: String,
    pub difficulty: Difficulty,
    pub end_game_state: EndGameState,
    timestamp: SaveTimeStamp,
    location: Vector,
    rotation: Rotator,
    current_loading_tip: i32,
    levels: Vec<Level>,
    streaming_states: Vec<StreamingState>,
    kismet_records: Vec<Kismet>,
    doors: Vec<Door>,
    placeables: Vec<Placeable>,
    pawns: Vec<Guid>,
    pub player: Player,
    squad: Vec<Henchman>,
    pub plot: PlotTable,
    _me1_plot: Me1PlotTable,
    pub player_variables: IndexMap<String, i32>,
    galaxy_map: GalaxyMap,
    dependant_dlcs: Vec<DependentDlc>,
    treasures: Vec<LevelTreasure>,
    use_modules: Vec<Guid>,
    pub conversation_mode: AutoReplyModeOptions,
    objective_markers: Vec<ObjectiveMarker>,
    saved_objective_text: i32,
}

#[derive(Serialize, Clone)]
pub struct Me3Version(i32);

impl<'de> Deserialize<'de> for Me3Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let version: i32 = Deserialize::deserialize(deserializer)?;

        if version != 59 {
            return Err(de::Error::custom(
                "Wrong save version, please use a save from the latest version of the game",
            ));
        }

        Ok(Self(version))
    }
}

#[derive(Deserialize, Serialize, Clone, RawUi)]
pub enum Difficulty {
    Narrative,
    Casual,
    Normal,
    Hardcore,
    Insanity,
}

#[rcize_fields_derive(RawUi)]
#[derive(Deserialize, Serialize, Clone, Default, Display)]
#[display(fmt = "")]
pub struct Placeable {
    guid: Guid,
    is_destroyed: PlaceableState,
    is_deactivated: PlaceableState,
}

#[derive(Deserialize, Serialize, Clone, RawUi)]
pub enum PlaceableState {
    No,
    Yes,
}

impl Default for PlaceableState {
    fn default() -> Self {
        PlaceableState::No
    }
}

#[rcize_fields_derive(RawUi)]
#[derive(Deserialize, Serialize, Clone, Default, Display)]
#[display(fmt = "{}", name)]
struct DependentDlc {
    id: i32,
    name: String,
    canonical_name: String,
}

#[rcize_fields_derive(RawUi)]
#[derive(Deserialize, Serialize, Clone, Default, Display)]
#[display(fmt = "{}", level_name)]
struct LevelTreasure {
    level_name: String,
    credits: i32,
    xp: i32,
    items: Vec<String>,
}

#[allow(clippy::enum_variant_names)]
#[derive(Deserialize, Serialize, Clone, RawUi)]
pub enum AutoReplyModeOptions {
    AllDecisions,
    MajorDecisions,
    NoDecisions,
}

#[rcize_fields_derive(RawUi)]
#[derive(Deserialize, Serialize, Clone, Default, Display)]
#[display(fmt = "")]
struct ObjectiveMarker {
    marker_owned_data: String,
    marker_offset: Vector,
    marker_label: i32,
    bone_to_attach_to: String,
    marker_icon_type: ObjectiveMarkerIconType,
}

#[derive(Deserialize, Serialize, Clone, RawUi)]
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
    fn deserialize_serialize() -> Result<()> {
        let input = fs::read("test/ME3Save.pcsav")?;

        let now = Instant::now();

        // Deserialize
        let me3_save_game: Me3SaveGame = unreal::Deserializer::from_bytes(&input.clone())?;

        println!("Deserialize : {:?}", Instant::now() - now);
        let now = Instant::now();

        // Serialize
        let mut output = unreal::Serializer::to_vec(&me3_save_game)?;

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
