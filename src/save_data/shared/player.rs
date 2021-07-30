use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, RawUi)]
pub enum Origin {
    None,
    Spacer,
    Colonist,
    Earthborn,
}

#[derive(Deserialize, Serialize, Clone, RawUi)]
pub enum Notoriety {
    None,
    Survivor,
    Warhero,
    Ruthless,
}

#[rcize_fields_derive(RawUi)]
#[derive(Deserialize, Serialize, Clone, Default)]
pub struct WeaponLoadout {
    assault_rifle: String,
    shotgun: String,
    sniper_rifle: String,
    submachine_gun: String,
    pistol: String,
    heavy_weapon: String,
}
