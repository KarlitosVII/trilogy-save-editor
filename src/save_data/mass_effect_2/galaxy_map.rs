use serde::{Deserialize, Serialize};
use derive_more::Display;

use crate::save_data::shared::Vector2d;

#[rcize_fields_derive(RawUi)]
#[derive(Deserialize, Serialize, Clone)]
pub struct GalaxyMap {
    planets: Vec<Planet>,
}

#[rcize_fields_derive(RawUi)]
#[derive(Deserialize, Serialize, Default, Clone, Display)]
#[display(fmt = "{}", id)]
pub struct Planet {
    id: i32,
    visited: bool,
    probes: Vec<Vector2d>,
}
