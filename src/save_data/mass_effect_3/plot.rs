use indexmap::IndexMap;

use crate::save_data::common::plot::{BoolVec, PlotCodex};

#[derive(SaveData, Clone)]
pub struct PlotTable {
    pub bool_variables: BoolVec,
    pub int_variables: IndexMap<i32, i32>,
    pub float_variables: IndexMap<i32, f32>,
    quest_progress_counter: i32,
    quest_progress: Vec<PlotQuest>,
    quest_ids: Vec<i32>,
    codex_entries: Vec<PlotCodex>,
    codex_ids: Vec<i32>,
}

#[derive(SaveData, Clone)]
pub struct Me1PlotTable {
    bool_variables: BoolVec,
    int_variables: IndexMap<i32, i32>,
    float_variables: IndexMap<i32, f32>,
}

#[derive(SaveData, Default, Clone)]
pub struct PlotQuest {
    quest_counter: i32,
    quest_updated: bool,
    active_goal: i32,
    history: Vec<i32>,
}
