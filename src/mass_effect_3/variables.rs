use anyhow::Result;
use indexmap::IndexMap;

use crate::{
    save_data::{SaveCursor, SaveData},
    ui::Ui,
};

#[derive(SaveData)]
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

#[derive(SaveData)]
pub(super) struct Me1PlotTable {
    bool_variables: BitArray,
    int_variables: IndexMap<i32, i32>,
    float_variables: IndexMap<i32, f32>,
}

pub struct BitArray {
    variables: Vec<bool>,
}
impl SaveData for BitArray {
    fn deserialize(input: &mut SaveCursor) -> Result<Self> {
        let num_bytes = Self::deserialize_from::<u32>(input)?;

        let mut variables = Vec::new();
        for _ in 0..num_bytes {
            let bits = Self::deserialize_from::<u32>(input)?;
            for bit in 0..32 {
                variables.push((bits & (1 << bit)) != 0);
            }
        }

        Ok(Self { variables })
    }

    fn draw_raw_ui(&mut self, ui: &Ui, ident: &str) {
        ui.draw_bitarray(ident, &mut self.variables);
    }
}

#[derive(SaveData, Default)]
pub(super) struct PlotQuest {
    quest_counter: i32,
    quest_updated: bool,
    active_goal: i32,
    history: Vec<i32>,
}

#[derive(SaveData, Default)]
pub(super) struct PlotCodex {
    pages: Vec<PlotCodexPage>,
}

#[derive(SaveData, Default)]
pub(super) struct PlotCodexPage {
    page: i32,
    is_new: bool,
}
