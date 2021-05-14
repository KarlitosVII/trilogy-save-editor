use serde::{Deserialize, Serialize};

use crate::save_data::shared::Vector2d;

#[derive(Deserialize, Serialize, RawUi, Clone)]
pub struct GalaxyMap {
    planets: Vec<Planet>,
    systems: Vec<System>,
}

#[derive(Deserialize, Serialize, RawUi, Default, Clone)]
pub struct Planet {
    id: i32,
    visited: bool,
    probes: Vec<Vector2d>,
    show_as_scanned: bool,
}

#[derive(Deserialize, Serialize, RawUi, Default, Clone)]
pub struct System {
    id: i32,
    reaper_alert_level: f32,
    reaper_detected: bool,
}
