use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::save_data::shared::plot::{BitVec, PlotCodex};

#[rcize_fields]
#[derive(Deserialize, Serialize, Clone)]
pub struct PlotTable {
    pub booleans: BitVec,
    pub integers: IndexMap<i32, i32>,
    pub floats: IndexMap<i32, f32>,
}

#[rcize_fields]
#[derive(Deserialize, Serialize, Clone, RawUi)]
pub struct Journal {
    quest_progress_counter: i32,
    quest_progress: Vec<PlotQuest>,
    quest_ids: Vec<i32>,
}

#[rcize_fields]
#[derive(Deserialize, Serialize, Clone, RawUi)]
pub struct Codex {
    codex_entries: Vec<PlotCodex>,
    codex_ids: Vec<i32>,
}

#[rcize_fields]
#[derive(Deserialize, Serialize, Clone, Default, Display, RawUi)]
#[display(fmt = "{}", quest_counter)]
pub struct PlotQuest {
    quest_counter: i32,
    quest_updated: bool,
    active_goal: i32,
    history: Vec<i32>,
}
