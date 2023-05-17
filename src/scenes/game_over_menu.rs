use ggez::{graphics, GameError};
use mooeye::{ui_element::UiContainer, *};
use serde::{Deserialize, Serialize};

use crate::PALETTE;

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

/// The Menu that is shown over the game state when the game ends.
pub struct GameOverMenu {
    /// The UI.
    ui: UiElement<()>,
}

impl GameOverMenu {
    /// Creates a new GameOverMenu displaying the passed score and adding it (if good enough) to the highscore list.
    /// Also displays the highscore list and marks the newly achieved score (it it shows up).
    pub fn new(ctx: &ggez::Context, score: i32) -> Result<Self, GameError> {
        // load highscores

        let mut highscores: Vec<Score> =
            if let Ok(file) = std::fs::read_to_string("./data/highscores.toml") {
                toml::from_str::<ScoreList>(&file)
                    .map(|sl| sl.scores)
                    .unwrap_or_else(|_| Vec::new())
            } else {
                Vec::new()
            };

        // if only a small amount of scores is recorded or the worst result from the list was beaten, insert the new result
        let own_index = if highscores.len() < 25
            || score >= highscores.last().map(|a| a.score).unwrap_or_default()
        {
            // set default index: at the end of the list
            let mut index = highscores.len();
            // see if there is an appropriate earlier index
            for (i, value) in highscores.iter().enumerate() {
                if score >= value.score {
                    index = i;
                    break;
                }
            }
            // insert at appropriate index
            highscores.insert(index, Score { score });
            // if list has grown too much, cut last element
            if highscores.len() > 25 {
                highscores.pop();
            }
            // return selected index
            Some(index)
        } else {
            None
        };

        // save highscores
        if let Ok(toml_string) = toml::to_string(&ScoreList {
            scores: highscores.clone(),
        }) {
            std::fs::write("./data/highscores.toml", &toml_string)?;
        }

        // create UI

        let mut main_box = containers::VerticalBox::new();
        main_box.spacing = 25.;

        // title

        let game_over = graphics::Text::new(
            graphics::TextFragment::new("Game Over!")
                .color(graphics::Color::from_rgb_u32(PALETTE[8])),
        )
        .set_font("Retro")
        .set_scale(54.)
        .to_owned()
        .to_element_builder(0, ctx)
        .build();
        main_box.add(game_over);

        // horizontal box with own score left and highscores right

        let mut score_box = containers::HorizontalBox::new();
        score_box.spacing = 35.;

        // own score + congratulatory message

        let score_disp = graphics::Text::new(
            graphics::TextFragment::new("Score\n")
                .color(graphics::Color::from_rgb_u32(PALETTE[7]))
                .scale(36.),
        )
        .add(
            graphics::TextFragment::new(format!("{:>5}", score))
                .color(graphics::Color::from_rgb_u32(PALETTE[6]))
                .scale(32.),
        )
        .add(
            // if new record, display that!
            graphics::TextFragment::new(if own_index.unwrap_or(128) == 0 {
                "\nNew Record!"
            } else {
                ""
            })
            .color(graphics::Color::from_rgb_u32(PALETTE[8]))
            .scale(34.),
        )
        // monospace font for scores
        .set_font("Retro_M")
        .to_owned()
        .to_element_builder(0, ctx)
        .with_alignment(ui_element::Alignment::Center, ui_element::Alignment::Min)
        .build();
        score_box.add(score_disp);

        // list of 5 best scores so far

        let mut highscore_disp = graphics::Text::new(
            graphics::TextFragment::new("Highscores")
                .color(graphics::Color::from_rgb_u32(PALETTE[7]))
                .scale(36.),
        );

        // add the first 5 (or less) scores as texts to the element

        for (index, value) in highscores.iter().enumerate().take(5) {
            highscore_disp.add(
                graphics::TextFragment::new(format!("\n  {:02}.{:>5}", index + 1, value.score))
                    .color(graphics::Color::from_rgb_u32(
                        // if own score shows up, change color to make it stand out
                        if index == own_index.unwrap_or(128) {
                            PALETTE[8]
                        } else {
                            PALETTE[6]
                        },
                    ))
                    .scale(32.),
            );
        }

        // convert to ui element

        let highscore_disp = highscore_disp
            .set_font("Retro_M")
            .to_owned()
            .to_element_builder(0, ctx)
            .with_alignment(ui_element::Alignment::Center, ui_element::Alignment::Min)
            .build();

        score_box.add(highscore_disp);

        main_box.add(score_box.to_element(0, ctx));

        // restart button

        let restart = graphics::Text::new(
            graphics::TextFragment::new("Restart").color(graphics::Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_builder(1, ctx)
        .with_trigger_key(ggez::winit::event::VirtualKeyCode::R)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .build();
        main_box.add(restart);

        // quit to main menu button

        let main_menu = graphics::Text::new(
            graphics::TextFragment::new("Return to Main Menu")
                .color(graphics::Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_builder(2, ctx)
        .with_trigger_key(ggez::winit::event::VirtualKeyCode::M)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .build();
        main_box.add(main_menu);

        let main_box = main_box
            .to_element_builder(0, ctx)
            .with_visuals(super::BUTTON_VIS)
            .with_alignment(ui_element::Alignment::Center, ui_element::Alignment::Center)
            .with_padding((25., 25., 25., 25.))
            .build();

        Ok(Self { ui: main_box })
    }
}

impl scene_manager::Scene for GameOverMenu {
    fn update(&mut self, ctx: &mut ggez::Context) -> Result<scene_manager::SceneSwitch, GameError> {
        let messages = self.ui.manage_messages(ctx, None);

        // restart the game

        if messages.contains(&mooeye::UiMessage::Triggered(1)) {
            return Ok(mooeye::scene_manager::SceneSwitch::replace(
                crate::game_state::GameState::new(ctx)?,
                2,
            ));
        }

        // return to main menu

        if messages.contains(&mooeye::UiMessage::Triggered(2)) {
            return Ok(mooeye::scene_manager::SceneSwitch::replace(
                super::main_menu::MainMenu::new(ctx)?,
                2,
            ));
        }
        Ok(mooeye::scene_manager::SceneSwitch::None)
    }

    fn draw(&mut self, ctx: &mut ggez::Context, mouse_listen: bool) -> Result<(), GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, None);
        canvas.set_sampler(graphics::Sampler::nearest_clamp());

        self.ui.draw_to_screen(ctx, &mut canvas, mouse_listen);

        canvas.finish(ctx)?;
        Ok(())
    }
}
