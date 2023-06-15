use std::fs;

use ggez::graphics;
use mooeye::{ui_element::UiContainer, *};
use serde::{Deserialize, Serialize};

use crate::PALETTE;

use super::{
    game_message::{GameMessageFilter, MessageReceiver},
    GameMessage,
};

#[derive(Clone, Debug)]
/// A struct that represents a feat to achvieve in the game (by triggering a message matching a condition a set amount of times)
pub struct Achievement {
    name: String,
    description: String,
    progress: u32,
    target: u32,
    icon: Option<graphics::Image>,
    model_message: (GameMessage, GameMessageFilter),
}

impl Achievement {
    pub fn new(
        name: &str,
        description: &str,
        icon: impl Into<Option<graphics::Image>>,
        target: u32,
        model_message: (GameMessage, GameMessageFilter),
    ) -> Self {
        Self {
            name: name.to_owned(),
            description: description.to_owned(),
            progress: 0,
            icon: icon.into(),
            target,
            model_message,
        }
    }

    pub fn reset_progress(&mut self) {
        self.progress = 0;
    }

    /// Checks a message and increases the internal progress counter if it triggers this achievement.
    /// Returns true if this completed the achievement.
    pub fn listen(&mut self, message: &GameMessage) -> bool {
        if self.model_message.1.check(&self.model_message.0, message) {
            self.progress += 1;
            self.progress == self.target
        } else {
            false
        }
    }

    /// Returns wether or not this achievement has been achieved often enough yet
    pub fn is_achieved(&self) -> bool {
        self.progress >= self.target
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Returns a ksmall UiElement representing this achievement, consisting of the icon and a tooltip.
    pub fn info_element_small<T: Copy + Eq + std::hash::Hash + 'static>(
        &self,
        ctx: &ggez::Context,
    ) -> mooeye::UiElement<T> {
        if self.is_achieved() && self.icon.is_some() {
            self.icon.clone().unwrap()
        } else {
            graphics::Image::from_path(ctx, "/sprites/ui/lock.png").unwrap()
        }
        .to_element_builder(0, ctx)
        .with_visuals(super::super::BUTTON_VIS)
        .scaled(4., 4.)
        .with_tooltip(
            graphics::Text::new(
                graphics::TextFragment::new(&self.name)
                    .color(graphics::Color::from_rgb_u32(PALETTE[7]))
                    .scale(28.),
            )
            .add("\n")
            .add(
                graphics::TextFragment::new(&self.description)
                    .color(graphics::Color::from_rgb_u32(PALETTE[6]))
                    .scale(20.),
            )
            .add(
                graphics::TextFragment::new(format!("\n  {} / {}", self.progress, self.target))
                    .color(graphics::Color::from_rgb_u32(PALETTE[6]))
                    .scale(20.),
            )
            .set_font("Retro")
            .set_wrap(true)
            .set_bounds(ggez::glam::Vec2::new(300., 200.))
            .to_owned()
            .to_element_builder(0, ctx)
            .with_visuals(super::super::BUTTON_VIS)
            .build(),
        )
        .build()
    }

    /// Returns a UiElement representing this achievement
    pub fn info_element_large<T: Copy + Eq + std::hash::Hash + 'static>(
        &self,
        ctx: &ggez::Context,
    ) -> mooeye::UiElement<T> {
        let mut ach_box = containers::HorizontalBox::new();

        if let Ok(trophy) = graphics::Image::from_path(ctx, "/sprites/achievements/a0_16_16.png") {
            ach_box.add(trophy.to_element_builder(0, ctx).scaled(4., 4.).build());
        }

        ach_box.add(
            graphics::Text::new(
                graphics::TextFragment::new(&self.name)
                    .color(graphics::Color::from_rgb_u32(PALETTE[7]))
                    .scale(28.),
            )
            .add("\n")
            .add(
                graphics::TextFragment::new(&self.description)
                    .color(graphics::Color::from_rgb_u32(PALETTE[6]))
                    .scale(20.),
            )
            .add(
                graphics::TextFragment::new(format!("\n  {} / {}", self.progress, self.target))
                    .color(graphics::Color::from_rgb_u32(PALETTE[6]))
                    .scale(20.),
            )
            .set_font("Retro")
            .set_wrap(true)
            .set_bounds(ggez::glam::Vec2::new(300., 200.))
            .to_owned()
            .to_element_builder(0, ctx)
            .build(),
        );

        if let Some(icon) = &self.icon {
            ach_box.add(
                icon.clone()
                    .to_element_builder(0, ctx)
                    .scaled(4., 4.)
                    .build(),
            );
        }

        let ach_box = ach_box
            .to_element_builder(0, ctx)
            .with_visuals(super::super::BUTTON_VIS)
            .build();

        ach_box
    }
}

pub struct AchievementSet {
    pub list: Vec<Achievement>,
    source: AchievementProgressSource,
}

impl AchievementSet {
    pub fn load(ctx: &ggez::Context, source: AchievementProgressSource) -> Self {
        let mut list = Vec::with_capacity(8);

        list.push(Achievement::new(
            "First Blood",
            "Kill an enemy.",
            graphics::Image::from_path(ctx, "/sprites/achievements/a1_16_16.png").ok(),
            1,
            (GameMessage::EnemyKilled(0), GameMessageFilter::Type),
        ));

        list.push(Achievement::new(
            "Survivor",
            "Reach wave 2.",
            graphics::Image::from_path(ctx, "/sprites/achievements/a2_16_16.png").ok(),
            1,
            (GameMessage::NextWave(2), GameMessageFilter::Equality),
        ));

        list.push(Achievement::new(
            "To Dust",
            "Kill 50 enemies.",
            graphics::Image::from_path(ctx, "/sprites/achievements/a3_16_16.png").ok(),
            50,
            (GameMessage::EnemyKilled(0), GameMessageFilter::Type),
        ));

        list.push(Achievement::new(
            "They were legion",
            "Kill 1000 enemies.",
            graphics::Image::from_path(ctx, "/sprites/achievements/a4_16_16.png").ok(),
            1000,
            (GameMessage::EnemyKilled(0), GameMessageFilter::Type),
        ));

        list.push(Achievement::new(
            "Royal Blood",
            "Kill an elite enemy.",
            graphics::Image::from_path(ctx, "/sprites/achievements/a5_16_16.png").ok(),
            1,
            (GameMessage::EnemyKilled(20), GameMessageFilter::Min),
        ));

        list.push(Achievement::new(
            "Party like it's 1789",
            "Kill 50 elite enemies.",
            graphics::Image::from_path(ctx, "/sprites/achievements/a6_16_16.png").ok(),
            50,
            (GameMessage::EnemyKilled(20), GameMessageFilter::Min),
        ));

        list.push(Achievement::new(
            "Survivor of Hathsin",
            "Reach wave 5.",
            graphics::Image::from_path(ctx, "/sprites/achievements/a7_16_16.png").ok(),
            1,
            (GameMessage::NextWave(5), GameMessageFilter::Equality),
        ));

        list.push(Achievement::new(
            "Build the wall!",
            "Take city damage 50 times.",
            graphics::Image::from_path(ctx, "/sprites/achievements/a8_16_16.png").ok(),
            50,
            (GameMessage::UpdateCityHealth(0), GameMessageFilter::Type),
        ));

        list.push(Achievement::new(
            "Power Overwhelming!",
            "Upgrade you mana well.",
            graphics::Image::from_path(ctx, "/sprites/achievements/a9_16_16.png").ok(),
            5,
            (
                GameMessage::BuildingUp(0, 1),
                GameMessageFilter::Min,
            ),
        ));

        list.push(Achievement::new(
            "Who you gonna call?",
            "Kill 15 ghosts.",
            graphics::Image::from_path(ctx, "/sprites/achievements/a10_16_16.png").ok(),
            15,
            (GameMessage::EnemyKilled(30), GameMessageFilter::Equality),
        ));

        list.push(Achievement::new(
            "I don't stress, I just cast a sweeper.",
            "Kill 1000 non-elite enemies.",
            graphics::Image::from_path(ctx, "/sprites/achievements/a4_16_16.png").ok(),
            1000,
            (GameMessage::EnemyKilled(19), GameMessageFilter::Max),
        ));

        // load progress

        match &source {
            AchievementProgressSource::File(path) => {
                let progress: AchievementProgress =
                    toml::from_str(&fs::read_to_string(path).unwrap_or_else(|_| "".to_owned()))
                        .unwrap_or_default();

                for i in 0..progress.progress_vals.len().min(list.len()) {
                    list[i].progress = progress.progress_vals[i];
                }
            }
            AchievementProgressSource::Percentage(percent) => {
                for achievement in list.iter_mut() {
                    achievement.progress = (percent * achievement.target as f32) as u32;
                }
            }
        }

        Self { list, source }
    }

    pub fn save(&self) {
        match &self.source {
            AchievementProgressSource::File(path) => {
                let progress = AchievementProgress {
                    progress_vals: self
                        .list
                        .iter()
                        .map(|achievement| achievement.progress)
                        .collect(),
                };
                if fs::write(path, toml::to_string(&progress).unwrap_or_default()).is_err() {
                    println!("[WARNING] Could not save achievements.");
                };
            }
            AchievementProgressSource::Percentage(_) => {}
        }
    }
}

impl Drop for AchievementSet {
    fn drop(&mut self) {
        self.save();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AchievementProgressSource {
    File(String),
    Percentage(f32),
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
struct AchievementProgress {
    progress_vals: Vec<u32>,
}

impl MessageReceiver for AchievementSet {
    fn receive(
        &mut self,
        message: &mooeye::UiMessage<GameMessage>,
        gui: &mut UiElement<GameMessage>,
        ctx: &ggez::Context,
    ) {
        if let mooeye::UiMessage::Extern(gm) = message {
            for ach in self.list.iter_mut() {
                if ach.listen(gm) {
                    gui.add_element(
                        100,
                        containers::DurationBox::new(
                            std::time::Duration::from_secs(15),
                            ach.info_element_large(ctx),
                        )
                        .to_element(0, ctx),
                    );
                }
            }
        }
    }
}

/// A struct that represents a list of scores. Allows Serde to .toml.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct ScoreList {
    /// The scores
    scores: Vec<i32>,
}

/// Loads the highscore list stored at ./data/highscores.toml and converts it to a vector of integer scores.
/// Returns an empty list if none can be found.
pub fn load_highscores() -> Vec<i32> {
    if let Ok(file) = std::fs::read_to_string("./data/highscores.toml") {
        toml::from_str::<ScoreList>(&file)
            .unwrap_or_default()
            .scores
    } else {
        Vec::new()
    }
}

pub fn save_highscores(scores: Vec<i32>) {
    if let Ok(toml_string) = toml::to_string(&ScoreList { scores }) {
        if std::fs::write("./data/highscores.toml", &toml_string).is_err() {
            println!("[ERROR] Could not save highscores.")
        };
    }
}
