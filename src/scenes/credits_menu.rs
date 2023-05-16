use ggez::{graphics, GameError};
use mooeye::{*, ui_element::UiContainer};

use crate::PALETTE;

pub struct CreditsMenu {
    gui: UiElement<()>,
}

impl CreditsMenu {
    pub fn new(ctx: &ggez::Context) -> Result<Self, GameError> {
        // title

        let title = graphics::Text::new(
            graphics::TextFragment::new("Credits").color(graphics::Color::from_rgb_u32(PALETTE[8])),
        )
        .set_font("Retro")
        .set_scale(48.)
        .to_owned()
        .to_element(0, ctx);

        let text = graphics::Text::new(
            graphics::TextFragment::new("Programming:\n")
                .color(graphics::Color::from_rgb_u32(PALETTE[7]))
                .scale(36.),
        )
        .add(
            graphics::TextFragment::new("  Linus Mußmächer\n")
                .color(graphics::Color::from_rgb_u32(PALETTE[6]))
                .scale(28.),
        )
        .add(
            graphics::TextFragment::new("Retro Font:\n")
                .color(graphics::Color::from_rgb_u32(PALETTE[7]))
                .scale(36.),
        )
        .add(
            graphics::TextFragment::new("  Daymarius\n")
                .color(graphics::Color::from_rgb_u32(PALETTE[6]))
                .scale(28.),
        )
        .add(
            graphics::TextFragment::new("Color Palette:\n")
                .color(graphics::Color::from_rgb_u32(PALETTE[7]))
                .scale(36.),
        )
        .add(
            graphics::TextFragment::new("  Elefella")
                .color(graphics::Color::from_rgb_u32(PALETTE[6]))
                .scale(28.),
        )
        .set_font("Retro")
        .to_owned()
        .to_element(0, ctx);

        let back = graphics::Text::new(
            graphics::TextFragment::new("Close").color(graphics::Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_builder(1, ctx)
        .with_trigger_key(ggez::winit::event::VirtualKeyCode::C)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .build();

        // Container

        let mut credits_box = mooeye::containers::VerticalBox::new();
        credits_box.add(title);
        credits_box.add(text);
        credits_box.add(back);
        credits_box.spacing = 25.;
        let credits_box = credits_box
            .to_element_builder(0, ctx)
            .with_visuals(super::BUTTON_VIS)
            .with_alignment(ui_element::Alignment::Min, ui_element::Alignment::Min)
            .with_offset(25., 25.)
            .with_padding((25., 25., 25., 25.))
            .build();

        Ok(Self { gui: credits_box })
    }
}

impl scene_manager::Scene for CreditsMenu {
    fn update(
        &mut self,
        ctx: &mut ggez::Context,
    ) -> Result<mooeye::scene_manager::SceneSwitch, ggez::GameError> {
        let messages = self.gui.manage_messages(ctx, None);

        if messages.contains(&mooeye::UiMessage::Triggered(1))
        {
            Ok(mooeye::scene_manager::SceneSwitch::Pop(1))
        } else {
            Ok(mooeye::scene_manager::SceneSwitch::None)
        }
    }

    fn draw(&mut self, ctx: &mut ggez::Context, mouse_listen: bool) -> Result<(), ggez::GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, None);
        canvas.set_sampler(graphics::Sampler::nearest_clamp());

        self.gui.draw_to_screen(ctx, &mut canvas, mouse_listen);

        canvas.finish(ctx)?;
        Ok(())
    }
}
