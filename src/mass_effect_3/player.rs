use imgui::ImString;

use super::appearance::Appearance;

#[derive(SaveData, Clone)]
pub(super) struct Player {
    is_female: bool,
    class_name: ImString,
    is_combat_pawn: bool,
    is_injured_pawn: bool,
    use_casual_appearance: bool,
    level: i32,
    current_xp: f32,
    first_name: ImString,
    last_name: i32,
    origin: Origin,
    notoriety: Notoriety,
    talent_points: i32,
    mapped_power_1: ImString,
    mapped_power_2: ImString,
    mapped_power_3: ImString,
    appearance: Appearance,
    emissive_id: i32,
}

#[derive(FromPrimitive, ToPrimitive, SaveData, Clone)]
enum Origin {
    None,
    Spacer,
    Colony,
    Earthborn,
}

#[derive(FromPrimitive, ToPrimitive, SaveData, Clone)]
enum Notoriety {
    None,
    Survivor,
    Warhero,
    Ruthless,
}

#[derive(SaveData, Default, Clone)]
pub(super) struct Power {
    name: ImString,
    current_rank: f32,
    evolved_choice_0: i32,
    evolved_choice_1: i32,
    evolved_choice_2: i32,
    evolved_choice_3: i32,
    evolved_choice_4: i32,
    evolved_choice_5: i32,
    power_class_name: ImString,
    wheel_display_index: i32,
}

#[derive(SaveData, Default, Clone)]
pub(super) struct Weapon {
    class_name: ImString,
    ammo_used_count: i32,
    ammo_total: i32,
    current_weapon: bool,
    was_last_weapon: bool,
    ammo_power_name: ImString,
    ammo_power_source_tag: ImString,
}

#[derive(SaveData, Default, Clone)]
pub(super) struct WeaponMod {
    weapon_class_name: ImString,
    weapon_mod_class_names: Vec<ImString>,
}

#[derive(SaveData, Default, Clone)]
pub(super) struct WeaponLoadout {
    assaul_rifle: ImString,
    shotgun: ImString,
    sniper_rifle: ImString,
    submachine_gun: ImString,
    pistol: ImString,
    heavy_weapon: ImString,
}

#[derive(SaveData, Default, Clone)]
pub(super) struct Hotkey {
    pawn_name: ImString,
    power_name: ImString,
}
