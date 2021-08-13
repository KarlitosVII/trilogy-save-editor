pub mod appearance;
pub mod player;
pub mod plot;

use derive_more::Display;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

use super::Guid;

#[derive(Clone, RawUi)]
#[repr(u32)]
pub enum EndGameState {
    NotFinished,
    OutInABlazeOfGlory,
    LivedToFightAgain,
}

impl<'de> Deserialize<'de> for EndGameState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let idx: u32 = Deserialize::deserialize(deserializer)?;

        let end_game_state = match idx {
            0 => EndGameState::NotFinished,
            1 => EndGameState::OutInABlazeOfGlory,
            2 => EndGameState::LivedToFightAgain,
            _ => return Err(de::Error::custom("invalid EndGameState variant")),
        };
        Ok(end_game_state)
    }
}

impl serde::Serialize for EndGameState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(self.clone() as u32)
    }
}

#[rcize_fields]
#[derive(Deserialize, Serialize, Clone, RawUi)]
pub struct SaveTimeStamp {
    seconds_since_midnight: i32,
    day: i32,
    month: i32,
    year: i32,
}

#[rcize_fields]
#[derive(Deserialize, Serialize, Clone, Default, Display, RawUi)]
#[display(fmt = "")]
pub struct Vector {
    x: f32,
    y: f32,
    z: f32,
}

#[rcize_fields]
#[derive(Deserialize, Serialize, Clone, Default, Display, RawUi)]
#[display(fmt = "")]
pub struct Vector2d {
    x: f32,
    y: f32,
}

#[rcize_fields]
#[derive(Deserialize, Serialize, Clone, RawUi)]
pub struct Rotator {
    pitch: i32,
    yaw: i32,
    roll: i32,
}

#[rcize_fields]
#[derive(Deserialize, Serialize, Clone, Default, Display, RawUi)]
#[display(fmt = "{}", name)]
pub struct Level {
    name: String,
    should_be_loaded: bool,
    should_be_visible: bool,
}

#[rcize_fields]
#[derive(Deserialize, Serialize, Clone, Default, Display, RawUi)]
#[display(fmt = "{}", name)]
pub struct StreamingState {
    name: String,
    is_active: bool,
}

#[rcize_fields]
#[derive(Deserialize, Serialize, Clone, Default, Display, RawUi)]
#[display(fmt = "")]
pub struct Kismet {
    guid: Guid,
    value: bool,
}

#[rcize_fields]
#[derive(Deserialize, Serialize, Clone, Default, Display, RawUi)]
#[display(fmt = "")]
pub struct Door {
    guid: Guid,
    current_state: u8,
    old_state: u8,
}
