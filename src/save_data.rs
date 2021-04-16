use anyhow::*;
use async_trait::async_trait;
use encoding_rs::{UTF_16LE, WINDOWS_1252};
use imgui::ImString;
use indexmap::IndexMap;
use std::{any::type_name, convert::TryInto, hash::Hash, mem::size_of, usize};

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
#[async_trait(?Send)]
pub trait SaveData: Sized {
    fn deserialize(cursor: &mut SaveCursor) -> Result<Self>;
    fn serialize(&self, output: &mut Vec<u8>) -> Result<()>;
    async fn draw_raw_ui(&mut self, gui: &Gui, ident: &str);
}

// Implémentation des dummy
pub type Dummy<const LEN: usize> = [u8; LEN];

#[async_trait(?Send)]
impl<const LEN: usize> SaveData for Dummy<LEN> {
    fn deserialize(cursor: &mut SaveCursor) -> Result<Self> {
        let bytes = cursor.read(LEN)?.try_into()?;
        Ok(bytes)
    }

    fn serialize(&self, output: &mut Vec<u8>) -> Result<()> {
        output.extend(self);
        Ok(())
    }

    async fn draw_raw_ui(&mut self, _: &Gui, _: &str) {}
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
        #[async_trait(?Send)]
        impl SaveData for $type {
            impl_deserialize!($type);
            impl_serialize!($type);

            async fn draw_raw_ui(&mut self, _: &Gui, _: &str) {}
        }
    };
}

impl_save_data_no_ui!(u8);
impl_save_data_no_ui!(u16);
impl_save_data_no_ui!(u32);
impl_save_data_no_ui!(u64);

#[async_trait(?Send)]
impl SaveData for i32 {
    impl_deserialize!(i32);
    impl_serialize!(i32);

    async fn draw_raw_ui(&mut self, ui: &Gui, ident: &str) {
        ui.draw_edit_i32(ident, self).await;
    }
}

#[async_trait(?Send)]
impl SaveData for f32 {
    impl_deserialize!(f32);
    impl_serialize!(f32);

    async fn draw_raw_ui(&mut self, ui: &Gui, ident: &str) {
        ui.draw_edit_f32(ident, self).await;
    }
}

#[async_trait(?Send)]
impl SaveData for bool {
    fn deserialize(cursor: &mut SaveCursor) -> Result<Self> {
        Ok(<u32>::deserialize(cursor)? != 0)
    }

    fn serialize(&self, output: &mut Vec<u8>) -> Result<()> {
        <u32>::serialize(&(*self as u32), output)
    }

    async fn draw_raw_ui(&mut self, ui: &Gui, ident: &str) {
        ui.draw_edit_bool(ident, self).await;
    }
}

#[async_trait(?Send)]
impl SaveData for ImString {
    fn deserialize(cursor: &mut SaveCursor) -> Result<Self> {
        let len = <i32>::deserialize(cursor)?;

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

    fn serialize(&self, output: &mut Vec<u8>) -> Result<()> {
        if self.is_empty() {
            <i32>::serialize(&0, output)?;
            return Ok(());
        }

        let string = self.to_str();
        let (bytes, len) = if string.chars().any(|c| c as u32 > 0xff) {
            // Unicode
            let mut encoded: Vec<_> = string.encode_utf16().collect();
            encoded.push(0);

            let mut bytes = Vec::new();
            for doublebyte in encoded.iter() {
                <u16>::serialize(doublebyte, &mut bytes)?;
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

        <i32>::serialize(&len, output)?;
        output.extend(bytes);
        Ok(())
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
        let is_some = <bool>::deserialize(cursor)?;

        let inner = match is_some {
            true => Some(T::deserialize(cursor)?),
            false => None,
        };
        Ok(inner)
    }

    fn serialize(&self, output: &mut Vec<u8>) -> Result<()> {
        let bytes = &output[output.len() - 4..output.len()];
        if <u32>::from_le_bytes(bytes.try_into()?) != 0 {
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
        let len = <u32>::deserialize(cursor)?;
        let mut vec = Vec::new();

        for _ in 0..len {
            vec.push(T::deserialize(cursor)?);
        }
        Ok(vec)
    }

    fn serialize(&self, output: &mut Vec<u8>) -> Result<()> {
        let len = self.len() as u32;
        <u32>::serialize(&len, output)?;

        for item in self.iter() {
            T::serialize(item, output)?;
        }
        Ok(())
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
        let len = <u32>::deserialize(cursor)?;
        let mut map = IndexMap::new();

        for _ in 0..len {
            map.insert(K::deserialize(cursor)?, V::deserialize(cursor)?);
        }
        Ok(map)
    }

    fn serialize(&self, output: &mut Vec<u8>) -> Result<()> {
        let len = self.len() as u32;
        <u32>::serialize(&len, output)?;

        for (key, value) in self.iter() {
            K::serialize(key, output)?;
            V::serialize(value, output)?;
        }
        Ok(())
    }

    async fn draw_raw_ui(&mut self, ui: &Gui, ident: &str) {
        ui.draw_indexmap(ident, self).await;
    }
}
