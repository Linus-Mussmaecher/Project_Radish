use ggez::{
    graphics::{self, Color, TextFragment},
    GameError,
};
use mooeye::{scene_manager::Scene, ui_element::Alignment, UiContent, UiElement};

use crate::{game_state, PALETTE};

pub struct MainMenu {
    gui: UiElement<()>,
}

impl MainMenu {
    pub fn new(ctx: &ggez::Context) -> Result<Self, GameError> {
        // title

        let title = ggez::graphics::Text::new(
            TextFragment::new("PowerDefense").color(Color::from_rgb_u32(PALETTE[14])),
        )
        .set_font("Retro")
        .set_scale(48.)
        .to_owned()
        .to_element(1, ctx);

        // play
        let play = ggez::graphics::Text::new(
            TextFragment::new("Play").color(Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_builder(1, ctx)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .build();

        let tutorial = ggez::graphics::Text::new(
            TextFragment::new("Tutorial").color(Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_builder(2, ctx)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .build();

        // highscores

        let highscores = ggez::graphics::Text::new(
            TextFragment::new("Highscores").color(Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_builder(3, ctx)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .build();

        // achievement

        let achievements = ggez::graphics::Text::new(
            TextFragment::new("Achievements").color(Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_builder(4, ctx)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .build();

        let options = ggez::graphics::Text::new(
            TextFragment::new("Options").color(Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_builder(5, ctx)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .build();

        let credits = ggez::graphics::Text::new(
            TextFragment::new("Credits").color(Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_builder(6, ctx)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .build();

        let quit = ggez::graphics::Text::new(
            TextFragment::new("Quit").color(Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_builder(7, ctx)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .build();

        // Container

        let mut menu_box = mooeye::containers::VerticalBox::new();
        menu_box.add(play)?;
        menu_box.add(tutorial)?;
        menu_box.add(highscores)?;
        menu_box.add(achievements)?;
        menu_box.add(options)?;
        menu_box.add(credits)?;
        menu_box.add(quit)?;
        menu_box.spacing = 25.;
        let menu_box = menu_box
            .to_element_builder(0, ctx)
            .with_visuals(super::BUTTON_VIS)
            .with_alignment(Alignment::Center, Alignment::Min)
            .with_padding((25., 25., 25., 25.))
            .build();

        let mut big_box = mooeye::containers::VerticalBox::new();
        big_box.add(title)?;
        big_box.add(menu_box)?;
        let big_box = big_box
            .to_element_builder(0, ctx)
            .with_alignment(Alignment::Max, Alignment::Min)
            .with_padding((25., 25., 25., 25.))
            .build();

        Ok(Self { gui: big_box })
    }
}

impl Scene for MainMenu {
    fn update(
        &mut self,
        ctx: &mut ggez::Context,
    ) -> Result<mooeye::scene_manager::SceneSwitch, ggez::GameError> {
        let messages = self.gui.manage_messages(ctx, None);

        let mut res = mooeye::scene_manager::SceneSwitch::None;

        if messages.contains(&mooeye::UiMessage::Clicked(1))
            || ctx
                .keyboard
                .is_key_just_pressed(ggez::winit::event::VirtualKeyCode::P)
        {
            res = mooeye::scene_manager::SceneSwitch::replace(game_state::GameState::new(ctx)?, 1);
        }
        if messages.contains(&mooeye::UiMessage::Clicked(2))
            || ctx
                .keyboard
                .is_key_just_pressed(ggez::winit::event::VirtualKeyCode::T)
        {
            res = mooeye::scene_manager::SceneSwitch::replace(game_state::GameState::new(ctx)?, 1);
        }

        if messages.contains(&mooeye::UiMessage::Clicked(3))
            || ctx
                .keyboard
                .is_key_just_pressed(ggez::winit::event::VirtualKeyCode::H)
        {
            res = mooeye::scene_manager::SceneSwitch::push(
                super::highscore_menu::HighscoreMenu::new(ctx)?,
            );
        }

        if messages.contains(&mooeye::UiMessage::Clicked(4))
            || ctx
                .keyboard
                .is_key_just_pressed(ggez::winit::event::VirtualKeyCode::A)
        {
            res = mooeye::scene_manager::SceneSwitch::push(
                super::achievement_menu::AchievementMenu::new(ctx)?,
            );
        }

        if messages.contains(&mooeye::UiMessage::Clicked(5))
            || ctx
                .keyboard
                .is_key_just_pressed(ggez::winit::event::VirtualKeyCode::O)
        {
            res = mooeye::scene_manager::SceneSwitch::push(super::options_menu::OptionsMenu::new(
                ctx,
            )?);
        }

        if messages.contains(&mooeye::UiMessage::Clicked(6))
            || ctx
                .keyboard
                .is_key_just_pressed(ggez::winit::event::VirtualKeyCode::C)
        {
            res = mooeye::scene_manager::SceneSwitch::push(super::credits_menu::CreditsMenu::new(
                ctx,
            )?);
        }

        if messages.contains(&mooeye::UiMessage::Clicked(7))
            || ctx
                .keyboard
                .is_key_just_pressed(ggez::winit::event::VirtualKeyCode::Q)
        {
            res = mooeye::scene_manager::SceneSwitch::Pop(1);
        }

        Ok(res)
    }

    fn draw(&mut self, ctx: &mut ggez::Context, mouse_listen: bool) -> Result<(), ggez::GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from_rgb_u32(PALETTE[5]));
        canvas.set_sampler(graphics::Sampler::nearest_clamp());

        self.gui.draw_to_screen(ctx, &mut canvas, mouse_listen);

        canvas.finish(ctx)?;
        Ok(())
    }
}
