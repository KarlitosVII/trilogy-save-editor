use crate::save_data::common::Vector2d;

#[derive(SaveData, Clone)]
pub struct GalaxyMap {
    planets: Vec<Planet>,
}

#[derive(SaveData, Default, Clone)]
pub struct Planet {
    id: i32,
    visited: bool,
    probes: Vec<Vector2d>,
}
