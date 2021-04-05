use anyhow::Result;
use indexmap::IndexMap;
use std::fmt::Debug;

use crate::serializer::{SaveCursor, Serializable};

#[derive(Serializable, Debug)]
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

#[derive(Serializable, Debug)]
pub(super) struct Me1PlotTable {
    bool_variables: BitArray,
    int_variables: IndexMap<i32, i32>,
    float_variables: IndexMap<i32, f32>,
}

#[derive(Debug)]
pub(super) struct BitArray {
    variables: Vec<bool>,
}
impl Serializable for BitArray {
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
}

#[derive(Serializable, Debug)]
pub(super) struct PlotQuest {
    quest_counter: i32,
    quest_updated: bool,
    active_goal: i32,
    history: Vec<i32>,
}

#[derive(Serializable, Debug)]
pub(super) struct PlotCodex {
    pages: Vec<PlotCodexPage>,
}

#[derive(Serializable, Debug)]
pub(super) struct PlotCodexPage {
    page: i32,
    is_new: bool,
}
