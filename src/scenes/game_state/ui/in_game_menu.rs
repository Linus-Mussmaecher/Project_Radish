use ggez::{graphics, GameError};
use mooeye::*;

use crate::PALETTE;

pub struct InGameMenu {
    gui: UiElement<()>,
}

impl InGameMenu {
    pub fn new(ctx: &ggez::Context) -> Result<Self, GameError> {
        // title
        let pause = graphics::Text::new(
            graphics::TextFragment::new("PAUSED").color(graphics::Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_builder(0, ctx)
        .build();

        let resume = graphics::Text::new(
            graphics::TextFragment::new("Resume").color(graphics::Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_builder(1, ctx)
        .with_trigger_key(ggez::winit::event::VirtualKeyCode::F10)
        .with_trigger_key(ggez::winit::event::VirtualKeyCode::R)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .build();

        let achievements = graphics::Text::new(
            graphics::TextFragment::new("Achievements")
                .color(graphics::Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_builder(2, ctx)
        .with_trigger_key(ggez::winit::event::VirtualKeyCode::A)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .build();

        let main_menu = graphics::Text::new(
            graphics::TextFragment::new("Return to Main Menu")
                .color(graphics::Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_builder(3, ctx)
        .with_trigger_key(ggez::winit::event::VirtualKeyCode::M)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .build();

        let quit = graphics::Text::new(
            graphics::TextFragment::new("Quit Game")
                .color(graphics::Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_builder(4, ctx)
        .with_trigger_key(ggez::winit::event::VirtualKeyCode::Q)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .build();

        // Container

        let menu_box = containers::VerticalBox::new_spaced(25.)
            .to_element_builder(0, ctx)
            .with_child(pause)
            .with_child(resume)
            .with_child(achievements)
            .with_child(main_menu)
            .with_child(quit)
            .with_visuals(super::BUTTON_VIS)
            .with_alignment(ui_element::Alignment::Center, ui_element::Alignment::Center)
            .with_padding((25., 25., 25., 25.))
            .build();

        Ok(Self { gui: menu_box })
    }
}

impl scene_manager::Scene for InGameMenu {
    fn update(
        &mut self,
        ctx: &mut ggez::Context,
    ) -> Result<scene_manager::SceneSwitch, ggez::GameError> {
        let messages = self.gui.manage_messages(ctx, None);

        let mut res = scene_manager::SceneSwitch::None;

        if messages.contains(&UiMessage::Triggered(1)) {
            res = scene_manager::SceneSwitch::pop(1);
        }

        if messages.contains(&mooeye::UiMessage::Triggered(2)) {
            res = mooeye::scene_manager::SceneSwitch::push(crate::scenes::main_menu::achievement_menu::AchievementMenu::new(
                ctx,
            )?);
        }

        if messages.contains(&UiMessage::Triggered(3)) {
            res = scene_manager::SceneSwitch::replace(
                crate::scenes::main_menu::MainMenu::new(ctx)?,
                2,
            );
        }

        if messages.contains(&UiMessage::Triggered(4)) {
            res = scene_manager::SceneSwitch::pop(2);
        }

        Ok(res)
    }

    fn draw(&mut self, ctx: &mut ggez::Context, mouse_listen: bool) -> Result<(), ggez::GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, None);
        canvas.set_sampler(graphics::Sampler::nearest_clamp());

        self.gui.draw_to_screen(ctx, &mut canvas, mouse_listen);

        canvas.finish(ctx)?;
        Ok(())
    }
}
