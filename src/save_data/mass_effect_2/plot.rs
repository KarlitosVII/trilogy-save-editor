use serde::{Deserialize, Serialize};

use crate::save_data::shared::plot::{BoolVec, PlotCodex};

#[derive(Deserialize, Serialize, RawUi, Clone)]
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

#[derive(Deserialize, Serialize, RawUi, Default, Clone)]
pub struct PlotQuest {
    quest_counter: i32,
    quest_updated: bool,
    history: Vec<i32>,
}
