use std::ops::{Deref, DerefMut};

use anyhow::*;
use bitvec::prelude::*;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::{
    gui::Gui,
    save_data::{SaveCursor, SaveData},
};

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

impl SaveData for BoolVec {
    fn deserialize(cursor: &mut SaveCursor) -> Result<Self> {
        let bitfields: Vec<u32> = SaveData::deserialize(cursor)?;

        let variables = BitVec::from_vec(bitfields);
        Ok(Self(variables))
    }

    fn draw_raw_ui(&mut self, gui: &Gui, ident: &str) {
        gui.draw_boolvec(ident, &mut self.0);
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

#[derive(Serialize, SaveData, Clone)]
pub struct Me1PlotTable {
    pub bool_variables: BoolVec,
    pub int_variables: Vec<i32>,
    pub float_variables: Vec<f32>,
}

#[derive(Deserialize, Serialize, SaveData, Default, Clone)]
pub struct PlotCodex {
    pages: Vec<PlotCodexPage>,
}

#[derive(Deserialize, Serialize, SaveData, Default, Clone)]
pub struct PlotCodexPage {
    page: i32,
    is_new: bool,
}

#[derive(Deserialize, Serialize)]
pub struct PlotCategory {
    pub booleans: IndexMap<usize, String>,
    pub ints: IndexMap<usize, String>,
}
