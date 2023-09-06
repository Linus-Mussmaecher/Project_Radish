use std::{cell::RefCell, fs};

use ggez::graphics;
use mooeye::{ui, ui::UiContainer, ui::UiContent};
use serde::{Deserialize, Serialize};

use crate::PALETTE;

pub const ACHIEVEMENT_BOX: u32 = 100;

use super::{
    game_message::{GameMessageFilter, MessageReceiver},
    GameMessage,
};

thread_local! {
    pub static ACHIEVEMENTS: RefCell<AchievementProgress> = RefCell::new(
        toml::from_str(&fs::read_to_string("./data/achievements.toml")
            .unwrap_or_else(|_| "".to_owned()))
            .unwrap_or_default()
    );

    pub static HIGHSCORES: RefCell<ScoreList> = RefCell::new(
            toml::from_str::<ScoreList>(&fs::read_to_string("./data/highscores.toml")
                .unwrap_or_else(|_| "".to_owned()))
                .unwrap_or_default()
    )
}

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
    ) -> ui::UiElement<T> {
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
    ) -> ui::UiElement<T> {
        let mut ach_box = ui::containers::HorizontalBox::new();

        if let Ok(trophy) = graphics::Image::from_path(ctx, "/sprites/achievements/a00_16_16.png") {
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

        ach_box
            .to_element_builder(0, ctx)
            .with_visuals(super::super::BUTTON_VIS)
            .build()
    }
}

pub struct AchievementSet {
    pub list: Vec<Achievement>,
    source: AchievementProgressSource,
}

impl AchievementSet {
    pub fn load(ctx: &ggez::Context, source: AchievementProgressSource) -> Self {
        // 3x kill counts (1, 50, 1000) + 1x kill basic 1000
        // 2x elite kills (1, 50)
        // 4x waves reached (2, 5, 50, 10x10)
        // 2x special kills (ghost, guardian)
        // 3x upgrades // 1x lose building

        let mut list = vec![
            Achievement::new(
                "First Blood",
                "Kill an enemy.",
                graphics::Image::from_path(ctx, "/sprites/achievements/a01_16_16.png").ok(),
                1,
                (GameMessage::EnemyKilled(0), GameMessageFilter::Type),
            ),
            Achievement::new(
                "To Dust",
                "Kill 50 enemies.",
                graphics::Image::from_path(ctx, "/sprites/achievements/a02_16_16.png").ok(),
                50,
                (GameMessage::EnemyKilled(0), GameMessageFilter::Type),
            ),
            Achievement::new(
                "They were legion",
                "Kill 1000 enemies.",
                graphics::Image::from_path(ctx, "/sprites/achievements/a03_16_16.png").ok(),
                1000,
                (GameMessage::EnemyKilled(0), GameMessageFilter::Type),
            ),
            Achievement::new(
                "Puttin' on the broom",
                "Kill 1000 non-elite enemies.",
                graphics::Image::from_path(ctx, "/sprites/achievements/a04_16_16.png").ok(),
                1000,
                (GameMessage::EnemyKilled(10), GameMessageFilter::Max),
            ),
            Achievement::new(
                "Survivor",
                "Reach wave 2.",
                graphics::Image::from_path(ctx, "/sprites/achievements/a05_16_16.png").ok(),
                1,
                (GameMessage::NextWave(2), GameMessageFilter::Equality),
            ),
            Achievement::new(
                "Can't touch this",
                "Reach wave 5.",
                graphics::Image::from_path(ctx, "/sprites/achievements/a06_16_16.png").ok(),
                1,
                (GameMessage::NextWave(5), GameMessageFilter::Equality),
            ),
            Achievement::new(
                "One kick, a thousand times",
                "Reach level 8, 8 times.",
                graphics::Image::from_path(ctx, "/sprites/achievements/a07_16_16.png").ok(),
                8,
                (GameMessage::NextWave(8), GameMessageFilter::Equality),
            ),
            Achievement::new(
                "Supreme",
                "Reach level 24.",
                graphics::Image::from_path(ctx, "/sprites/achievements/a08_16_16.png").ok(),
                1,
                (GameMessage::NextWave(24), GameMessageFilter::Equality),
            ),
            Achievement::new(
                "Royal Blood",
                "Kill an elite enemy.",
                graphics::Image::from_path(ctx, "/sprites/achievements/a09_16_16.png").ok(),
                1,
                (GameMessage::EnemyKilled(10), GameMessageFilter::Min),
            ),
            Achievement::new(
                "Party like it's 1789",
                "Kill 50 elite enemies.",
                graphics::Image::from_path(ctx, "/sprites/achievements/a10_16_16.png").ok(),
                50,
                (GameMessage::EnemyKilled(10), GameMessageFilter::Min),
            ),
            Achievement::new(
                "Speed limit",
                "Kill 50 bannermen.",
                graphics::Image::from_path(ctx, "/sprites/achievements/a11_16_16.png").ok(),
                50,
                (GameMessage::EnemyKilled(11), GameMessageFilter::Equality),
            ),
            Achievement::new(
                "Who you gonna call?",
                "Kill 15 ghosts.",
                graphics::Image::from_path(ctx, "/sprites/achievements/a12_16_16.png").ok(),
                15,
                (GameMessage::EnemyKilled(16), GameMessageFilter::Equality),
            ),
            Achievement::new(
                "The Lives of Others",
                "Upgrade your watchtower five times.",
                graphics::Image::from_path(ctx, "/sprites/achievements/a13_16_16.png").ok(),
                5,
                (GameMessage::BuildingUp(0, 1), GameMessageFilter::Min),
            ),
            Achievement::new(
                "Union fees",
                "Upgrade your mage's guild five times.",
                graphics::Image::from_path(ctx, "/sprites/achievements/a14_16_16.png").ok(),
                5,
                (GameMessage::BuildingUp(1, 1), GameMessageFilter::Min),
            ),
            Achievement::new(
                "Power Overwhelming!",
                "Upgrade you mana well five times.",
                graphics::Image::from_path(ctx, "/sprites/achievements/a15_16_16.png").ok(),
                5,
                (GameMessage::BuildingUp(2, 1), GameMessageFilter::Min),
            ),
            Achievement::new(
                "Oops",
                "Lose ten buildings.",
                graphics::Image::from_path(ctx, "/sprites/achievements/a16_16_16.png").ok(),
                10,
                (GameMessage::BuildingDown(0, 0), GameMessageFilter::Type),
            ),
        ];

        // load progress

        match &source {
            AchievementProgressSource::Percentage(percent) => {
                for achievement in list.iter_mut() {
                    achievement.progress = (percent * achievement.target as f32) as u32;
                }
            }
            AchievementProgressSource::Cache => ACHIEVEMENTS.with(|ach| {
                for i in 0..ach.borrow().progress_vals.len().min(list.len()) {
                    list[i].progress = ach.borrow().progress_vals[i];
                }
            }),
        }

        Self { list, source }
    }

    pub fn save(&self) {
        match &self.source {
            AchievementProgressSource::Cache => {
                let progress = AchievementProgress {
                    progress_vals: self
                        .list
                        .iter()
                        .map(|achievement| achievement.progress)
                        .collect(),
                };
                ACHIEVEMENTS.with(|ach| *ach.borrow_mut() = progress)
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
    Percentage(f32),
    Cache,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AchievementProgress {
    progress_vals: Vec<u32>,
}

impl MessageReceiver for AchievementSet {
    fn receive(
        &mut self,
        message: &ui::UiMessage<GameMessage>,
        gui: &mut ui::UiElement<GameMessage>,
        ctx: &ggez::Context,
    ) {
        if let ui::UiMessage::Extern(gm) = message {
            for ach in self.list.iter_mut() {
                if ach.listen(gm) {
                    gui.add_element(
                        ACHIEVEMENT_BOX,
                        ui::containers::DurationBox::new(
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
pub struct ScoreList {
    /// The scores
    pub scores: Vec<(u32, u32)>,
}
