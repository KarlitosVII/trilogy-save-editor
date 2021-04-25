use crate::save_data::ImguiString;

#[derive(SaveData, Clone)]
pub enum Origin {
    None,
    Spacer,
    Colony,
    Earthborn,
}

#[derive(SaveData, Clone)]
pub enum Notoriety {
    None,
    Survivor,
    Warhero,
    Ruthless,
}

#[derive(SaveData, Default, Clone)]
pub struct WeaponLoadout {
    assault_rifle: ImguiString,
    shotgun: ImguiString,
    sniper_rifle: ImguiString,
    submachine_gun: ImguiString,
    pistol: ImguiString,
    heavy_weapon: ImguiString,
}
