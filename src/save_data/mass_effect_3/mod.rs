use anyhow::Result;
use indexmap::IndexMap;
use serde::{de, Deserialize, Serialize};

use crate::save_data::Dummy;

use super::{
    shared::{EndGameState, Level, Rotator, SaveTimeStamp, Vector},
    ImguiString,
};

pub mod player;
use player::*;

mod squad;
use squad::*;

pub mod plot;
use plot::*;

pub mod known_plot;

mod galaxy_map;
use galaxy_map::*;

#[derive(Deserialize, Serialize, RawUi, Clone)]
pub struct Me3SaveGame {
    _version: Me3Version,
    _debug_name: ImguiString,
    seconds_played: f32,
    _disc: Dummy<4>,
    base_level_name: ImguiString,
    base_level_name_display_override_as_read: ImguiString,
    pub difficulty: Difficulty,
    pub end_game_state: EndGameState,
    timestamp: SaveTimeStamp,
    location: Vector,
    rotation: Rotator,
    _current_loading_tip: Dummy<4>,
    levels: Vec<Level>,
    streaming_records: IndexMap<ImguiString, bool>,
    _kismet_records: Vec<Dummy<20>>,
    _doors: Vec<Dummy<18>>,
    _placeables: Vec<Dummy<18>>,
    _pawns: Vec<Dummy<16>>,
    pub player: Player,
    squad: Vec<Henchman>,
    pub plot: PlotTable,
    _me1_plot: Me1PlotTable,
    pub player_variables: IndexMap<ImguiString, i32>,
    galaxy_map: GalaxyMap,
    dependant_dlcs: Vec<DependentDlc>,
    treasures: Vec<LevelTreasure>,
    _use_modules: Vec<Dummy<16>>,
    pub conversation_mode: AutoReplyModeOptions,
    objectice_markers: Vec<ObjectiveMarker>,
    _saved_objective_text: Dummy<4>,
}

#[derive(Serialize, Clone)]
pub struct Me3Version(i32);

impl<'de> serde::Deserialize<'de> for Me3Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let version: i32 = serde::Deserialize::deserialize(deserializer)?;

        if version != 59 {
            return Err(de::Error::custom(
                "Wrong save version, please use a save from the latest version of the game",
            ));
        }

        Ok(Self(version))
    }
}

#[derive(Deserialize, Serialize, RawUi, Clone)]
pub enum Difficulty {
    Narrative,
    Casual,
    Normal,
    Hardcore,
    Insanity,
}

#[derive(Deserialize, Serialize, RawUi, Default, Clone)]
struct DependentDlc {
    id: i32,
    name: ImguiString,
    canonical_name: ImguiString,
}

#[derive(Deserialize, Serialize, RawUi, Default, Clone)]
struct LevelTreasure {
    level_name: ImguiString,
    credits: i32,
    xp: i32,
    items: Vec<ImguiString>,
}

#[allow(clippy::enum_variant_names)]
#[derive(Deserialize, Serialize, RawUi, Clone)]
pub enum AutoReplyModeOptions {
    AllDecisions,
    MajorDecisions,
    NoDecisions,
}

#[derive(Deserialize, Serialize, RawUi, Default, Clone)]
struct ObjectiveMarker {
    marker_owned_data: ImguiString,
    marker_offset: Vector,
    marker_label: i32,
    bone_to_attach_to: ImguiString,
    marker_icon_type: ObjectiveMarkerIconType,
}

#[derive(Deserialize, Serialize, RawUi, Clone)]
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
    use std::{
        time::Instant,
        {fs::File, io::Read},
    };

    use crate::unreal;

    use super::*;

    #[test]
    fn deserialize_serialize() -> Result<()> {
        let mut input = Vec::new();
        {
            let mut file = File::open("test/ME3Save.pcsav")?;
            file.read_to_end(&mut input)?;
        }

        let now = Instant::now();

        // Deserialize
        let me3_save_game: Me3SaveGame = unreal::Deserializer::from_bytes(&input.clone())?;

        println!("Deserialize : {:?}", Instant::now().saturating_duration_since(now));
        let now = Instant::now();

        // Serialize
        let mut output = unreal::Serializer::to_byte_buf(&me3_save_game)?;

        // Checksum
        let crc = Crc::<u32>::new(&CRC_32_BZIP2);
        let checksum = crc.checksum(&output);
        output.extend(&u32::to_le_bytes(checksum));

        println!("Serialize : {:?}", Instant::now().saturating_duration_since(now));

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
