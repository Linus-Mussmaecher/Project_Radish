use ggez::{graphics, GameError};
use mooeye::{scene_manager, ui, ui::UiContainer, ui::UiContent};

use crate::PALETTE;

use super::super::game_state;

pub struct HighscoreMenu {
    gui: ui::UiElement<()>,
}

impl HighscoreMenu {
    pub fn new(ctx: &ggez::Context) -> Result<Self, GameError> {
        // title

        let title = graphics::Text::new(
            graphics::TextFragment::new("Highscores")
                .color(graphics::Color::from_rgb_u32(PALETTE[8])),
        )
        .set_font("Retro")
        .set_scale(48.)
        .to_owned()
        .to_element(0, ctx);

        // Score display

        let mut highscore_disp = graphics::Text::default();

        game_state::achievements::HIGHSCORES.with(|scores| {
            for (index, (wave, score)) in scores.borrow().scores.iter().enumerate().take(10) {
                highscore_disp.add(
                    graphics::TextFragment::new(format!(
                        "{:02}. {:>3} {:>7}\n",
                        index + 1,
                        *wave,
                        *score
                    ))
                    .color(graphics::Color::from_rgb_u32(PALETTE[6]))
                    .scale(32.),
                );
            }
        });

        let highscore_disp = highscore_disp
            .set_font("Retro_M")
            .to_owned()
            .to_element_builder(0, ctx)
            .with_alignment(ui::Alignment::Center, ui::Alignment::Min)
            .build();

        let reset_scores = graphics::Text::new(
            graphics::TextFragment::new("Reset Highscores")
                .color(graphics::Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(28.)
        .to_owned()
        .to_element_builder(1, ctx)
        .with_tooltip(
            graphics::Text::new(
                graphics::TextFragment::new(
                    "Clears the highscore list.\nAlso resets your quick advance progress.",
                )
                .color(graphics::Color::from_rgb_u32(PALETTE[6])),
            )
            .set_scale(24.)
            .set_font("Retro")
            .to_owned()
            .to_element_builder(0, ctx)
            .with_visuals(super::BUTTON_VIS)
            .build(),
        )
        .with_trigger_key(ggez::winit::event::VirtualKeyCode::R)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .with_trigger_sound(ggez::audio::Source::new(ctx, "/audio/sounds/ui/blipSelect.wav").ok())
        .build();

        let back = graphics::Text::new(
            graphics::TextFragment::new("Close").color(graphics::Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_builder(2, ctx)
        .with_trigger_key(ggez::winit::event::VirtualKeyCode::C)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .with_trigger_sound(ggez::audio::Source::new(ctx, "/audio/sounds/ui/blipSelect.wav").ok())
        .build();

        // Container

        let mut hs_box = ui::containers::VerticalBox::new();
        hs_box.add(title);
        hs_box.add(highscore_disp);
        hs_box.add(reset_scores);
        hs_box.add(back);
        hs_box.spacing = 25.;
        let credits_box = hs_box
            .to_element_builder(0, ctx)
            .with_visuals(super::BUTTON_VIS)
            .with_alignment(ui::Alignment::Max, ui::Alignment::Center)
            .with_offset(-25., 0.)
            .with_padding((25., 25., 25., 25.))
            .build();

        Ok(Self { gui: credits_box })
    }
}

impl scene_manager::Scene for HighscoreMenu {
    fn update(
        &mut self,
        ctx: &mut ggez::Context,
    ) -> Result<scene_manager::SceneSwitch, ggez::GameError> {
        let messages = self.gui.manage_messages(ctx, None);

        if messages.contains(&ui::UiMessage::Triggered(1)) {
            // delete highscores
            std::fs::write("./data/highscores.toml", "")?;
            std::fs::remove_file("./data/quick_config.toml")?;
        }

        if messages.contains(&ui::UiMessage::Triggered(2)) {
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
