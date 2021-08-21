mod galaxy_map;
pub mod player;
pub mod plot_db;
mod squad;

use self::{galaxy_map::*, player::*, squad::*};

use anyhow::Result;
use serde::{de, Deserialize, Deserializer, Serialize};

use super::shared::{
    plot::{Codex, Journal, PlotTable},
    Door, EndGameState, Kismet, Level, Rotator, SaveTimeStamp, StreamingState, Vector,
};
use super::Guid;

#[rcize_fields]
#[derive(Deserialize, Serialize, Clone, RawUiRoot)]
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
    streaming_states: Vec<StreamingState>,
    kismet_records: Vec<Kismet>,
    doors: Vec<Door>,
    pawns: Vec<Guid>,
    pub player: Player,
    squad: Vec<Henchman>,
    pub plot: PlotTable,
    journal: Journal,
    codex: Codex,
    pub me1_plot: PlotTable,
    galaxy_map: GalaxyMap,
    dependant_dlcs: Vec<DependentDlc>,
}

#[derive(Serialize, Clone)]
pub struct Me2Version {
    version: i32,
    #[serde(skip)]
    pub is_xbox360: bool,
}

impl<'de> Deserialize<'de> for Me2Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        const GAME_VERSION: i32 = 29;

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

#[rcize_fields]
#[derive(Deserialize, Serialize, Clone, RawUiRoot)]
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
    streaming_states: Vec<StreamingState>,
    kismet_records: Vec<Kismet>,
    doors: Vec<Door>,
    pawns: Vec<Guid>,
    pub player: Player,
    me1_import_bonus: Me1ImportBonus,
    squad: Vec<Henchman>,
    pub plot: PlotTable,
    journal: Journal,
    codex: Codex,
    pub me1_plot: PlotTable,
    galaxy_map: GalaxyMap,
    dependant_dlcs: Vec<DependentDlc>,
}

#[rcize_fields]
#[derive(Deserialize, Serialize, Clone, RawUi)]
struct Me1ImportBonus {
    imported_me1_level: i32,
    starting_me2_level: i32,
    bonus_xp: f32,
    bonus_credits: f32,
    bonus_resources: f32,
    bonus_paragon: f32,
    bonus_renegade: f32,
}

#[derive(Serialize, Clone)]
pub struct Me2LeVersion(i32);

impl<'de> Deserialize<'de> for Me2LeVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let version: i32 = Deserialize::deserialize(deserializer)?;

        if version != 30 {
            return Err(de::Error::custom(
                "Wrong save version, please use a save from the latest version of the game",
            ));
        }

        Ok(Self(version))
    }
}

#[derive(Deserialize, Serialize, Clone, RawUi)]
pub enum Difficulty {
    Casual,
    Normal,
    Veteran,
    Hardcore,
    Insanity,
}

#[rcize_fields]
#[derive(Deserialize, Serialize, Clone, Default, Display, RawUi)]
#[display(fmt = "{}", name)]
struct DependentDlc {
    id: i32,
    name: String,
}

#[cfg(test)]
mod test {
    use std::fs;

    use anyhow::Result;
    use crc::{Crc, CRC_32_BZIP2};

    use super::*;
    use crate::unreal;

    #[test]
    fn deserialize_serialize_vanilla() -> Result<()> {
        let input = fs::read("test/ME2Save.pcsav")?;

        // Deserialize
        let me2_save_game: Me2SaveGame = unreal::Deserializer::from_bytes(&input)?;

        // Serialize
        let mut output = unreal::Serializer::to_vec(&me2_save_game)?;

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
    fn deserialize_serialize_vanilla_xbox360() -> Result<()> {
        let input_pc = fs::read("test/ME2Save.pcsav")?;
        let input_xb360 = fs::read("test/ME2Save360.xbsav")?;

        // Deserialize
        let me2_pc: Me2SaveGame = unreal::Deserializer::from_bytes(&input_pc)?;
        let me2_xb360: Me2SaveGame = unreal::Deserializer::from_be_bytes(&input_xb360)?;

        // Serialize
        let mut output_xb360_to_pc = unreal::Serializer::to_vec(&me2_xb360)?;

        let crc = Crc::<u32>::new(&CRC_32_BZIP2);
        let checksum = crc.checksum(&output_xb360_to_pc);
        output_xb360_to_pc.extend(&u32::to_le_bytes(checksum));

        let mut output_pc_to_xb360 = unreal::Serializer::to_be_vec(&me2_pc)?;

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

    #[test]
    fn deserialize_serialize_legendary() -> Result<()> {
        let input = fs::read("test/ME2LeSave.pcsav")?;

        // Deserialize
        let me2_save_game: Me2LeSaveGame = unreal::Deserializer::from_bytes(&input)?;

        // Serialize
        let mut output = unreal::Serializer::to_vec(&me2_save_game)?;

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
}
