use serde::{Deserialize, Serialize};

use crate::save_data::ImguiString;

#[derive(Deserialize, Serialize, RawUi, Clone)]
pub enum Origin {
    None,
    Spacer,
    Colonist,
    Earthborn,
}

#[derive(Deserialize, Serialize, RawUi, Clone)]
pub enum Notoriety {
    None,
    Survivor,
    Warhero,
    Ruthless,
}

#[derive(Deserialize, Serialize, RawUi, Default, Clone)]
pub struct WeaponLoadout {
    assault_rifle: ImguiString,
    shotgun: ImguiString,
    sniper_rifle: ImguiString,
    submachine_gun: ImguiString,
    pistol: ImguiString,
    heavy_weapon: ImguiString,
}
