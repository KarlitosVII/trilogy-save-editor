use serde::{Deserialize, Serialize};

use crate::save_data::ImguiString;

#[derive(Deserialize, Serialize, SaveData, Clone)]
pub enum Origin {
    None,
    Spacer,
    Colony,
    Earthborn,
}

#[derive(Deserialize, Serialize, SaveData, Clone)]
pub enum Notoriety {
    None,
    Survivor,
    Warhero,
    Ruthless,
}

#[derive(Deserialize, Serialize, SaveData, Default, Clone)]
pub struct WeaponLoadout {
    assault_rifle: ImguiString,
    shotgun: ImguiString,
    sniper_rifle: ImguiString,
    submachine_gun: ImguiString,
    pistol: ImguiString,
    heavy_weapon: ImguiString,
}
