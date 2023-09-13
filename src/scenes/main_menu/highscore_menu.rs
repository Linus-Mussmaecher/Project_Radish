use good_web_game::{graphics, GameError};
use mooeye::{scene_manager, ui, ui::UiContainer, ui::UiContent};

use crate::PALETTE;

use super::super::game_state;

pub struct HighscoreMenu {
    gui: ui::UiElement<()>,
}

impl HighscoreMenu {
    pub fn new(ctx: &mut good_web_game::Context) -> Result<Self, GameError> {
        // title

        let title = graphics::Text::new(
            graphics::TextFragment::new("Highscores")
                .color(graphics::Color::from_rgb_u32(PALETTE[8]))
                .font(crate::RETRO.with(|f| f.borrow().unwrap()))
                .scale(48.),
        )
        .to_owned()
        .to_element(0, ctx);

        // Score display

        let mut highscore_disp = graphics::Text::default();
        let retro = crate::RETRO.with(|f| f.borrow().unwrap());
        let retro_m = crate::RETRO_M.with(|f| f.borrow().unwrap());

        game_state::achievements::HIGHSCORES.with(|scores| {
            for (index, (_, score)) in scores.borrow().iter().enumerate().take(10) {
                highscore_disp.add(
                    graphics::TextFragment::new(format!("{:02}.{:>7}\n", index + 1, *score))
                        .color(graphics::Color::from_rgb_u32(PALETTE[6]))
                        .scale(32.)
                        .font(retro_m),
                );
            }
        });

        let highscore_disp = highscore_disp
            .to_owned()
            .to_element_builder(0, ctx)
            .with_alignment(ui::Alignment::Center, ui::Alignment::Min)
            .build();

        let reset_scores = graphics::Text::new(
            graphics::TextFragment::new("Reset Highscores")
                .color(graphics::Color::from_rgb_u32(PALETTE[6]))
                .scale(28.)
                .font(retro),
        )
        .to_owned()
        .to_element_builder(1, ctx)
        .with_tooltip(
            graphics::Text::new(
                graphics::TextFragment::new(
                    "Clears the highscore list.\nAlso resets your quick advance progress.",
                )
                .color(graphics::Color::from_rgb_u32(PALETTE[6]))
                .scale(24.)
                .font(retro),
            )
            .to_owned()
            .to_element_builder(0, ctx)
            .with_visuals(super::BUTTON_VIS)
            .build(),
        )
        .with_trigger_key(good_web_game::input::keyboard::KeyCode::R)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .with_trigger_sound(
            good_web_game::audio::Source::new(ctx, "./audio/sounds/ui/blipSelect.wav").ok(),
        )
        .build();

        let back = graphics::Text::new(
            graphics::TextFragment::new("Close")
                .color(graphics::Color::from_rgb_u32(PALETTE[6]))
                .scale(32.)
                .font(retro),
        )
        .to_owned()
        .to_element_builder(2, ctx)
        .with_trigger_key(good_web_game::input::keyboard::KeyCode::C)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .with_trigger_sound(
            good_web_game::audio::Source::new(ctx, "./audio/sounds/ui/blipSelect.wav").ok(),
        )
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
        ctx: &mut good_web_game::Context,
        _gfx_ctx: &mut good_web_game::event::GraphicsContext,
    ) -> Result<scene_manager::SceneSwitch, good_web_game::GameError> {
        let messages = self.gui.manage_messages(ctx, None);

        if messages.contains(&ui::UiMessage::Triggered(1)) {
            // delete highscores
            std::fs::write("./data/highscores.toml", "")?;
        }

        if messages.contains(&ui::UiMessage::Triggered(2)) {
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
