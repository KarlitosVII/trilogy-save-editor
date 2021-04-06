use anyhow::Result;

use crate::serializer::{SaveCursor, SaveData};

#[derive(SaveData, Debug)]
pub(super) struct Level {
    name: String,
    should_be_loaded: bool,
    should_be_visible: bool,
}

#[derive(SaveData, Debug)]
pub(super) struct StreamingRecord {
    name: String,
    is_active: bool,
}

#[derive(Debug)]
pub(super) struct Dummy16Bytes(Vec<u8>);
impl SaveData for Dummy16Bytes {
    fn deserialize(input: &mut SaveCursor) -> Result<Self> {
        let vec = input.read(16)?.to_owned();
        Ok(Self(vec))
    }
}

#[derive(Debug)]
pub(super) struct Dummy18Bytes(Vec<u8>);
impl SaveData for Dummy18Bytes {
    fn deserialize(input: &mut SaveCursor) -> Result<Self> {
        let vec = input.read(18)?.to_owned();
        Ok(Self(vec))
    }
}

#[derive(Debug)]
pub(super) struct Dummy20Bytes(Vec<u8>);
impl SaveData for Dummy20Bytes {
    fn deserialize(input: &mut SaveCursor) -> Result<Self> {
        let vec = input.read(20)?.to_owned();
        Ok(Self(vec))
    }
}
