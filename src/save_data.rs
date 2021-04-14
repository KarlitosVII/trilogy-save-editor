use anyhow::*;
use async_trait::async_trait;
use bincode::{
    config::{AllowTrailing, FixintEncoding, WithOtherIntEncoding, WithOtherTrailing},
    DefaultOptions, Options,
};
use encoding_rs::{UTF_16LE, WINDOWS_1252};
use imgui::{ImStr, ImString};
use indexmap::IndexMap;
use lazy_static::lazy_static;
use num_traits::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use std::{any::type_name, hash::Hash, mem::size_of, usize};

use crate::gui::Gui;

pub mod common;
mod crc32;
pub mod mass_effect_1;
pub mod mass_effect_2;
pub mod mass_effect_3;

lazy_static! {
    pub static ref BINCODE: WithOtherTrailing<WithOtherIntEncoding<DefaultOptions, FixintEncoding>, AllowTrailing> =
        bincode::DefaultOptions::new().with_fixint_encoding().allow_trailing_bytes();
}

pub struct SaveCursor {
    position: usize,
    bytes: Vec<u8>,
}

impl SaveCursor {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { position: 0, bytes }
    }

    pub fn read(&mut self, num_bytes: usize) -> Result<&[u8]> {
        let end = self.position + num_bytes;

        ensure!(end <= self.bytes.len(), "Unexpected end of file, some data in your save are unexpected or your save is corrupted ?\nSave again and retry. If this error persists, please report a bug with your save attached.");

        let slice = &self.bytes[self.position..end];
        self.position = end;

        Ok(slice)
    }

    pub fn read_to_end(&mut self) -> Result<&[u8]> {
        self.read(self.bytes.len() - self.position)
    }

    pub fn rshift_position(&mut self, shift: usize) {
        self.position -= shift;
    }
}

#[async_trait(?Send)]
pub trait SaveData: Sized {
    fn deserialize(cursor: &mut SaveCursor) -> Result<Self>;
    fn serialize(&self, output: &mut Vec<u8>) -> Result<()>;
    async fn draw_raw_ui(&mut self, ui: &Gui, ident: &str);

    // Generic
    fn deserialize_from<'a, D>(input: &'a mut SaveCursor) -> Result<D>
    where
        D: Deserialize<'a>,
    {
        let size = size_of::<D>();
        let bytes = input.read(size)?;

        BINCODE.deserialize::<D>(bytes).map_err(|e| anyhow!(e))
    }

    fn serialize_to<S>(input: &S, output: &mut Vec<u8>) -> Result<()>
    where
        S: Serialize,
    {
        let bytes = BINCODE.serialize::<S>(input)?;
        output.extend(bytes);
        Ok(())
    }

    fn deserialize_from_bool(cursor: &mut SaveCursor) -> Result<bool> {
        Ok(Self::deserialize_from::<i32>(cursor)? != 0)
    }

    fn serialize_to_bool(input: bool, output: &mut Vec<u8>) -> Result<()> {
        Self::serialize_to::<i32>(&(input as i32), output)
    }

    fn deserialize_enum_from_u8<E>(cursor: &mut SaveCursor) -> Result<E>
    where
        E: FromPrimitive,
    {
        E::from_u8(Self::deserialize_from::<u8>(cursor)?).context("invalid enum representation")
    }

    fn serialize_enum_to_u8<E>(input: &E, output: &mut Vec<u8>) -> Result<()>
    where
        E: ToPrimitive,
    {
        Self::serialize_to::<u8>(&E::to_u8(input).context("invalid enum representation")?, output)
    }

    fn deserialize_enum_from_u32<E>(cursor: &mut SaveCursor) -> Result<E>
    where
        E: FromPrimitive,
    {
        E::from_u32(Self::deserialize_from::<u32>(cursor)?).context("invalid enum representation")
    }

    fn serialize_enum_to_u32<E>(input: &E, output: &mut Vec<u8>) -> Result<()>
    where
        E: ToPrimitive,
    {
        Self::serialize_to::<u32>(&E::to_u32(input).context("invalid enum representation")?, output)
    }

    // String
    fn deserialize_from_string(cursor: &mut SaveCursor) -> Result<ImString> {
        let len = Self::deserialize_from::<i32>(cursor)?;

        if len == 0 {
            return Ok(ImString::default());
        }

        let string = if len < 0 {
            // Unicode
            let string_len = (len.abs() * 2) as usize;
            let bytes = cursor.read(string_len)?.to_owned();

            let (decoded, _, had_errors) = UTF_16LE.decode(&bytes);
            ensure!(!had_errors, "UTF_16LE decoding error");

            ImString::new(decoded)
        } else {
            // Ascii
            let string_len = len as usize;
            let bytes = cursor.read(string_len)?.to_owned();

            let (decoded, _, had_errors) = WINDOWS_1252.decode(&bytes);
            ensure!(!had_errors, "WINDOWS_1252 decoding error");

            ImString::new(decoded)
        };

        Ok(string)
    }

    fn serialize_to_string(input: &ImStr, output: &mut Vec<u8>) -> Result<()> {
        if input.is_empty() {
            Self::serialize_to::<u32>(&0, output)?;
            return Ok(());
        }

        let string = input.to_str();
        let (bytes, len) = if string.chars().any(|c| c as u32 > 0xff) {
            // Unicode
            let mut encoded: Vec<_> = string.encode_utf16().collect();
            encoded.push(0);

            let mut bytes = Vec::new();
            for doublebyte in encoded.iter() {
                Self::serialize_to::<u16>(&doublebyte, &mut bytes)?;
            }

            let len = bytes.len() as i32;
            (bytes, -(len / 2))
        } else {
            // Ascii
            let (encoded, _, had_errors) = WINDOWS_1252.encode(&string);
            ensure!(!had_errors, "WINDOWS_1252 encoding error");

            let mut encoded = encoded.into_owned();
            encoded.push(0);

            let len = encoded.len() as i32;
            (encoded, len)
        };

        Self::serialize_to::<i32>(&len, output)?;
        output.extend(bytes);
        Ok(())
    }

    // Array
    fn deserialize_from_array<D>(cursor: &mut SaveCursor) -> Result<Vec<D>>
    where
        D: SaveData,
    {
        let len = Self::deserialize_from::<u32>(cursor)?;
        let mut vec = Vec::new();

        for _ in 0..len {
            vec.push(D::deserialize(cursor)?);
        }
        Ok(vec)
    }

    fn serialize_to_array<D>(input: &[D], output: &mut Vec<u8>) -> Result<()>
    where
        D: SaveData,
    {
        let len = input.len() as u32;
        Self::serialize_to::<u32>(&len, output)?;

        for item in input.iter() {
            D::serialize(item, output)?;
        }
        Ok(())
    }

    // IndexMap
    fn deserialize_from_indexmap<K, V>(cursor: &mut SaveCursor) -> Result<IndexMap<K, V>>
    where
        K: SaveData + Eq + Hash,
        V: SaveData,
    {
        let len = Self::deserialize_from::<u32>(cursor)?;
        let mut map = IndexMap::new();

        for _ in 0..len {
            map.insert(K::deserialize(cursor)?, V::deserialize(cursor)?);
        }
        Ok(map)
    }

    fn serialize_to_indexmap<K, V>(input: &IndexMap<K, V>, output: &mut Vec<u8>) -> Result<()>
    where
        K: SaveData + Eq + Hash,
        V: SaveData,
    {
        let len = input.len() as u32;
        Self::serialize_to::<u32>(&len, output)?;

        for (key, value) in input.iter() {
            K::serialize(key, output)?;
            V::serialize(value, output)?;
        }
        Ok(())
    }
}

// Implémentation des dummy
pub type Dummy<const LEN: usize> = [u8; LEN];

#[async_trait(?Send)]
impl<const LEN: usize> SaveData for Dummy<LEN> {
    fn deserialize(cursor: &mut SaveCursor) -> Result<Self> {
        let mut array = [0; LEN];
        for byte in array.iter_mut() {
            *byte = Self::deserialize_from(cursor)?
        }
        Ok(array)
    }

    fn serialize(&self, output: &mut Vec<u8>) -> Result<()> {
        for byte in self.iter() {
            Self::serialize_to(byte, output)?;
        }
        Ok(())
    }

    async fn draw_raw_ui(&mut self, _: &Gui, _: &str) {}
}

// Implémentation des types std
macro_rules! no_ui_save_data {
    ($type:ty) => {
        #[async_trait(?Send)]
        impl SaveData for $type {
            fn deserialize(cursor: &mut SaveCursor) -> Result<Self> {
                Self::deserialize_from(cursor)
            }

            fn serialize(&self, output: &mut Vec<u8>) -> Result<()> {
                Self::serialize_to(self, output)
            }

            async fn draw_raw_ui(&mut self, _: &Gui, _: &str) {}
        }
    };
}

no_ui_save_data!(u8);
no_ui_save_data!(u32);
no_ui_save_data!(u64);

#[async_trait(?Send)]
impl SaveData for i32 {
    fn deserialize(cursor: &mut SaveCursor) -> Result<Self> {
        Self::deserialize_from(cursor)
    }

    fn serialize(&self, output: &mut Vec<u8>) -> Result<()> {
        Self::serialize_to(self, output)
    }

    async fn draw_raw_ui(&mut self, ui: &Gui, ident: &str) {
        ui.draw_edit_i32(ident, self).await;
    }
}

#[async_trait(?Send)]
impl SaveData for f32 {
    fn deserialize(cursor: &mut SaveCursor) -> Result<Self> {
        Self::deserialize_from(cursor)
    }

    fn serialize(&self, output: &mut Vec<u8>) -> Result<()> {
        Self::serialize_to(self, output)
    }

    async fn draw_raw_ui(&mut self, ui: &Gui, ident: &str) {
        ui.draw_edit_f32(ident, self).await;
    }
}

#[async_trait(?Send)]
impl SaveData for bool {
    fn deserialize(cursor: &mut SaveCursor) -> Result<Self> {
        Self::deserialize_from_bool(cursor)
    }

    fn serialize(&self, output: &mut Vec<u8>) -> Result<()> {
        Self::serialize_to_bool(*self, output)
    }

    async fn draw_raw_ui(&mut self, ui: &Gui, ident: &str) {
        ui.draw_edit_bool(ident, self).await;
    }
}

#[async_trait(?Send)]
impl SaveData for ImString {
    fn deserialize(cursor: &mut SaveCursor) -> Result<Self> {
        Self::deserialize_from_string(cursor)
    }

    fn serialize(&self, output: &mut Vec<u8>) -> Result<()> {
        Self::serialize_to_string(self, output)
    }

    async fn draw_raw_ui(&mut self, ui: &Gui, ident: &str) {
        ui.draw_edit_string(ident, self).await;
    }
}

#[async_trait(?Send)]
impl<T> SaveData for Option<T>
where
    T: SaveData,
{
    fn deserialize(cursor: &mut SaveCursor) -> Result<Self> {
        cursor.rshift_position(4);
        let is_some = Self::deserialize_from_bool(cursor)?;

        let inner = match is_some {
            true => Some(T::deserialize(cursor)?),
            false => None,
        };

        Ok(inner)
    }

    fn serialize(&self, output: &mut Vec<u8>) -> Result<()> {
        if BINCODE.deserialize::<i32>(&output[output.len() - 4..output.len()])? != 0 {
            if let Some(input) = self {
                T::serialize(input, output)?;
            }
        }
        Ok(())
    }

    async fn draw_raw_ui(&mut self, ui: &Gui, ident: &str) {
        if let Some(inner) = self {
            inner.draw_raw_ui(ui, ident).await;
        }
    }
}

#[async_trait(?Send)]
impl<T> SaveData for Vec<T>
where
    T: SaveData + Default,
{
    fn deserialize(cursor: &mut SaveCursor) -> Result<Self> {
        Self::deserialize_from_array(cursor)
    }

    fn serialize(&self, output: &mut Vec<u8>) -> Result<()> {
        Self::serialize_to_array(self, output)
    }

    async fn draw_raw_ui(&mut self, ui: &Gui, ident: &str) {
        // Ignore Dummy
        if type_name::<T>().contains("[u8; ") {
            return;
        }

        ui.draw_vec(ident, self).await;
    }
}

#[async_trait(?Send)]
impl<K, V> SaveData for IndexMap<K, V>
where
    K: SaveData + Eq + Hash + Default,
    V: SaveData + Default,
{
    fn deserialize(cursor: &mut SaveCursor) -> Result<Self> {
        Self::deserialize_from_indexmap(cursor)
    }

    fn serialize(&self, output: &mut Vec<u8>) -> Result<()> {
        Self::serialize_to_indexmap(self, output)
    }

    async fn draw_raw_ui(&mut self, ui: &Gui, ident: &str) {
        ui.draw_indexmap(ident, self).await;
    }
}
