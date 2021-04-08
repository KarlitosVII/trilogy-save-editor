use anyhow::Result;
use indexmap::IndexMap;

use crate::{
    save_data::{SaveCursor, SaveData},
    ui::Ui,
};

#[derive(SaveData, Clone)]
pub(super) struct PlotTable {
    bool_variables: BitArray,
    int_variables: IndexMap<i32, i32>,
    float_variables: IndexMap<i32, f32>,
    quest_progress_counter: i32,
    quest_progress: Vec<PlotQuest>,
    quest_ids: Vec<i32>,
    codex_entries: Vec<PlotCodex>,
    codex_ids: Vec<i32>,
}

#[derive(SaveData, Clone)]
pub(super) struct Me1PlotTable {
    bool_variables: BitArray,
    int_variables: IndexMap<i32, i32>,
    float_variables: IndexMap<i32, f32>,
}

#[derive(Clone)]
pub struct BitArray {
    variables: Vec<bool>,
}

impl SaveData for BitArray {
    fn deserialize(input: &mut SaveCursor) -> Result<Self> {
        let num_bytes = Self::deserialize_from::<u32>(input)?;
        let mut variables = Vec::new();

        let len = num_bytes * 32;
        let mut bits = 0;
        for i in 0..len {
            let bit = i % 32;

            if bit == 0 {
                bits = Self::deserialize_from::<u32>(input)?;
            }
            variables.push((bits & (1 << bit)) != 0);
        }

        Ok(Self { variables })
    }

    fn serialize(&self, output: &mut Vec<u8>) -> Result<()> {
        let len = self.variables.len() as u32 / 32;
        Self::serialize_to::<u32>(&len, output)?;

        let mut number = 0u32;
        for (i, &var) in self.variables.iter().enumerate() {
            let bit = i as u32 % 32;

            if var {
                number |= 1 << bit;
            }

            if bit == 31 {
                Self::serialize_to::<u32>(&number, output)?;
                number = 0;
            }
        }
        Ok(())
    }

    fn draw_raw_ui(&mut self, ui: &Ui, ident: &str) {
        ui.draw_bitarray(ident, &mut self.variables);
    }
}

#[derive(SaveData, Default, Clone)]
pub(super) struct PlotQuest {
    quest_counter: i32,
    quest_updated: bool,
    active_goal: i32,
    history: Vec<i32>,
}

#[derive(SaveData, Default, Clone)]
pub(super) struct PlotCodex {
    pages: Vec<PlotCodexPage>,
}

#[derive(SaveData, Default, Clone)]
pub(super) struct PlotCodexPage {
    page: i32,
    is_new: bool,
}
