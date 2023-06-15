use std::{path::Path, fs};

use serde::{Serialize, Deserialize};




/// A struct that represents the game options.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct OptionsConfig{
    /// The volume of sound effects played by the game.
    pub volume: f32,
    /// The volume of the in-game music.
    pub music_volume: f32,
}

impl OptionsConfig{
    /// Loads a keymap from the given path and constructs a controller.
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, Box<dyn std::error::Error>> {
        let string = fs::read_to_string(
            path.as_ref()
                .to_str()
                .ok_or_else(|| ggez::GameError::CustomError("Could not read path.".to_owned()))?,
        )?;
        Ok(toml::from_str(&string)?)
    }

    /// Saves this controllers keymap to the given path.
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

impl Default for OptionsConfig{
    fn default() -> Self {
        Self { volume: 0.5, music_volume: 0.5 }
    }
}