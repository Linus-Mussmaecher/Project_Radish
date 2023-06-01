pub mod achievement_menu;
pub mod credits_menu;
pub mod highscore_menu;
pub mod options_menu;

use super::game_state;
use super::BUTTON_HOVER_VIS;
use super::BUTTON_VIS;

use ggez::{graphics, GameError};
use mooeye::*;

use crate::PALETTE;

pub struct MainMenu {
    gui: UiElement<()>,
}

impl MainMenu {
    pub fn new(ctx: &ggez::Context) -> Result<Self, GameError> {
        //TODO: audio
        ggez::audio::SoundSource::play_detached(
            &mut ggez::audio::Source::new(ctx, "/audio/music/Song1.mp3")?,
            ctx,
        )
        .unwrap();

        // title

        let title = graphics::Text::new(
            graphics::TextFragment::new("Spellstruck")
                .color(graphics::Color::from_rgb_u32(PALETTE[14])),
        )
        .set_font("Retro")
        .set_scale(48.)
        .to_owned()
        .to_element(1, ctx);

        // play
        let play = graphics::Text::new(
            graphics::TextFragment::new("Play").color(graphics::Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_builder(1, ctx)
        .with_trigger_key(ggez::winit::event::VirtualKeyCode::P)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .build();

        let debug = graphics::Text::new(
            graphics::TextFragment::new("Debug").color(graphics::Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_builder(2, ctx)
        .with_trigger_key(ggez::winit::event::VirtualKeyCode::D)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .build();

        // highscores

        let highscores = graphics::Text::new(
            graphics::TextFragment::new("Highscores")
                .color(graphics::Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_builder(3, ctx)
        .with_trigger_key(ggez::winit::event::VirtualKeyCode::H)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .build();

        // achievement

        let achievements = graphics::Text::new(
            graphics::TextFragment::new("Achievements")
                .color(graphics::Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_builder(4, ctx)
        .with_trigger_key(ggez::winit::event::VirtualKeyCode::A)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .build();

        let options = graphics::Text::new(
            graphics::TextFragment::new("Options").color(graphics::Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_builder(5, ctx)
        .with_trigger_key(ggez::winit::event::VirtualKeyCode::O)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .build();

        let credits = graphics::Text::new(
            graphics::TextFragment::new("Credits").color(graphics::Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_builder(6, ctx)
        .with_trigger_key(ggez::winit::event::VirtualKeyCode::C)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .build();

        let quit = graphics::Text::new(
            graphics::TextFragment::new("Quit").color(graphics::Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_builder(7, ctx)
        .with_trigger_key(ggez::winit::event::VirtualKeyCode::Q)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .build();

        // Container
        let menu_box = mooeye::containers::VerticalBox::new_spaced(25.)
            .to_element_builder(0, ctx)
            .with_child(play)
            .with_child(debug)
            .with_child(highscores)
            .with_child(achievements)
            .with_child(options)
            .with_child(credits)
            .with_child(quit)
            .with_visuals(super::BUTTON_VIS)
            .with_alignment(ui_element::Alignment::Center, ui_element::Alignment::Min)
            .with_padding((25., 25., 25., 25.))
            .build();

        let big_box = mooeye::containers::VerticalBox::new()
            .to_element_builder(0, ctx)
            .with_child(title)
            .with_child(menu_box)
            .with_alignment(ui_element::Alignment::Max, ui_element::Alignment::Min)
            .with_padding((25., 25., 25., 25.))
            .build();

        Ok(Self { gui: big_box })
    }
}

impl scene_manager::Scene for MainMenu {
    fn update(
        &mut self,
        ctx: &mut ggez::Context,
    ) -> Result<mooeye::scene_manager::SceneSwitch, ggez::GameError> {
        let messages = self.gui.manage_messages(ctx, None);

        let mut res = mooeye::scene_manager::SceneSwitch::None;

        if messages.contains(&mooeye::UiMessage::Triggered(1)) {
            res = mooeye::scene_manager::SceneSwitch::replace(
                game_state::GameState::new(ctx, game_state::GameConfig::default())?,
                1,
            );
        }

        if messages.contains(&mooeye::UiMessage::Triggered(2)) {
            res = mooeye::scene_manager::SceneSwitch::replace(
                game_state::GameState::new(ctx, game_state::GameConfig::debug())?,
                1,
            );
        }

        if messages.contains(&mooeye::UiMessage::Triggered(3)) {
            res =
                mooeye::scene_manager::SceneSwitch::push(highscore_menu::HighscoreMenu::new(ctx)?);
        }

        if messages.contains(&mooeye::UiMessage::Triggered(4)) {
            res = mooeye::scene_manager::SceneSwitch::push(achievement_menu::AchievementMenu::new(
                ctx,
            )?);
        }

        if messages.contains(&mooeye::UiMessage::Triggered(5)) {
            res = mooeye::scene_manager::SceneSwitch::push(options_menu::OptionsMenu::new(ctx)?);
        }

        if messages.contains(&mooeye::UiMessage::Triggered(6)) {
            res = mooeye::scene_manager::SceneSwitch::push(credits_menu::CreditsMenu::new(ctx)?);
        }

        if messages.contains(&mooeye::UiMessage::Triggered(7)) {
            res = mooeye::scene_manager::SceneSwitch::Pop(1);
        }

        Ok(res)
    }

    fn draw(&mut self, ctx: &mut ggez::Context, mouse_listen: bool) -> Result<(), ggez::GameError> {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from_rgb_u32(PALETTE[5]));
        canvas.set_sampler(graphics::Sampler::nearest_clamp());

        self.gui.draw_to_screen(ctx, &mut canvas, mouse_listen);

        canvas.finish(ctx)?;
        Ok(())
    }
}
