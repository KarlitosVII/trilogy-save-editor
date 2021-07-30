use serde::{Deserialize, Serialize};
use derive_more::Display;

use crate::save_data::shared::Vector2d;

#[rc_ize_fields_derive(RawUi)]
#[derive(Deserialize, Serialize, Clone)]
pub struct GalaxyMap {
    planets: Vec<Planet>,
}

#[rc_ize_fields_derive(RawUi)]
#[derive(Deserialize, Serialize, Default, Clone, Display)]
#[display(fmt = "{}", id)]
pub struct Planet {
    id: i32,
    visited: bool,
    probes: Vec<Vector2d>,
}
