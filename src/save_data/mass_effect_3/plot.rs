use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::save_data::shared::plot::{BoolVec, PlotCodex};

#[derive(Deserialize, Serialize, RawUi, Clone)]
pub struct PlotTable {
    pub booleans: BoolVec,
    pub integers: IndexMap<i32, i32>,
    pub floats: IndexMap<i32, f32>,
    quest_progress_counter: i32,
    quest_progress: Vec<PlotQuest>,
    quest_ids: Vec<i32>,
    codex_entries: Vec<PlotCodex>,
    codex_ids: Vec<i32>,
}

#[derive(Deserialize, Serialize, RawUi, Clone)]
pub struct Me1PlotTable {
    booleans: BoolVec,
    integers: IndexMap<i32, i32>,
    floats: IndexMap<i32, f32>,
}

#[derive(Deserialize, Serialize, RawUi, Default, Clone)]
pub struct PlotQuest {
    quest_counter: i32,
    quest_updated: bool,
    active_goal: i32,
    history: Vec<i32>,
}
