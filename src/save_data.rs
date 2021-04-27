use anyhow::{Result, ensure};
use encoding_rs::{UTF_16LE, WINDOWS_1252};
use imgui::ImString;
use indexmap::IndexMap;
use serde::{de, ser::SerializeSeq, Serialize};
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
}

// Save Data
pub trait SaveData: Sized {
    fn deserialize(cursor: &mut SaveCursor) -> Result<Self>;
    fn draw_raw_ui(&mut self, gui: &Gui, ident: &str);
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

    fn draw_raw_ui(&mut self, gui: &Gui, ident: &str) {
        gui.draw_edit_string(ident, &mut self.0);
    }
}

impl<'de> serde::Deserialize<'de> for ImguiString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let string: String = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self(ImString::new(string)))
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

// List<T> : Vec<T> qui se (dé)sérialise sans précision de longueur
#[derive(Clone)]
pub struct List<T>(Vec<T>)
where
    T: Serialize + Clone;

impl<T> Deref for List<T>
where
    T: Serialize + Clone,
{
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for List<T>
where
    T: Serialize + Clone,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<Vec<T>> for List<T>
where
    T: Serialize + Clone,
{
    fn from(from: Vec<T>) -> Self {
        Self(from)
    }
}

impl<T> From<&[T]> for List<T>
where
    T: Serialize + Clone,
{
    fn from(from: &[T]) -> Self {
        Self(from.to_vec())
    }
}

impl<T> serde::Serialize for List<T>
where
    T: Serialize + Clone,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_seq(None)?;
        for element in &self.0 {
            s.serialize_element(element)?;
        }
        s.end()
    }
}

// Implémentation des dummy
#[derive(Clone)]
pub struct Dummy<const LEN: usize>([u8; LEN]);

impl<const LEN: usize> Default for Dummy<LEN> {
    fn default() -> Self {
        Self([0; LEN])
    }
}

impl<const LEN: usize> SaveData for Dummy<LEN> {
    fn deserialize(cursor: &mut SaveCursor) -> Result<Self> {
        let bytes = cursor.read(LEN)?.try_into()?;
        Ok(Self(bytes))
    }

    fn draw_raw_ui(&mut self, _: &Gui, _: &str) {}
}

impl<'de, const LEN: usize> serde::Deserialize<'de> for Dummy<LEN> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct DummyVisitor<const LEN: usize>;
        impl<'de, const LEN: usize> de::Visitor<'de> for DummyVisitor<LEN> {
            type Value = Dummy<LEN>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a seq")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let mut result = [0u8; LEN];
                let mut i = 0;
                while let Some(element) = seq.next_element()? {
                    result[i] = element;
                    i += 1;
                }
                Ok(Dummy(result))
            }
        }
        deserializer.deserialize_tuple(LEN, DummyVisitor)
    }
}

impl<const LEN: usize> serde::Serialize for Dummy<LEN> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(&self.0)
    }
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

macro_rules! impl_save_data_no_ui {
    ($type:ty) => {
        impl SaveData for $type {
            impl_deserialize!($type);

            fn draw_raw_ui(&mut self, _: &Gui, _: &str) {}
        }
    };
}

impl_save_data_no_ui!(u8);
impl_save_data_no_ui!(u32);
impl_save_data_no_ui!(u64);

impl SaveData for i32 {
    impl_deserialize!(i32);

    fn draw_raw_ui(&mut self, gui: &Gui, ident: &str) {
        gui.draw_edit_i32(ident, self);
    }
}

impl SaveData for f32 {
    impl_deserialize!(f32);

    fn draw_raw_ui(&mut self, gui: &Gui, ident: &str) {
        gui.draw_edit_f32(ident, self);
    }
}

impl SaveData for bool {
    fn deserialize(cursor: &mut SaveCursor) -> Result<Self> {
        Ok(<u32 as SaveData>::deserialize(cursor)? != 0)
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

    fn draw_raw_ui(&mut self, gui: &Gui, ident: &str) {
        gui.draw_indexmap(ident, self);
    }
}
