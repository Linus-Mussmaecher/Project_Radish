use super::game_state::achievements;
use good_web_game::event::GraphicsContext;
use good_web_game::{graphics, GameError};
use mooeye::{scene_manager, ui, ui::UiContainer, ui::UiContent};

use crate::music;
use crate::PALETTE;

/// The Menu that is shown over the game state when the game ends.
pub struct GameOverMenu {
    /// The UI.
    ui: ui::UiElement<()>,
    /// The music player
    music_player: music::MusicPlayer,
}

impl GameOverMenu {
    /// Creates a new GameOverMenu displaying the passed score and adding it (if good enough) to the highscore list.
    /// Also displays the highscore list and marks the newly achieved score (it it shows up).
    pub fn new(ctx: &mut good_web_game::Context, wave: u32, score: u32) -> Result<Self, GameError> {
        // load highscores

        let own_index = achievements::HIGHSCORES.with(|scores| {
            let highscores = &mut scores.borrow_mut();
            // if only a small amount of scores is recorded or the worst result from the list was beaten, insert the new result
            if highscores.len() < 25 || score >= highscores.last().copied().unwrap_or_default().1 {
                // set default index: at the end of the list
                let mut index = highscores.len();
                // see if there is an appropriate earlier index
                for (i, &value) in highscores.iter().enumerate() {
                    if score >= value.1 {
                        index = i;
                        break;
                    }
                }
                // insert at appropriate index
                highscores.insert(index, (wave, score));
                // if list has grown too much, cut last element
                if highscores.len() > 25 {
                    highscores.pop();
                }
                // return selected index
                Some(index)
            } else {
                None
            }
        });

        // create UI

        let mut main_box = ui::containers::VerticalBox::new();
        main_box.spacing = 25.;

        // title

        let game_over = graphics::Text::new(
            graphics::TextFragment::new("Game Over!")
                .color(graphics::Color::from_rgb_u32(PALETTE[8])),
        )
        .set_font(
            crate::RETRO.with(|f| f.borrow().unwrap()),
            graphics::PxScale { x: 54., y: 54. },
        )
        .to_owned()
        .to_element_builder(0, ctx)
        .build();
        main_box.add(game_over);

        // horizontal box with own score left and highscores right

        let mut score_box = ui::containers::HorizontalBox::new();
        score_box.spacing = 35.;

        // own score + congratulatory message

        let score_disp = graphics::Text::new(
            graphics::TextFragment::new("Score\n")
                .color(graphics::Color::from_rgb_u32(PALETTE[7]))
                .scale(36.),
        )
        // monospace font for scores
        .set_font(
            crate::RETRO_M.with(|f| f.borrow().unwrap()),
            graphics::PxScale { x: 36., y: 36. },
        )
        .add(
            graphics::TextFragment::new(format!("{:>7}", score))
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
        .to_owned()
        .to_element_builder(0, ctx)
        .with_alignment(ui::Alignment::Center, ui::Alignment::Min)
        .build();
        score_box.add(score_disp);

        // list of 5 best scores so far

        let mut highscore_disp = graphics::Text::new(
            graphics::TextFragment::new("Highscores")
                .color(graphics::Color::from_rgb_u32(PALETTE[7]))
                .scale(36.),
        );

        // add the first 5 (or less) scores as texts to the element

        achievements::HIGHSCORES.with(|scores| {
            for (index, &(_, score)) in scores.borrow().iter().enumerate().take(5) {
                highscore_disp.add(
                    graphics::TextFragment::new(format!("\n  {:02}.{:>5}", index + 1, score))
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
        });

        // convert to ui element

        let highscore_disp = highscore_disp
            .set_font(
                crate::RETRO_M.with(|f| f.borrow().unwrap()),
                graphics::PxScale { x: 36., y: 36. },
            )
            .to_owned()
            .to_element_builder(0, ctx)
            .with_alignment(ui::Alignment::Center, ui::Alignment::Min)
            .build();

        score_box.add(highscore_disp);

        main_box.add(score_box.to_element(0, ctx));

        // restart button

        let restart = graphics::Text::new(
            graphics::TextFragment::new("Restart").color(graphics::Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font(
            crate::RETRO.with(|f| f.borrow().unwrap()),
            graphics::PxScale { x: 32., y: 32. },
        )
        .to_owned()
        .to_element_builder(1, ctx)
        .with_trigger_key(good_web_game::input::keyboard::KeyCode::R)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .with_trigger_sound(
            good_web_game::audio::Source::new(ctx, "./audio/sounds/ui/blipSelect.wav").ok(),
        )
        .build();
        main_box.add(restart);

        // quit to main menu button

        let main_menu = graphics::Text::new(
            graphics::TextFragment::new("Return to Main Menu")
                .color(graphics::Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font(
            crate::RETRO.with(|f| f.borrow().unwrap()),
            graphics::PxScale { x: 32., y: 32. },
        )
        .to_owned()
        .to_element_builder(2, ctx)
        .with_trigger_key(good_web_game::input::keyboard::KeyCode::M)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .with_trigger_sound(
            good_web_game::audio::Source::new(ctx, "./audio/sounds/ui/blipSelect.wav").ok(),
        )
        .build();
        main_box.add(main_menu);

        let main_box = main_box
            .to_element_builder(0, ctx)
            .with_visuals(super::BUTTON_VIS)
            .with_alignment(ui::Alignment::Center, ui::Alignment::Center)
            .with_padding((25., 25., 25., 25.))
            .build();

        let mut music_player = music::MusicPlayer::from_folder(ctx, "./audio/music/in_game");
        music_player.poll_options();
        music_player.next_song(ctx);

        Ok(Self {
            ui: main_box,
            music_player,
        })
    }
}

impl scene_manager::Scene for GameOverMenu {
    fn update(
        &mut self,
        ctx: &mut good_web_game::Context,
        gfx_ctx: &mut GraphicsContext,
    ) -> Result<scene_manager::SceneSwitch, GameError> {
        let messages = self.ui.manage_messages(ctx, None);

        // restart the game

        if messages.contains(&ui::UiMessage::Triggered(1)) {
            self.music_player.stop(ctx);
            return Ok(mooeye::scene_manager::SceneSwitch::replace(
                super::game_state::GameState::new(
                    ctx,
                    gfx_ctx,
                    super::game_state::GameConfig::default(),
                )?,
                2,
            ));
        }

        // return to main menu

        if messages.contains(&ui::UiMessage::Triggered(2)) {
            self.music_player.stop(ctx);
            return Ok(mooeye::scene_manager::SceneSwitch::replace(
                super::main_menu::MainMenu::new(ctx, gfx_ctx)?,
                2,
            ));
        }
        Ok(mooeye::scene_manager::SceneSwitch::None)
    }

    fn draw(
        &mut self,
        ctx: &mut good_web_game::Context,
        gfx_ctx: &mut GraphicsContext,
        mouse_listen: bool,
    ) -> Result<(), GameError> {
        // music
        self.music_player.check_song(ctx, 0);

        self.ui.draw_to_screen(ctx, gfx_ctx, mouse_listen);

        Ok(())
    }
}
