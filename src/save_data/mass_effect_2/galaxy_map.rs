use serde::{Deserialize, Serialize};

use crate::save_data::shared::Vector2d;

#[derive(Deserialize, Serialize, RawUi, Clone)]
pub struct GalaxyMap {
    planets: Vec<Planet>,
}

#[derive(Deserialize, Serialize, RawUi, Default, Clone)]
pub struct Planet {
    id: i32,
    visited: bool,
    probes: Vec<Vector2d>,
}
