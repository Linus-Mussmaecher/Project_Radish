use ggez::{graphics, GameError};
use mooeye::{ui_element::UiContainer, *};

use super::game_state::achievements;
use crate::PALETTE;

pub struct AchievementMenu {
    gui: UiElement<()>,
}

impl AchievementMenu {
    pub fn new(ctx: &ggez::Context) -> Result<Self, GameError> {
        // title

        let title = graphics::Text::new(
            graphics::TextFragment::new("Achievements")
                .color(graphics::Color::from_rgb_u32(PALETTE[8])),
        )
        .set_font("Retro")
        .set_scale(48.)
        .to_owned()
        .to_element(0, ctx);

        let a_list = achievements::AchievementSet::load(
            ctx,
            achievements::AchievementProgressSource::File("./data/achievements.toml".to_owned()),
        );

        let mut achievements = mooeye::containers::GridBox::new(4, (a_list.list.len() - 1) / 4 + 1);
        for (index, ach) in a_list.list.iter().enumerate() {
            achievements.add(ach.info_element_small(ctx), index % 4, index / 4)?;
        }
        achievements.vertical_spacing = 10.;
        achievements.horizontal_spacing = 10.;

        let achievements = achievements.to_element(0, ctx);

        let reset = graphics::Text::new(
            graphics::TextFragment::new("Reset progress")
                .color(graphics::Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_builder(2, ctx)
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
        .to_element_builder(1, ctx)
        .with_trigger_key(ggez::winit::event::VirtualKeyCode::C)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .with_trigger_sound(ggez::audio::Source::new(ctx, "/audio/sounds/ui/blipSelect.wav").ok())
        .build();

        // Container

        let mut credits_box = mooeye::containers::VerticalBox::new();
        credits_box.add(title);
        credits_box.add(achievements);
        credits_box.add(reset);
        credits_box.add(back);
        credits_box.spacing = 25.;
        let credits_box = credits_box
            .to_element_builder(0, ctx)
            .with_visuals(super::BUTTON_VIS)
            .with_alignment(ui_element::Alignment::Max, ui_element::Alignment::Center)
            .with_offset(-25., 25.)
            .with_padding((25., 25., 25., 25.))
            .build();

        Ok(Self { gui: credits_box })
    }
}

impl scene_manager::Scene for AchievementMenu {
    fn update(
        &mut self,
        ctx: &mut ggez::Context,
    ) -> Result<mooeye::scene_manager::SceneSwitch, ggez::GameError> {
        let messages = self.gui.manage_messages(ctx, None);

        if messages.contains(&mooeye::UiMessage::Triggered(2)) {
            let mut a_list = achievements::AchievementSet::load(
                ctx,
                achievements::AchievementProgressSource::File(
                    "./data/achievements.toml".to_owned(),
                ),
            );
            for achievement in a_list.list.iter_mut() {
                achievement.reset_progress();
            }
            a_list.save();
        }

        if messages.contains(&mooeye::UiMessage::Triggered(1)) {
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
