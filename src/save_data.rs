use anyhow::*;
use encoding_rs::{UTF_16LE, WINDOWS_1252};
use imgui::ImString;
use indexmap::IndexMap;
use serde::de;
use std::{
    convert::TryInto,
    fmt::{self, Display},
    hash::Hash,
    mem::size_of,
    ops::{Deref, DerefMut},
};

use crate::gui::Gui;

pub mod common;
pub mod mass_effect_1;
pub mod mass_effect_2;
pub mod mass_effect_3;

pub struct SaveCursor {
    position: usize,
    bytes: Vec<u8>,
}

// Cursor
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

// Save Data
pub trait SaveData: Sized {
    fn deserialize(cursor: &mut SaveCursor) -> Result<Self>;
    fn serialize(&self, output: &mut Vec<u8>) -> Result<()>;
    fn draw_raw_ui(&mut self, gui: &Gui, ident: &str);
}

// Implémentation des dummy
pub type Dummy<const LEN: usize> = [u8; LEN];

impl<const LEN: usize> SaveData for Dummy<LEN> {
    fn deserialize(cursor: &mut SaveCursor) -> Result<Self> {
        let bytes = cursor.read(LEN)?.try_into()?;
        Ok(bytes)
    }

    fn serialize(&self, output: &mut Vec<u8>) -> Result<()> {
        output.extend(self);
        Ok(())
    }

    fn draw_raw_ui(&mut self, _: &Gui, _: &str) {}
}

// Implémentation des types std
macro_rules! impl_deserialize {
    ($type:ty) => {
        fn deserialize(cursor: &mut SaveCursor) -> Result<Self> {
            const SIZE: usize = size_of::<$type>();
            let bytes = cursor.read(SIZE)?;
            Ok(<$type>::from_le_bytes(bytes.try_into()?))
        }
    };
}

macro_rules! impl_serialize {
    ($type:ty) => {
        fn serialize(&self, output: &mut Vec<u8>) -> Result<()> {
            let bytes = <$type>::to_le_bytes(*self);
            output.extend(&bytes);
            Ok(())
        }
    };
}

macro_rules! impl_save_data_no_ui {
    ($type:ty) => {
        impl SaveData for $type {
            impl_deserialize!($type);
            impl_serialize!($type);

            fn draw_raw_ui(&mut self, _: &Gui, _: &str) {}
        }
    };
}

impl_save_data_no_ui!(u8);
impl_save_data_no_ui!(u16);
impl_save_data_no_ui!(u32);
impl_save_data_no_ui!(u64);

impl SaveData for i32 {
    impl_deserialize!(i32);
    impl_serialize!(i32);

    fn draw_raw_ui(&mut self, gui: &Gui, ident: &str) {
        gui.draw_edit_i32(ident, self);
    }
}

impl SaveData for f32 {
    impl_deserialize!(f32);
    impl_serialize!(f32);

    fn draw_raw_ui(&mut self, gui: &Gui, ident: &str) {
        gui.draw_edit_f32(ident, self);
    }
}

impl SaveData for bool {
    fn deserialize(cursor: &mut SaveCursor) -> Result<Self> {
        Ok(<u32 as SaveData>::deserialize(cursor)? != 0)
    }

    fn serialize(&self, output: &mut Vec<u8>) -> Result<()> {
        SaveData::serialize(&(*self as u32), output)
    }

    fn draw_raw_ui(&mut self, gui: &Gui, ident: &str) {
        gui.draw_edit_bool(ident, self);
    }
}

impl<T> SaveData for Vec<T>
where
    T: SaveData + Default,
{
    fn deserialize(cursor: &mut SaveCursor) -> Result<Self> {
        let len: u32 = SaveData::deserialize(cursor)?;
        let mut vec = Vec::new();

        for _ in 0..len {
            vec.push(T::deserialize(cursor)?);
        }
        Ok(vec)
    }

    fn serialize(&self, output: &mut Vec<u8>) -> Result<()> {
        let len = self.len() as u32;
        SaveData::serialize(&len, output)?;

        for item in self.iter() {
            T::serialize(item, output)?;
        }
        Ok(())
    }

    fn draw_raw_ui(&mut self, gui: &Gui, ident: &str) {
        gui.draw_vec(ident, self);
    }
}

impl<K, V> SaveData for IndexMap<K, V>
where
    K: SaveData + Eq + Hash + Default + Display,
    V: SaveData + Default,
{
    fn deserialize(cursor: &mut SaveCursor) -> Result<Self> {
        let len: u32 = SaveData::deserialize(cursor)?;
        let mut map = IndexMap::new();

        for _ in 0..len {
            map.insert(K::deserialize(cursor)?, V::deserialize(cursor)?);
        }
        Ok(map)
    }

    fn serialize(&self, output: &mut Vec<u8>) -> Result<()> {
        let len = self.len() as u32;
        SaveData::serialize(&len, output)?;

        for (key, value) in self.iter() {
            K::serialize(key, output)?;
            V::serialize(value, output)?;
        }
        Ok(())
    }

    fn draw_raw_ui(&mut self, gui: &Gui, ident: &str) {
        gui.draw_indexmap(ident, self);
    }
}

// Nouveau string type pour pouvoir implémenter serde...
#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct ImguiString(ImString);

impl Deref for ImguiString {
    type Target = ImString;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ImguiString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<ImString> for ImguiString {
    fn from(im_string: ImString) -> Self {
        Self(im_string)
    }
}

impl Display for ImguiString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl SaveData for ImguiString {
    fn deserialize(cursor: &mut SaveCursor) -> Result<Self> {
        let len: i32 = SaveData::deserialize(cursor)?;

        if len == 0 {
            return Ok(Self(ImString::default()));
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

        Ok(Self(string))
    }

    fn serialize(&self, output: &mut Vec<u8>) -> Result<()> {
        if self.0.is_empty() {
            SaveData::serialize(&0i32, output)?;
            return Ok(());
        }

        let string = self.0.to_str();
        let (bytes, len) = if string.chars().any(|c| c as u32 > 0xff) {
            // Unicode
            let mut encoded: Vec<u16> = string.encode_utf16().collect();
            encoded.push(0);

            let mut bytes = Vec::new();
            for doublebyte in encoded.drain(..) {
                SaveData::serialize(&doublebyte, &mut bytes)?;
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

        SaveData::serialize(&len, output)?;
        output.extend(bytes);
        Ok(())
    }

    fn draw_raw_ui(&mut self, gui: &Gui, ident: &str) {
        gui.draw_edit_string(ident, &mut self.0);
    }
}

impl<'de> serde::Deserialize<'de> for ImguiString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct StringVisitor;
        impl<'de> de::Visitor<'de> for StringVisitor {
            type Value = ImString;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a &str")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ImString::new(value))
            }
        }

        let result = deserializer.deserialize_str(StringVisitor)?;
        Ok(Self(result))
    }
}

impl serde::Serialize for ImguiString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.0.to_str())
    }
}
