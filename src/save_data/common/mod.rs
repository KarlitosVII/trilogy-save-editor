use serde::{Deserialize, Serialize};

use super::ImguiString;

pub mod appearance;
pub mod player;
pub mod plot;

#[derive(SaveData, Clone)]
#[repr(u32)]
pub enum EndGameState {
    NotFinished,
    OutInABlazeOfGlory,
    LivedToFightAgain,
}

impl serde::Serialize for EndGameState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(self.clone() as u32)
    }
}

#[derive(Deserialize, Serialize, SaveData, Clone)]
pub struct SaveTimeStamp {
    seconds_since_midnight: i32,
    day: i32,
    month: i32,
    year: i32,
}

#[derive(Deserialize, Serialize, SaveData, Default, Clone)]
pub struct Vector {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Deserialize, Serialize, SaveData, Default, Clone)]
pub struct Vector2d {
    x: f32,
    y: f32,
}

#[derive(Deserialize, Serialize, SaveData, Clone)]
pub struct Rotator {
    pitch: i32,
    yaw: i32,
    roll: i32,
}

#[derive(Deserialize, Serialize, SaveData, Default, Clone)]
pub struct Level {
    name: ImguiString,
    should_be_loaded: bool,
    should_be_visible: bool,
}

#[derive(Deserialize, Serialize, SaveData, Default, Clone)]
pub struct StreamingRecord {
    name: ImguiString,
    is_active: bool,
}
