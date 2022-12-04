use std::sync::Mutex;

use once_cell::sync::Lazy;
pub struct GlobalGameState {
    pub tutorial_completed: bool,
}

impl Default for GlobalGameState {
    fn default() -> Self {
        Self {
            tutorial_completed: false,
        }
    }
}

pub const GLOBAL_GAME_STATE: Lazy<Mutex<GlobalGameState>> =
    Lazy::new(|| Mutex::new(GlobalGameState::default()));
