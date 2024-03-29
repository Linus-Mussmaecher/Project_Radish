use ggez::{graphics, GameError};
use mooeye::{scene_manager, ui, ui::UiContainer, ui::UiContent};

use crate::PALETTE;

pub struct CreditsMenu {
    gui: ui::UiElement<()>,
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

        let title_size = 28.;
        let credit_size = 24.;

        let text = graphics::Text::new(
            graphics::TextFragment::new("Programming:\n")
                .color(graphics::Color::from_rgb_u32(PALETTE[7]))
                .scale(title_size),
        )
        .add(
            graphics::TextFragment::new("  Linus Mußmächer\n")
                .color(graphics::Color::from_rgb_u32(PALETTE[6]))
                .scale(credit_size),
        )
        .add(
            graphics::TextFragment::new("  github.com/Linus-Mussmaecher\n")
                .color(graphics::Color::from_rgb_u32(PALETTE[6]))
                .scale(credit_size),
        )
        .add(
            graphics::TextFragment::new("Pixel Art:\n")
                .color(graphics::Color::from_rgb_u32(PALETTE[7]))
                .scale(title_size),
        )
        .add(
            graphics::TextFragment::new("  Linus Mußmächer\n")
                .color(graphics::Color::from_rgb_u32(PALETTE[6]))
                .scale(credit_size),
        )
        .add(
            graphics::TextFragment::new("Color Palette:\n")
                .color(graphics::Color::from_rgb_u32(PALETTE[7]))
                .scale(title_size),
        )
        .add(
            graphics::TextFragment::new("  Elefella\n")
                .color(graphics::Color::from_rgb_u32(PALETTE[6]))
                .scale(credit_size),
        )
        .add(
            graphics::TextFragment::new("Retro Font:\n")
                .color(graphics::Color::from_rgb_u32(PALETTE[7]))
                .scale(title_size),
        )
        .add(
            graphics::TextFragment::new("  Daymarius\n")
                .color(graphics::Color::from_rgb_u32(PALETTE[6]))
                .scale(credit_size),
        )
        .add(
            graphics::TextFragment::new("Music:\n")
                .color(graphics::Color::from_rgb_u32(PALETTE[7]))
                .scale(title_size),
        )
        .add(
            graphics::TextFragment::new("  Abundant Music by Per Nyblom\n")
                .color(graphics::Color::from_rgb_u32(PALETTE[6]))
                .scale(credit_size),
        )
        .add(
            graphics::TextFragment::new("Sounds:\n")
                .color(graphics::Color::from_rgb_u32(PALETTE[7]))
                .scale(title_size),
        )
        .add(
            graphics::TextFragment::new("  Various Artists\n")
                .color(graphics::Color::from_rgb_u32(PALETTE[6]))
                .scale(credit_size),
        )
        .add(
            graphics::TextFragment::new("\nFor full sources & links, see\n/resources folder.")
                .color(graphics::Color::from_rgb_u32(PALETTE[6]))
                .scale(credit_size),
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
        .with_trigger_sound(ggez::audio::Source::new(ctx, "/audio/sounds/ui/blipSelect.wav").ok())
        .build();

        // Container

        let mut credits_box = ui::containers::VerticalBox::new();
        credits_box.add(title);
        credits_box.add(text);
        credits_box.add(back);
        credits_box.spacing = 25.;
        let credits_box = credits_box
            .to_element_builder(0, ctx)
            .with_visuals(super::BUTTON_VIS)
            .with_alignment(ui::Alignment::Max, ui::Alignment::Center)
            .with_offset(-25., 0.)
            .with_padding((25., 25., 25., 25.))
            .build();

        Ok(Self { gui: credits_box })
    }
}

impl scene_manager::Scene for CreditsMenu {
    fn update(
        &mut self,
        ctx: &mut ggez::Context,
    ) -> Result<scene_manager::SceneSwitch, ggez::GameError> {
        let messages = self.gui.manage_messages(ctx, None);

        if messages.contains(&ui::UiMessage::Triggered(1)) {
            Ok(scene_manager::SceneSwitch::Pop(1))
        } else {
            Ok(scene_manager::SceneSwitch::None)
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
