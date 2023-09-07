use std::{cell::RefCell, fs, path::Path};

use serde::{Deserialize, Serialize};

thread_local! {
    pub static OPTIONS: RefCell<OptionsConfig> = RefCell::new(OptionsConfig::from_path("./data/options.toml").unwrap_or_default());
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

impl OptionsConfig {
    /// Loads an option config from the given path.
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, Box<dyn std::error::Error>> {
        let string = fs::read_to_string(
            path.as_ref()
                .to_str()
                .ok_or_else(|| ggez::GameError::CustomError("Could not read path.".to_owned()))?,
        )?;
        Ok(toml::from_str(&string)?)
    }

    /// Saves this option config to the given path.
    pub fn save_to_file(&self, path: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
        fs::write(
            path.as_ref()
                .to_str()
                .ok_or_else(|| ggez::GameError::CustomError("Could not read path.".to_owned()))?,
            toml::to_string(&self)?,
        )?;
        Ok(())
    }
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

pub fn save_options() {
    // save options to file on game exit (when the main menu is dropped)
    crate::options::OPTIONS.with(|opt| {
        if opt.borrow().save_to_file("./data/options.toml").is_err() {
            println!("[ERROR/Radish] Could not save options.")
        };
    });
}
