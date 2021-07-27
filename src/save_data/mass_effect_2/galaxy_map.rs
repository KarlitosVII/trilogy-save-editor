use serde::{Deserialize, Serialize};

use crate::save_data::shared::Vector2d;

#[rc_ize_fields_derive(RawUi)]
#[derive(Deserialize, Serialize, Clone)]
pub struct GalaxyMap {
    planets: Vec<Planet>,
}

#[rc_ize_fields_derive(RawUi)]
#[derive(Deserialize, Serialize, Default, Clone, PartialEq)]
pub struct Planet {
    id: i32,
    visited: bool,
    probes: Vec<Vector2d>,
}
