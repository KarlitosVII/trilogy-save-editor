mod general;
mod plot;
mod raw_plot;

pub use self::{general::*, plot::*, raw_plot::*};

use crate::save_data::{
    mass_effect_2::{Me2LeSaveGame, Me2SaveGame},
    RcRef,
};

#[derive(Clone)]
pub enum Me2Type {
    Vanilla(RcRef<Me2SaveGame>),
    Legendary(RcRef<Me2LeSaveGame>),
}

impl PartialEq for Me2Type {
    fn eq(&self, other: &Me2Type) -> bool {
        match (self, other) {
            (Me2Type::Vanilla(vanilla), Me2Type::Vanilla(other)) => vanilla == other,
            (Me2Type::Legendary(legendary), Me2Type::Legendary(other)) => legendary == other,
            _ => false,
        }
    }
}
