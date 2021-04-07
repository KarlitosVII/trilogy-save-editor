#[derive(SaveData)]
pub(super) struct GalaxyMap {
    planets: Vec<Planet>,
    systems: Vec<System>,
}

#[derive(SaveData, Default)]
pub(super) struct Planet {
    id: i32,
    visited: bool,
    probes: Vec<Vector2d>,
    show_as_scanned: bool,
}

#[derive(SaveData, Default)]
pub(super) struct System {
    id: i32,
    reaper_alert_level: f32,
    reaper_detected: bool,
}

#[derive(SaveData, Default)]
pub(super) struct Vector2d {
    x: f32,
    y: f32,
}
