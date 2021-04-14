use anyhow::*;
use async_trait::async_trait;
use crc::{CRC_32_BZIP2, Crc};
use imgui::ImString;

use crate::{
    gui::Gui,
    save_data::{SaveCursor, SaveData},
};

pub mod appearance;
pub mod player;
pub mod plot;

#[derive(Clone)]
pub struct Checksum(u32);

#[async_trait(?Send)]
impl SaveData for Checksum {
    fn deserialize(cursor: &mut SaveCursor) -> Result<Self> {
        Ok(Self(Self::deserialize_from(cursor)?))
    }

    fn serialize(&self, output: &mut Vec<u8>) -> Result<()> {
        let crc = Crc::<u32>::new(&CRC_32_BZIP2);
        Self::serialize_to(&crc.checksum(output), output)
    }

    async fn draw_raw_ui(&mut self, _: &Gui, _: &str) {}
}

#[derive(FromPrimitive, ToPrimitive, SaveData, Clone)]
pub enum EndGameState {
    NotFinished,
    OutInABlazeOfGlory,
    LivedToFightAgain,
}

#[derive(SaveData, Clone)]
pub struct SaveTimeStamp {
    seconds_since_midnight: i32,
    day: i32,
    month: i32,
    year: i32,
}

#[derive(SaveData, Default, Clone)]
pub struct Vector {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(SaveData, Default, Clone)]
pub struct Vector2d {
    x: f32,
    y: f32,
}

#[derive(SaveData, Clone)]
pub struct Rotator {
    pitch: i32,
    yaw: i32,
    roll: i32,
}

#[derive(SaveData, Default, Clone)]
pub struct Level {
    name: ImString,
    should_be_loaded: bool,
    should_be_visible: bool,
}

#[derive(SaveData, Default, Clone)]
pub struct StreamingRecord {
    name: ImString,
    is_active: bool,
}
