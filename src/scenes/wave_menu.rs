use ggez::{graphics, GameError};
use mooeye::*;

use crate::PALETTE;

pub struct WaveMenu {
    gui: UiElement<()>,
}

impl WaveMenu {
    pub fn new(ctx: &ggez::Context, wave: i32) -> Result<Self, GameError> {
        // title
        let wave_info = graphics::Text::new(
            graphics::TextFragment::new(format!("You have survived wave {}.", wave)).color(graphics::Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_builder(0, ctx)
        .build();

        let purchase = graphics::Text::new(
            graphics::TextFragment::new("Purchase additional\nspell slot.").color(graphics::Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(28.)
        .to_owned()
        .to_element_builder(1, ctx)
        .as_shrink()
        .with_trigger_key(ggez::winit::event::VirtualKeyCode::F10)
        .with_trigger_key(ggez::winit::event::VirtualKeyCode::R)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .build();

        let next = graphics::Text::new(
            graphics::TextFragment::new("Next Wave").color(graphics::Color::from_rgb_u32(PALETTE[6])),
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

        // Container

        let mut menu_box: containers::VerticalBox<()> = containers::VerticalBox::new();
        menu_box.add(wave_info)?;
        menu_box.add(purchase)?;
        menu_box.add(next)?;
        menu_box.spacing = 25.;
        let menu_box = menu_box
            .to_element_builder(0, ctx)
            .with_visuals(super::BUTTON_VIS)
            .with_alignment(ui_element::Alignment::Center, ui_element::Alignment::Center)
            .with_padding((25., 25., 25., 25.))
            .build();

        Ok(Self { gui: menu_box })
    }
}

impl scene_manager::Scene for WaveMenu {
    fn update(
        &mut self,
        ctx: &mut ggez::Context,
    ) -> Result<scene_manager::SceneSwitch, ggez::GameError> {
        let messages = self.gui.manage_messages(ctx, None);

        let mut res = scene_manager::SceneSwitch::None;

        if messages.contains(&UiMessage::Triggered(1))
        {
            res = scene_manager::SceneSwitch::pop(1);
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
