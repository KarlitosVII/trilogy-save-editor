use serde::{Deserialize, Serialize};

use crate::save_data::common::Vector2d;

#[derive(Deserialize, Serialize, SaveData, Clone)]
pub struct GalaxyMap {
    planets: Vec<Planet>,
}

#[derive(Deserialize, Serialize, SaveData, Default, Clone)]
pub struct Planet {
    id: i32,
    visited: bool,
    probes: Vec<Vector2d>,
}
