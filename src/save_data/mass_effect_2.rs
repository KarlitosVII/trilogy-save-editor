use anyhow::*;
use async_trait::async_trait;
use imgui::ImString;

use crate::{gui::Gui, save_data::Dummy};

use super::{
    common::{
        plot::Me1PlotTable, Checksum, EndGameState, Level, Rotator, SaveTimeStamp, StreamingRecord,
        Vector,
    },
    SaveCursor, SaveData,
};

mod player;
use player::*;

mod squad;
use squad::*;

pub mod plot;
use plot::*;

pub mod known_plot;

mod galaxy_map;
use galaxy_map::*;

#[derive(SaveData, Clone)]
pub struct Me2SaveGame {
    version: Version,
    debug_name: ImString,
    seconds_played: f32,
    _disc: Dummy<4>,
    base_level_name: ImString,
    difficulty: Difficulty,
    end_game_state: EndGameState,
    timestamp: SaveTimeStamp,
    location: Vector,
    rotation: Rotator,
    _current_loading_tip: Dummy<4>,
    levels: Vec<Level>,
    streaming_records: Vec<StreamingRecord>,
    _kismet_records: Vec<Dummy<20>>,
    _doors: Vec<Dummy<18>>,
    _pawns: Vec<Dummy<16>>,
    player: Player,
    squad: Vec<Henchman>,
    pub plot: PlotTable,
    pub me1_plot: Me1PlotTable,
    galaxy_map: GalaxyMap,
    dependant_dlcs: Vec<DependentDlc>,
    checksum: Checksum,
}

#[derive(Clone)]
pub struct Version(i32);

#[async_trait(?Send)]
impl SaveData for Version {
    fn deserialize(cursor: &mut SaveCursor) -> Result<Self> {
        let version = <i32>::deserialize(cursor)?;

        ensure!(
            version == 29,
            "Wrong save version, please use a save from the last version of the game"
        );

        Ok(Self(version))
    }

    fn serialize(&self, output: &mut Vec<u8>) -> Result<()> {
        <i32>::serialize(&self.0, output)
    }

    async fn draw_raw_ui(&mut self, _: &Gui, _: &str) {}
}

#[derive(SaveData, Clone)]
enum Difficulty {
    Casual,
    Normal,
    Veteran,
    Hardcore,
    Insanity,
}

#[derive(SaveData, Default, Clone)]
struct DependentDlc {
    id: i32,
    name: ImString,
}

#[derive(SaveData, Default, Clone)]
struct LevelTreasure {
    level_name: ImString,
    credits: i32,
    xp: i32,
    items: Vec<ImString>,
}

#[allow(clippy::enum_variant_names)]
#[derive(SaveData, Clone)]
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

#[derive(SaveData, Clone)]
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
    use anyhow::*;
    use std::{
        time::Instant,
        {fs::File, io::Read},
    };

    use crate::save_data::*;

    use super::*;

    #[test]
    fn deserialize_serialize() -> Result<()> {
        let mut input = Vec::new();
        {
            let mut file = File::open("test/ME2Save.pcsav")?;
            file.read_to_end(&mut input)?;
        }

        let now = Instant::now();

        // Deserialize
        let mut cursor = SaveCursor::new(input.clone());
        let me2_save_game = Me2SaveGame::deserialize(&mut cursor)?;

        println!("Deserialize : {:?}", Instant::now().saturating_duration_since(now));
        let now = Instant::now();

        // Serialize
        let mut output = Vec::new();
        Me2SaveGame::serialize(&me2_save_game, &mut output)?;

        println!("Serialize : {:?}", Instant::now().saturating_duration_since(now));

        // Check serialized = input
        let cmp = input.chunks(4).zip(output.chunks(4));
        for (i, (a, b)) in cmp.enumerate() {
            if a != b {
                panic!("0x{:02x?} : {:02x?} != {:02x?}", i * 4, a, b);
            }
        }

        Ok(())
    }
}
