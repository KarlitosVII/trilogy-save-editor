use serde::{Deserialize, Serialize};

use crate::save_data::{
    shared::{
        appearance::HasHeadMorph,
        player::{Notoriety, Origin},
    },
    Dummy, ImguiString,
};

#[derive(Deserialize, Serialize, RawUi, Clone)]
pub struct Player {
    pub is_female: bool,
    _unknown1: Dummy<13>,
    pub first_name: ImguiString,
    last_name: i32,
    pub origin: Origin,
    pub notoriety: Notoriety,
    _unknown2: Dummy<13>,
    _unknown3: ImguiString,
    pub head_morph: HasHeadMorph,
    _unknown4: Vec<Dummy<8>>,
}
