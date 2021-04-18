use anyhow::*;
use async_trait::async_trait;
use bitvec::prelude::*;
use indexmap::IndexMap;
use serde::Deserialize;

use crate::{
    gui::Gui,
    save_data::{SaveCursor, SaveData},
};

pub type BoolVec = BitVec<Lsb0, u32>;
pub type BoolSlice = BitSlice<Lsb0, u32>;

#[async_trait(?Send)]
impl SaveData for BoolVec {
    fn deserialize(cursor: &mut SaveCursor) -> Result<Self> {
        let bitfields: Vec<u32> = SaveData::deserialize(cursor)?;

        let variables = BoolVec::from_vec(bitfields);
        Ok(variables)
    }

    fn serialize(&self, output: &mut Vec<u8>) -> Result<()> {
        let bitfields = self.clone().into_vec();

        SaveData::serialize(&bitfields, output)
    }

    async fn draw_raw_ui(&mut self, ui: &Gui, ident: &str) {
        ui.draw_boolvec(ident, self).await;
    }
}

#[derive(SaveData, Clone)]
pub struct Me1PlotTable {
    pub bool_variables: BoolVec,
    pub int_variables: Vec<i32>,
    pub float_variables: Vec<f32>,
}

#[derive(SaveData, Default, Clone)]
pub struct PlotCodex {
    pages: Vec<PlotCodexPage>,
}

#[derive(SaveData, Default, Clone)]
pub struct PlotCodexPage {
    page: i32,
    is_new: bool,
}

#[derive(Deserialize)]
pub struct KnownPlot {
    pub booleans: IndexMap<usize, String>,
    pub ints: IndexMap<usize, String>,
}
