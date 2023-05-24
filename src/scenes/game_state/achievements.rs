use std::fs;

use ggez::graphics;
use mooeye::{ui_element::UiContainer, *};
use serde::{Deserialize, Serialize};

use crate::PALETTE;

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

    pub fn reset_progress(&mut self) {
        self.progress = 0;
    }

    /// Checks a message and increases the internal progress counter if it triggers this achievement.
    /// Returns true if this completed the achievement.
    pub fn listen(&mut self, message: &GameMessage) -> bool {
        if (self.check_fulfil)(message) {
            self.progress += 1;
            self.progress == self.target
        } else {
            false
        }
    }

    /// Returns how often the conditions for this achievement have been fulfiled
    pub fn get_progress(&self) -> u32 {
        self.progress
    }

    /// Returns wether or not this achievement has been achieved often enough yet
    pub fn is_achieved(&self) -> bool {
        self.progress >= self.target
    }

    /// Returns a small UiElement representing this achievement, consisting of the icon and a tooltip.
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

        containers::DurationBox::new(std::time::Duration::from_secs(15), ach_box).to_element(0, ctx)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy)]
struct Progress {
    prog: u32,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct ProgressList {
    progresses: Vec<Progress>,
}

impl Default for ProgressList {
    fn default() -> Self {
        Self {
            progresses: Default::default(),
        }
    }
}
pub struct AchievementSet {
    pub list: Vec<Achievement>,
}

impl AchievementSet {
    pub fn load(ctx: &ggez::Context) -> Self {
        let mut res = Vec::with_capacity(8);

        res.push(Achievement::new(
            "First Blood",
            "Kill an enemy.",
            graphics::Image::from_path(ctx, "/sprites/achievements/a1_16_16.png").ok(),
            1,
            |msg| matches!(msg, GameMessage::EnemyKilled(_)),
        ));

        res.push(Achievement::new(
            "Survivor",
            "Reach wave 2.",
            graphics::Image::from_path(ctx, "/sprites/achievements/a2_16_16.png").ok(),
            1,
            |msg| matches!(msg, GameMessage::NextWave(2)),
        ));

        res.push(Achievement::new(
            "To Dust",
            "Kill 50 enemies.",
            graphics::Image::from_path(ctx, "/sprites/achievements/a3_16_16.png").ok(),
            50,
            |msg| matches!(msg, GameMessage::EnemyKilled(_)),
        ));
        res.push(Achievement::new(
            "They were legion",
            "Kill 1000 enemies.",
            graphics::Image::from_path(ctx, "/sprites/achievements/a4_16_16.png").ok(),
            1000,
            |msg| matches!(msg, GameMessage::EnemyKilled(_)),
        ));

        res.push(Achievement::new(
            "Royal Blood",
            "Kill an elite enemy.",
            graphics::Image::from_path(ctx, "/sprites/achievements/a5_16_16.png").ok(),
            1,
            |msg| matches!(msg, GameMessage::EnemyKilled(gold) if *gold >= 20),
        ));

        res.push(Achievement::new(
            "Party like it's 1789",
            "Kill 50 elite enemies.",
            graphics::Image::from_path(ctx, "/sprites/achievements/a6_16_16.png").ok(),
            50,
            |msg| matches!(msg, GameMessage::EnemyKilled(gold) if *gold >= 20),
        ));

        res.push(Achievement::new(
            "Survivor of Hathsin",
            "Reach wave 5.",
            graphics::Image::from_path(ctx, "/sprites/achievements/a7_16_16.png").ok(),
            1,
            |msg| matches!(msg, GameMessage::NextWave(5)),
        ));

        res.push(Achievement::new(
            "Build the wall!",
            "Take 50 city damage.",
            graphics::Image::from_path(ctx, "/sprites/achievements/a8_16_16.png").ok(),
            50,
            |msg| matches!(msg, GameMessage::UpdateCityHealth(health) if *health < 10),
        ));

        let progress: ProgressList = toml::from_str(
            &fs::read_to_string("./data/achievements.toml").unwrap_or_else(|_| "".to_owned()),
        )
        .unwrap_or_default();

        for i in 0..progress.progresses.len().min(res.len()) {
            res[i].progress = progress.progresses[i].prog;
        }

        Self { list: res }
    }

    pub fn save(&self) {
        let mut progress = ProgressList {
            progresses: Vec::new(),
        };

        for ach in self.list.iter() {
            progress.progresses.push(Progress {
                prog: ach.get_progress(),
            });
        }

        if fs::write(
            "./data/achievements.toml",
            toml::to_string(&progress).unwrap_or_default(),
        )
        .is_err()
        {
            println!("[WARNING] Could not save achievements.");
        };
    }
}

impl Drop for AchievementSet {
    fn drop(&mut self) {
        self.save();
    }
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
                    gui.add_element(100, ach.info_element_large(ctx));
                }
            }
        }
    }
}

/// A struct that represents the score achieved in a single game. Allows Serde to .toml.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
struct Score {
    /// The score as an integer.
    score: i32,
}

/// A struct that represents a list of scores. Allows Serde to .toml.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ScoreList {
    /// The scores as [Score]-structs.
    scores: Vec<Score>,
}

/// Loads the highscore list stored at ./data/highscores.toml and converts it to a vector of integer scores.
/// Returns an empty list if none can be found.
pub fn load_highscores() -> Vec<i32> {
    if let Ok(file) = std::fs::read_to_string("./data/highscores.toml") {
        toml::from_str::<ScoreList>(&file)
            .map(|sl| sl.scores.iter().map(|s| s.score).collect())
            .unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    }
}

pub fn save_highscores(scores: Vec<i32>) {
    if let Ok(toml_string) = toml::to_string(&ScoreList {
        scores: scores.iter().map(|&score| Score { score }).collect(),
    }) {
        if std::fs::write("./data/highscores.toml", &toml_string).is_err() {
            println!("[ERROR] Could not save highscores.")
        };
    }
}
