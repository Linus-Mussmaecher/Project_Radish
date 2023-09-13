use good_web_game::{graphics, GameError};
use mooeye::{scene_manager, ui, ui::UiContainer, ui::UiContent};

use super::game_state::achievements;
use crate::PALETTE;

pub struct AchievementMenu {
    gui: ui::UiElement<()>,
}

impl AchievementMenu {
    pub fn new(
        ctx: &mut good_web_game::Context,
        gfx_ctx: &mut good_web_game::event::GraphicsContext,
    ) -> Result<Self, GameError> {
        // title

        let title = graphics::Text::new(
            graphics::TextFragment::new("Achievements")
                .color(graphics::Color::from_rgb_u32(PALETTE[8]))
                .font(crate::RETRO.with(|f| f.borrow().unwrap()))
                .scale(48.),
        )
        .to_owned()
        .to_element(0, ctx);

        let a_list = achievements::AchievementSet::load(
            ctx,
            gfx_ctx,
            achievements::AchievementProgressSource::Cache,
        );

        let mut achievements = ui::containers::GridBox::new(4, (a_list.list.len() - 1) / 4 + 1);
        for (index, ach) in a_list.list.iter().enumerate() {
            achievements.add(ach.info_element_small(ctx, gfx_ctx), index % 4, index / 4)?;
        }
        achievements.vertical_spacing = 10.;
        achievements.horizontal_spacing = 10.;

        let achievements = achievements.to_element(0, ctx);

        let reset = graphics::Text::new(
            graphics::TextFragment::new("Reset progress")
                .color(graphics::Color::from_rgb_u32(PALETTE[6]))
                .scale(28.)
                .font(crate::RETRO.with(|f| f.borrow().unwrap())),
        )
        .to_owned()
        .to_element_builder(2, ctx)
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
                .font(crate::RETRO.with(|f| f.borrow().unwrap())),
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
        credits_box.add(achievements);
        credits_box.add(reset);
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

impl scene_manager::Scene for AchievementMenu {
    fn update(
        &mut self,
        ctx: &mut good_web_game::Context,
        gfx_ctx: &mut good_web_game::event::GraphicsContext,
    ) -> Result<scene_manager::SceneSwitch, good_web_game::GameError> {
        let messages = self.gui.manage_messages(ctx, None);

        if messages.contains(&ui::UiMessage::Triggered(2)) {
            let mut a_list = achievements::AchievementSet::load(
                ctx,
                gfx_ctx,
                achievements::AchievementProgressSource::Cache,
            );
            for achievement in a_list.list.iter_mut() {
                achievement.reset_progress();
            }
            a_list.save();
        }

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
