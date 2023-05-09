use std::fs;

use ggez::graphics;

use super::{game_message::MessageReceiver, GameMessage};

#[derive(Clone)]
/// A struct that represents a feat to achvieve in the game (by triggering a message matching a condition a set amount of times)
pub struct Achievement {
    name: String,
    description: String,
    progress: u32,
    target: u32,
    icon: Option<graphics::Image>,
    check_fulfil: fn(&GameMessage) -> bool,
}

impl Achievement {
    pub fn new(
        name: &str,
        description: &str,
        icon: impl Into<Option<graphics::Image>>,
        target: u32,
        check_fulfil: fn(&GameMessage) -> bool,
    ) -> Self {
        Self {
            name: name.to_owned(),
            description: description.to_owned(),
            progress: 0,
            icon: icon.into(),
            target,
            check_fulfil,
        }
    }

    /// Checks a message and increases the internal progress counter if it triggers this achievement
    pub fn listen(&mut self, message: &GameMessage) {
        if (self.check_fulfil)(message) {
            self.progress = (self.progress + 1).min(self.target);
        }
    }

    /// Returns how often the conditions for this achievement have been fulfiled
    pub fn get_progress(&self) -> u32 {
        self.progress
    }

    /// Returns how often the conditions for this achievement need to be fulfiled to count as achieved
    pub fn get_target(&self) -> u32 {
        self.target
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_desc(&self) -> &str {
        &self.description
    }

    pub fn get_icon(&self) -> &Option<graphics::Image> {
        &self.icon
    }

    /// Returns wether or not this achievement has been achieved often enough yet
    pub fn is_achieved(&self) -> bool {
        self.progress >= self.target
    }

    pub fn load_set(ctx: &ggez::Context) -> AchievementSet {
        let mut res = Vec::new();

        res.push(Achievement::new(
            "First Blood",
            "Kill an enemy.",
            graphics::Image::from_path(ctx, "/sprites/heal.png").ok(),
            1,
            |msg| matches!(msg, GameMessage::UpdateGold(_)),
        ));

        res.push(Achievement::new(
            "Survivor",
            "Reach wave 2.",
            graphics::Image::from_path(ctx, "/sprites/heal.png").ok(),
            1,
            |msg| matches!(msg, GameMessage::NextWave(1)),
        ));

        res.push(Achievement::new(
            "To Dust",
            "Kill 50 enemies.",
            graphics::Image::from_path(ctx, "/sprites/heal.png").ok(),
            50,
            |msg| matches!(msg, GameMessage::UpdateGold(_)),
        ));

        let progress: Vec<u32> = toml::from_str(
            &fs::read_to_string("./data/achievements.toml").unwrap_or_else(|_| "".to_owned()),
        )
        .unwrap_or_default();

        for i in 0..progress.len().min(res.len()) {
            res[i].progress = progress[i];
        }

        res
    }

    pub fn save_set(set: AchievementSet) {
        let mut progress = Vec::new();

        for ach in set {
            progress.push(ach.get_progress());
        }

        if fs::write(
            "./data/achievements.toml",
            toml::to_string(&progress).unwrap_or_default(),
        )
        .is_err()
        {
            println!("Could not save achievements.");
        };
    }
}

type AchievementSet = Vec<Achievement>;

impl MessageReceiver for AchievementSet {
    fn receive(
        &mut self,
        message: &mooeye::UiMessage<GameMessage>,
    ) -> mooeye::scene_manager::SceneSwitch {
        if let mooeye::UiMessage::Extern(gm) = message {
            for ach in self.iter_mut() {
                ach.listen(gm);
            }
        }

        mooeye::scene_manager::SceneSwitch::None
    }
}

impl Default for Achievement {
    fn default() -> Self {
        Self::new(
            "Pride and Accomplishment",
            "This achievement has not been implemented.",
            None,
            1,
            |_| false,
        )
    }
}
