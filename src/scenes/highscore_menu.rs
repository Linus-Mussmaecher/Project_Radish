use ggez::{
    graphics::{self, Color, TextFragment},
    Context, GameError,
};
use mooeye::{scene_manager::Scene, ui_element::Alignment, UiContent, UiElement};

use crate::PALETTE;

use super::game_over_menu;

pub struct HighscoreMenu {
    gui: UiElement<()>,
}

impl HighscoreMenu {
    pub fn new(ctx: &Context) -> Result<Self, GameError> {
        // title

        let title = ggez::graphics::Text::new(
            TextFragment::new("Highscores").color(Color::from_rgb_u32(PALETTE[8])),
        )
        .set_font("Retro")
        .set_scale(48.)
        .to_owned()
        .to_element(0, ctx);

        // Score display

        let mut highscore_disp = ggez::graphics::Text::new("");

        for (index, value) in game_over_menu::load_highscores().iter().enumerate().take(5) {
            highscore_disp.add(
                graphics::TextFragment::new(format!("  {:02}.{:>5}\n", index + 1, *value))
                    .color(Color::from_rgb_u32(PALETTE[6]))
                    .scale(32.),
            );
        }

        let highscore_disp = highscore_disp
            .set_font("Retro")
            .to_owned()
            .to_element_builder(0, ctx)
            .with_alignment(
                mooeye::ui_element::Alignment::Center,
                mooeye::ui_element::Alignment::Min,
            )
            .build();

        let back = ggez::graphics::Text::new(
            TextFragment::new("Close").color(Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_builder(2, ctx)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .build();

        // Container

        let mut hs_box = mooeye::containers::VerticalBox::new();
        hs_box.add(title)?;
        hs_box.add(highscore_disp)?;
        hs_box.add(back)?;
        hs_box.spacing = 25.;
        let credits_box = hs_box
            .to_element_builder(0, ctx)
            .with_visuals(super::BUTTON_VIS)
            .with_alignment(Alignment::Min, Alignment::Min)
            .with_offset(25., 25.)
            .with_padding((25., 25., 25., 25.))
            .build();

        Ok(Self { gui: credits_box })
    }
}

impl Scene for HighscoreMenu {
    fn update(
        &mut self,
        ctx: &mut ggez::Context,
    ) -> Result<mooeye::scene_manager::SceneSwitch, ggez::GameError> {
        let messages = self.gui.manage_messages(ctx, None);

        if messages.contains(&mooeye::UiMessage::Clicked(2))
            || ctx
                .keyboard
                .is_key_just_pressed(ggez::winit::event::VirtualKeyCode::C)
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
