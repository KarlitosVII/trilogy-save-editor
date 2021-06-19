use serde::{Deserialize, Serialize};

use crate::save_data::{Dummy, ImguiString};

use super::BaseObject;

#[derive(Deserialize, Serialize, RawUiMe1Legacy, Clone)]
pub struct ArtPlaceableBehavior {
    is_dead: bool,
    generated_treasure: bool,
    challenge_scaled: bool,
    owner: Box<Option<BaseObject>>,
    health: f32,
    current_health: f32,
    enabled: bool,
    current_fsm_state_name: ImguiString,
    is_destroyed: bool,
    state_0: ImguiString,
    state_1: ImguiString,
    use_case: u8,
    use_case_override: bool,
    player_only: bool,
    skill_difficulty: u8,
    inventory: Box<Option<BaseObject>>,
    skill_game_failed: bool,
    skill_game_xp_awarded: bool,
}

#[derive(Deserialize, Serialize, RawUiMe1Legacy, Clone)]
pub struct ArtPlaceable {
    _unknown: Dummy<60>,
}
