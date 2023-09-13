use std::cell::RefCell;

use serde::{Deserialize, Serialize};

thread_local! {
    pub static OPTIONS: RefCell<OptionsConfig> = RefCell::new(OptionsConfig::default());
}

/// A struct that represents the game options.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct OptionsConfig {
    /// The volume of sound effects played by the game.
    pub volume: u8,
    /// The volume of the in-game music.
    pub music_volume: u8,
    /// Wether or not to display tutorial hints
    pub tutorial: bool,
}

impl Default for OptionsConfig {
    fn default() -> Self {
        Self {
            volume: 50,
            music_volume: 50,
            tutorial: true,
        }
    }
}
