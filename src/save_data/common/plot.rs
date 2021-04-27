use anyhow::Result;
use bitvec::prelude::*;
use indexmap::IndexMap;
use serde::{de, Deserialize, Serialize};
use std::{
    fmt,
    ops::{Deref, DerefMut},
};

use crate::{gui::Gui, save_data::RawUi};

pub type BoolSlice = BitSlice<Lsb0, u32>;

#[derive(Clone)]
pub struct BoolVec(BitVec<Lsb0, u32>);

impl Deref for BoolVec {
    type Target = BoolSlice;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BoolVec {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl RawUi for BoolVec {
    fn draw_raw_ui(&mut self, gui: &Gui, ident: &str) {
        gui.draw_boolvec(ident, &mut self.0);
    }
}

impl<'de> serde::Deserialize<'de> for BoolVec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct BoolVecVisitor;
        impl<'de> de::Visitor<'de> for BoolVecVisitor {
            type Value = BoolVec;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a seq")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let mut bitfields: Vec<u32> = Vec::new();
                while let Some(element) = seq.next_element()? {
                    bitfields.push(element);
                }

                let variables = BitVec::from_vec(bitfields);
                Ok(BoolVec(variables))
            }
        }
        deserializer.deserialize_seq(BoolVecVisitor)
    }
}

impl serde::Serialize for BoolVec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let bitfields = self.0.clone().into_vec();
        serializer.collect_seq(bitfields)
    }
}

#[derive(Deserialize, Serialize, RawUi, Clone)]
pub struct Me1PlotTable {
    pub bool_variables: BoolVec,
    pub int_variables: Vec<i32>,
    pub float_variables: Vec<f32>,
}

#[derive(Deserialize, Serialize, RawUi, Default, Clone)]
pub struct PlotCodex {
    pages: Vec<PlotCodexPage>,
}

#[derive(Deserialize, Serialize, RawUi, Default, Clone)]
pub struct PlotCodexPage {
    page: i32,
    is_new: bool,
}

#[derive(Deserialize, Serialize)]
pub struct PlotCategory {
    pub booleans: IndexMap<usize, String>,
    pub ints: IndexMap<usize, String>,
}
