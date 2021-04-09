use crate::save_data::common::plot::{BitArray, PlotCodex};

#[derive(SaveData, Clone)]
pub struct PlotTable {
    bool_variables: BitArray,
    int_variables: Vec<i32>,
    float_variables: Vec<f32>,
    quest_progress_counter: i32,
    quest_progress: Vec<PlotQuest>,
    quest_ids: Vec<i32>,
    codex_entries: Vec<PlotCodex>,
    codex_ids: Vec<i32>,
}

#[derive(SaveData, Clone)]
pub struct Me1PlotTable {
    bool_variables: BitArray,
    int_variables: Vec<i32>,
    float_variables: Vec<f32>,
}

#[derive(SaveData, Default, Clone)]
pub struct PlotQuest {
    quest_counter: i32,
    quest_updated: bool,
    history: Vec<i32>,
}
