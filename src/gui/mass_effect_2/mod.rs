mod general;
mod plot;

use crate::save_data::mass_effect_2::{Me2LeSaveGame, Me2SaveGame};

pub use self::{general::*, plot::*};

use super::RcUi;

#[derive(Clone)]
pub enum Me2Type {
    Vanilla(RcUi<Me2SaveGame>),
    Legendary(RcUi<Me2LeSaveGame>),
}

impl PartialEq for Me2Type {
    fn eq(&self, other: &Me2Type) -> bool {
        match self {
            Me2Type::Vanilla(me2) => match other {
                Me2Type::Vanilla(other) => me2 == other,
                _ => false,
            },
            Me2Type::Legendary(me2) => match other {
                Me2Type::Legendary(other) => me2 == other,
                _ => false,
            },
        }
    }
}
