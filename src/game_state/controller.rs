use ggez::{winit::event::VirtualKeyCode, Context};
use std::{collections::HashMap, fs, path::Path, time::Duration};

use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use toml;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
/// An enum containing all possible commands the user can give to the game.
pub enum Command {
    /// Move player character to the left.
    MoveLeft,
    /// Move player character to the right.
    MoveRight,
    /// Attack -> TODO: Expand this to multiple spells.
    Attack,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
/// A struct that serves as the interface between the user and the game state.
pub struct Controller {
    #[serde_as(as = "Vec<(_,_)>")]
    /// Manages which keys are mapped to which in-game commands.
    command_map: HashMap<VirtualKeyCode, Command>,
}

impl Controller {
    /// Loads a keymap from the given path and constructs a controller.
    pub fn from_path(path: impl AsRef<Path>) -> Self {
        let string = fs::read_to_string(
            path.as_ref()
                .to_str()
                .expect("Could not unwrap path string of keymap."),
        )
        .expect("Could not open keymap file.");
        toml::from_str(&string).expect("Cound not convert file.")
    }

    /// Saves this controllers keymap to the given path.
    pub fn save_to_file(&self, path: impl AsRef<Path>) {
        fs::write(
            path.as_ref()
                .to_str()
                .expect("Could not unwrap path string of keymap."),
            toml::to_string(&self).expect("Could convert to TOML."),
        )
        .expect("Could not write to key map file.");
    }

    /// Listens to all key presses in the context of the last frame and converts it to a list of commands given by the user as well as the time spent in the frame.
    pub fn get_interactions(&self, ctx: &Context) -> Interactions {
        let mut inter = Interactions {
            commands: HashMap::new(),
            delta: ctx.time.delta(),
        };

        for (key, value) in self.command_map.iter() {
            if ctx.keyboard.is_key_pressed(*key) {
                inter.commands.insert(*value, true);
            }
        }

        inter
    }
}

/// A struct that contains all (relevant interactions that happened in the last frame)
pub struct Interactions {
    /// Keys pressed are directly mapped to the relevant commands given
    pub commands: HashMap<Command, bool>,
    /// Time spent in the last frame
    pub delta: Duration,
}
