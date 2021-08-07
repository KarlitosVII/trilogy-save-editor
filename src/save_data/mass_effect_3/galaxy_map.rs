use derive_more::Display;
use serde::{Deserialize, Serialize};

use crate::save_data::shared::Vector2d;

#[rcize_fields]
#[derive(Deserialize, Serialize, Clone, RawUi)]
pub struct GalaxyMap {
    planets: Vec<Planet>,
    systems: Vec<System>,
}

#[rcize_fields]
#[derive(Deserialize, Serialize, Clone, Default, Display, RawUi)]
#[display(fmt = "{}", id)]
pub struct Planet {
    id: i32,
    visited: bool,
    probes: Vec<Vector2d>,
    show_as_scanned: bool,
}

#[rcize_fields]
#[derive(Deserialize, Serialize, Clone, Default, Display, RawUi)]
#[display(fmt = "{}", id)]
pub struct System {
    id: i32,
    reaper_alert_level: f32,
    reaper_detected: bool,
}
