use serde::{de, ser, Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

use super::List;

pub mod appearance;
pub mod player;
pub mod plot;

#[derive(Clone, RawUi)]
#[repr(u32)]
pub enum EndGameState {
    NotFinished,
    OutInABlazeOfGlory,
    LivedToFightAgain,
}

impl<'de> serde::Deserialize<'de> for EndGameState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let idx: u32 = serde::Deserialize::deserialize(deserializer)?;

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
        S: serde::Serializer,
    {
        serializer.serialize_u32(self.clone() as u32)
    }
}

#[rc_ize_fields_derive(RawUi)]
#[derive(Deserialize, Serialize, Clone)]
pub struct SaveTimeStamp {
    seconds_since_midnight: i32,
    day: i32,
    month: i32,
    year: i32,
}

#[rc_ize_fields_derive(RawUi)]
#[derive(Deserialize, Serialize, Default, Clone)]
pub struct Vector {
    x: f32,
    y: f32,
    z: f32,
}

#[rc_ize_fields_derive(RawUi)]
#[derive(Deserialize, Serialize, Default, Clone)]
pub struct Vector2d {
    x: f32,
    y: f32,
}

#[rc_ize_fields_derive(RawUi)]
#[derive(Deserialize, Serialize, Clone)]
pub struct Rotator {
    pitch: i32,
    yaw: i32,
    roll: i32,
}

#[rc_ize_fields_derive(RawUi)]
#[derive(Deserialize, Serialize, Default, Clone)]
pub struct Level {
    name: String,
    should_be_loaded: bool,
    should_be_visible: bool,
}

#[derive(Clone)]
pub struct Guid {
    pub part1: String,
    pub part2: String,
    pub part3: String,
    pub part4: String,
    pub part5: String,
}

impl Default for Guid {
    fn default() -> Self {
        let mut part1 = String::with_capacity(8);
        part1.push_str("00000000");
        let mut part2 = String::with_capacity(4);
        part2.push_str("0000");
        let mut part3 = String::with_capacity(4);
        part3.push_str("0000");
        let mut part4 = String::with_capacity(4);
        part4.push_str("0000");
        let mut part5 = String::with_capacity(12);
        part5.push_str("000000000000");

        Guid { part1, part2, part3, part4, part5 }
    }
}

impl<'de> serde::Deserialize<'de> for Guid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct GuidVisitor;
        impl<'de> de::Visitor<'de> for GuidVisitor {
            type Value = Guid;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a Guid")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let mut result = [0u8; 16];
                let mut i = 0;
                while let Some(element) = seq.next_element()? {
                    result[i] = element;
                    i += 1;
                }

                let guid =
                    Uuid::from_slice(&result).map_err(de::Error::custom)?.to_simple().to_string();

                let mut part1 = String::with_capacity(8);
                part1.push_str(&guid[0..8]);
                let mut part2 = String::with_capacity(4);
                part2.push_str(&guid[8..12]);
                let mut part3 = String::with_capacity(4);
                part3.push_str(&guid[12..16]);
                let mut part4 = String::with_capacity(4);
                part4.push_str(&guid[16..20]);
                let mut part5 = String::with_capacity(12);
                part5.push_str(&guid[20..32]);

                Ok(Guid { part1, part2, part3, part4, part5 })
            }
        }
        deserializer.deserialize_tuple_struct("Guid", 16, GuidVisitor)
    }
}

impl serde::Serialize for Guid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let guid_str =
            self.part1.to_owned() + &self.part2 + &self.part3 + &self.part4 + &self.part5;
        let guid = List(
            Uuid::parse_str(&guid_str)
                .map_err(|err| ser::Error::custom(format!("GUID: {}", err)))?
                .as_bytes()
                .to_vec(),
        );
        serde::Serialize::serialize(&guid, serializer)
    }
}

#[rc_ize_fields_derive(RawUi)]
#[derive(Deserialize, Serialize, Default, Clone)]
pub struct Kismet {
    guid: Guid,
    value: bool,
}

#[rc_ize_fields_derive(RawUi)]
#[derive(Deserialize, Serialize, Default, Clone)]
pub struct Door {
    guid: Guid,
    current_state: u8,
    old_state: u8,
}
