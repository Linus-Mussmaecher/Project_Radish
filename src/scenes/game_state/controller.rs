use good_web_game::{input::keyboard::KeyCode, Context};
use std::{collections::HashMap, time::Duration};

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, Serialize, Deserialize)]
/// An enum containing all possible commands the user can give to the game.
pub enum Command {
    /// Move player character to the left.
    MoveLeft,
    /// Move player character to the right.
    MoveRight,
    /// Cast the first spell in the spell selection.
    Spell0,
    /// Cast the second spell in the spell selection.
    Spell1,
    /// Cast the third spell in the spell selection.
    Spell2,
    /// Cast the fourth spell in the spell selection.
    Spell3,
    /// No command
    #[default]
    None,
}

impl Command {
    pub fn spell_from_int(index: usize) -> Self {
        match index {
            0 => Self::Spell0,
            1 => Self::Spell1,
            2 => Self::Spell2,
            3 => Self::Spell3,
            _ => Self::None,
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Mapping {
    keycode: KeyCode,
    command: Command,
}

impl Mapping {
    pub fn new(keycode: KeyCode, command: Command) -> Self {
        Self { keycode, command }
    }
}

#[derive(Debug)]
/// A struct that serves as the interface between the user and the game state.
pub struct Controller {
    /// Manages which keys are mapped to which in-game commands.
    command_map: Vec<Mapping>,
}

impl Controller {
    /// Listens to all key presses in the context of the last frame and converts it to a list of commands given by the user as well as the time spent in the frame.
    pub fn get_interactions(&self, ctx: &Context) -> Interactions {
        let mut inter = Interactions {
            commands: HashMap::new(),
            delta: good_web_game::timer::delta(ctx),
        };

        for &Mapping { keycode, command } in self.command_map.iter() {
            if good_web_game::input::keyboard::is_key_pressed(ctx, keycode) {
                inter.commands.insert(command, true);
            }
        }

        inter
    }
}

impl Default for Controller {
    fn default() -> Self {
        Self {
            command_map: Vec::from([
                Mapping::new(KeyCode::A, Command::MoveLeft),
                Mapping::new(KeyCode::D, Command::MoveRight),
                Mapping::new(KeyCode::Left, Command::MoveLeft),
                Mapping::new(KeyCode::Right, Command::MoveRight),
                Mapping::new(KeyCode::Y, Command::Spell0),
                Mapping::new(KeyCode::Z, Command::Spell0),
                Mapping::new(KeyCode::J, Command::Spell0),
                Mapping::new(KeyCode::X, Command::Spell1),
                Mapping::new(KeyCode::K, Command::Spell1),
                Mapping::new(KeyCode::C, Command::Spell2),
                Mapping::new(KeyCode::L, Command::Spell2),
                Mapping::new(KeyCode::V, Command::Spell3),
                Mapping::new(KeyCode::Semicolon, Command::Spell3),
            ]),
        }
    }
}

/// A struct that contains all (relevant interactions that happened in the last frame)
pub struct Interactions {
    /// Keys pressed are directly mapped to the relevant commands given
    pub commands: HashMap<Command, bool>,
    /// Time spent in the last frame
    pub delta: Duration,
}
