use good_web_game::{graphics, GameError};
use mooeye::{scene_manager, ui, ui::UiContainer, ui::UiContent};

use crate::PALETTE;

pub struct CreditsMenu {
    gui: ui::UiElement<()>,
}

impl CreditsMenu {
    pub fn new(ctx: &mut good_web_game::Context) -> Result<Self, GameError> {
        // title

        let title = graphics::Text::new(
            graphics::TextFragment::new("Credits")
                .color(graphics::Color::from_rgb_u32(PALETTE[8]))
                .scale(48.)
                .font(crate::RETRO.with(|f| f.borrow().unwrap())),
        )
        .to_owned()
        .to_element(0, ctx);

        let title_size = 28.;
        let credit_size = 24.;
        let font = crate::RETRO.with(|f| f.borrow().unwrap());

        let text = graphics::Text::new(
            graphics::TextFragment::new("Programming:\n")
                .color(graphics::Color::from_rgb_u32(PALETTE[7]))
                .scale(title_size)
                .font(font),
        )
        .add(
            graphics::TextFragment::new("  Linus Mußmächer\n")
                .color(graphics::Color::from_rgb_u32(PALETTE[6]))
                .scale(credit_size)
                .font(font),
        )
        .add(
            graphics::TextFragment::new("  github.com/Linus-Mussmaecher\n")
                .color(graphics::Color::from_rgb_u32(PALETTE[6]))
                .scale(credit_size)
                .font(font),
        )
        .add(
            graphics::TextFragment::new("Pixel Art:\n")
                .color(graphics::Color::from_rgb_u32(PALETTE[7]))
                .scale(title_size)
                .font(font),
        )
        .add(
            graphics::TextFragment::new("  Linus Mußmächer\n")
                .color(graphics::Color::from_rgb_u32(PALETTE[6]))
                .scale(credit_size)
                .font(font),
        )
        .add(
            graphics::TextFragment::new("Color Palette:\n")
                .color(graphics::Color::from_rgb_u32(PALETTE[7]))
                .scale(title_size)
                .font(font),
        )
        .add(
            graphics::TextFragment::new("  Elefella\n")
                .color(graphics::Color::from_rgb_u32(PALETTE[6]))
                .scale(credit_size)
                .font(font),
        )
        .add(
            graphics::TextFragment::new("Retro Font:\n")
                .color(graphics::Color::from_rgb_u32(PALETTE[7]))
                .scale(title_size)
                .font(font),
        )
        .add(
            graphics::TextFragment::new("  Daymarius\n")
                .color(graphics::Color::from_rgb_u32(PALETTE[6]))
                .scale(credit_size)
                .font(font),
        )
        .add(
            graphics::TextFragment::new("Music:\n")
                .color(graphics::Color::from_rgb_u32(PALETTE[7]))
                .scale(title_size)
                .font(font),
        )
        .add(
            graphics::TextFragment::new("  Abundant Music by Per Nyblom\n")
                .color(graphics::Color::from_rgb_u32(PALETTE[6]))
                .scale(credit_size)
                .font(font),
        )
        .add(
            graphics::TextFragment::new("Sounds:\n")
                .color(graphics::Color::from_rgb_u32(PALETTE[7]))
                .scale(title_size)
                .font(font),
        )
        .add(
            graphics::TextFragment::new("  Various Artists\n")
                .color(graphics::Color::from_rgb_u32(PALETTE[6]))
                .scale(credit_size)
                .font(font),
        )
        .add(
            graphics::TextFragment::new("\nFor full sources & links, see\n/resources folder.")
                .color(graphics::Color::from_rgb_u32(PALETTE[6]))
                .scale(credit_size)
                .font(font),
        )
        .to_owned()
        .to_element(0, ctx);

        let back = graphics::Text::new(
            graphics::TextFragment::new("Close")
                .color(graphics::Color::from_rgb_u32(PALETTE[6]))
                .font(crate::RETRO.with(|f| f.borrow().unwrap()))
                .scale(32.),
        )
        .to_owned()
        .to_element_builder(1, ctx)
        .with_trigger_key(good_web_game::input::keyboard::KeyCode::C)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .with_trigger_sound(
            good_web_game::audio::Source::new(ctx, "./audio/sounds/ui/blipSelect.wav").ok(),
        )
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
        ctx: &mut good_web_game::Context,
        _gfx_ctx: &mut good_web_game::event::GraphicsContext,
    ) -> Result<scene_manager::SceneSwitch, good_web_game::GameError> {
        let messages = self.gui.manage_messages(ctx, None);

        if messages.contains(&ui::UiMessage::Triggered(1)) {
            Ok(scene_manager::SceneSwitch::Pop(1))
        } else {
            Ok(scene_manager::SceneSwitch::None)
        }
    }

    fn draw(
        &mut self,
        ctx: &mut good_web_game::Context,
        gfx_ctx: &mut good_web_game::event::GraphicsContext,
        mouse_listen: bool,
    ) -> Result<(), good_web_game::GameError> {
        self.gui.draw_to_screen(ctx, gfx_ctx, mouse_listen);

        Ok(())
    }
}
