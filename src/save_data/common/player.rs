use imgui::ImString;

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
    assault_rifle: ImString,
    shotgun: ImString,
    sniper_rifle: ImString,
    submachine_gun: ImString,
    pistol: ImString,
    heavy_weapon: ImString,
}
