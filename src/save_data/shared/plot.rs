use anyhow::Result;
use bitvec::prelude::*;
use derive_more::{Deref, DerefMut, Display};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

pub type BoolSlice = BitSlice<Lsb0, u32>;

#[derive(Deref, DerefMut, Clone)]
pub struct BoolVec(BitVec<Lsb0, u32>);

impl<'de> serde::Deserialize<'de> for BoolVec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bitfields: Vec<u32> = serde::Deserialize::deserialize(deserializer)?;
        let bitvec = BitVec::from_vec(bitfields);
        Ok(BoolVec(bitvec))
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

#[rcize_fields_derive(RawUi)]
#[derive(Deserialize, Serialize, Clone)]
pub struct Me1PlotTable {
    pub booleans: BoolVec,
    pub integers: Vec<i32>,
    pub floats: Vec<f32>,
}

#[rcize_fields_derive(RawUi)]
#[derive(Deserialize, Serialize, Clone)]
pub struct PlotTable {
    pub booleans: BoolVec,
    pub integers: Vec<i32>,
    pub floats: Vec<f32>,
    quest_progress_counter: i32,
    quest_progress: Vec<PlotQuest>,
    quest_ids: Vec<i32>,
    codex_entries: Vec<PlotCodex>,
    codex_ids: Vec<i32>,
}

#[rcize_fields_derive(RawUi)]
#[derive(Deserialize, Serialize, Default, Clone, Display)]
#[display(fmt = "{}", quest_counter)]
pub struct PlotQuest {
    quest_counter: i32,
    quest_updated: bool,
    history: Vec<i32>,
}

#[rcize_fields_derive(RawUi)]
#[derive(Deserialize, Serialize, Default, Clone, Display)]
#[display(fmt = "")]
pub struct PlotCodex {
    pages: Vec<PlotCodexPage>,
}

#[rcize_fields_derive(RawUi)]
#[derive(Deserialize, Serialize, Default, Clone, Display)]
#[display(fmt = "{}", page)]
pub struct PlotCodexPage {
    page: i32,
    is_new: bool,
}

#[derive(Deserialize)]
pub struct PlotCategory {
    pub booleans: IndexMap<usize, String>,
    pub integers: IndexMap<usize, String>,
}

#[derive(Deserialize, Clone)]
pub struct RawPlotDb {
    pub booleans: IndexMap<usize, String>,
    pub integers: IndexMap<usize, String>,
    pub floats: IndexMap<usize, String>,
}
