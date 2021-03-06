use serde::{Deserialize, Serialize};

use crate::save_data::shared::Vector2d;

#[rcize_fields]
#[derive(Deserialize, Serialize, Clone, RawUi)]
pub struct GalaxyMap {
    planets: Vec<Planet>,
}

#[rcize_fields]
#[derive(Deserialize, Serialize, Clone, Default, Display, RawUi)]
#[display(fmt = "{}", id)]
pub struct Planet {
    id: i32,
    visited: bool,
    probes: Vec<Vector2d>,
}
