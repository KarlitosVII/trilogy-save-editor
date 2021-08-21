mod galaxy_map;
pub mod player;
pub mod plot;
pub mod plot_db;
mod squad;

use self::{galaxy_map::*, player::*, plot::*, squad::*};

use anyhow::Result;
use indexmap::IndexMap;
use serde::{de, Deserialize, Deserializer, Serialize};

use super::shared::{
    plot::PlotTable as Me1PlotTable, Door, EndGameState, Kismet, Level, Rotator, SaveTimeStamp,
    StreamingState, Vector,
};
use super::Guid;

#[rcize_fields]
#[derive(Deserialize, Serialize, Clone, RawUiRoot)]
pub struct Me3SaveGame {
    _version: Me3Version,
    debug_name: String,
    seconds_played: f32,
    disc: i32,
    base_level_name: String,
    base_level_name_display_override: String,
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
    journal: Journal,
    codex: Codex,
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
pub struct Me3Version {
    version: i32,
    #[serde(skip)]
    pub is_xbox360: bool,
}

impl<'de> Deserialize<'de> for Me3Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        const GAME_VERSION: i32 = 59;

        let bytes: [u8; 4] = Deserialize::deserialize(deserializer)?;
        let version_le = i32::from_le_bytes(bytes);
        let version_be = i32::from_be_bytes(bytes);

        if version_le == GAME_VERSION {
            Ok(Self { version: version_le, is_xbox360: false })
        } else if version_be == GAME_VERSION {
            Ok(Self { version: version_be, is_xbox360: true })
        } else {
            Err(de::Error::custom(
                "Wrong save version, please use a save from the latest version of the game",
            ))
        }
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

#[rcize_fields]
#[derive(Deserialize, Serialize, Clone, Default, Display, RawUi)]
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

#[rcize_fields]
#[derive(Deserialize, Serialize, Clone, Default, Display, RawUi)]
#[display(fmt = "{}", name)]
struct DependentDlc {
    id: i32,
    name: String,
    canonical_name: String,
}

#[rcize_fields]
#[derive(Deserialize, Serialize, Clone, Default, Display, RawUi)]
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

#[rcize_fields]
#[derive(Deserialize, Serialize, Clone, Default, Display, RawUi)]
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
    use std::fs;

    use anyhow::Result;
    use crc::{Crc, CRC_32_BZIP2};

    use super::*;
    use crate::unreal;

    #[test]
    fn deserialize_serialize() -> Result<()> {
        let input = fs::read("test/ME3Save.pcsav")?;

        // Deserialize
        let me3_save_game: Me3SaveGame = unreal::Deserializer::from_bytes(&input)?;

        // Serialize
        let mut output = unreal::Serializer::to_vec(&me3_save_game)?;

        // Checksum
        let crc = Crc::<u32>::new(&CRC_32_BZIP2);
        let checksum = crc.checksum(&output);
        output.extend(&u32::to_le_bytes(checksum));

        // // Check serialized = input
        // let cmp = input.chunks(4).zip(output.chunks(4));
        // for (i, (a, b)) in cmp.enumerate() {
        //     if a != b {
        //         panic!("0x{:02x?} : {:02x?} != {:02x?}", i * 4, a, b);
        //     }
        // }

        // Check serialized = input
        assert!(input == output);

        Ok(())
    }

    #[test]
    fn deserialize_serialize_xbox360() -> Result<()> {
        let input_pc = fs::read("test/ME3Save.pcsav")?;
        let input_xb360 = fs::read("test/ME3Save360.xbsav")?;

        // Deserialize
        let me3_pc: Me3SaveGame = unreal::Deserializer::from_bytes(&input_pc)?;
        let me3_xb360: Me3SaveGame = unreal::Deserializer::from_be_bytes(&input_xb360)?;

        // Serialize
        let mut output_xb360_to_pc = unreal::Serializer::to_vec(&me3_xb360)?;

        let crc = Crc::<u32>::new(&CRC_32_BZIP2);
        let checksum = crc.checksum(&output_xb360_to_pc);
        output_xb360_to_pc.extend(&u32::to_le_bytes(checksum));

        let mut output_pc_to_xb360 = unreal::Serializer::to_be_vec(&me3_pc)?;

        let crc = Crc::<u32>::new(&CRC_32_BZIP2);
        let checksum = crc.checksum(&output_pc_to_xb360);
        output_pc_to_xb360.extend(&u32::to_be_bytes(checksum));

        // // Check serialized = input
        // let cmp = input.chunks(4).zip(output.chunks(4));
        // for (i, (a, b)) in cmp.enumerate() {
        //     if a != b {
        //         panic!("0x{:02x?} : {:02x?} != {:02x?}", i * 4, a, b);
        //     }
        // }

        // Check serialized = input
        assert!(input_pc == output_xb360_to_pc);
        assert!(input_xb360 == output_pc_to_xb360);

        Ok(())
    }
}
