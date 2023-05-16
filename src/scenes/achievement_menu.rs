use ggez::{glam::Vec2, graphics, GameError};
use mooeye::{*, ui_element::UiContainer};

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

        let a_list = crate::game_state::achievements::AchievementSet::load(ctx);

        let mut achievements = mooeye::containers::GridBox::new(4, (a_list.list.len() - 1) / 4 + 1);
        for (index, ach) in a_list.list.iter().enumerate() {
            let achievement = if ach.is_achieved() && ach.get_icon().is_some() {
                ach.get_icon().clone().unwrap()
            } else {
                graphics::Image::from_path(ctx, "/sprites/ui/lock.png")?
            }
            .to_element_builder(0, ctx)
            .with_visuals(super::BUTTON_VIS)
            .scaled(4., 4.)
            .with_tooltip(
                graphics::Text::new(
                    graphics::TextFragment::new(ach.get_name())
                        .color(graphics::Color::from_rgb_u32(PALETTE[7]))
                        .scale(28.),
                )
                .add("\n")
                .add(
                    graphics::TextFragment::new(ach.get_desc())
                        .color(graphics::Color::from_rgb_u32(PALETTE[6]))
                        .scale(20.),
                )
                .add(
                    graphics::TextFragment::new(format!(
                        "\n  {} / {}",
                        ach.get_progress(),
                        ach.get_target()
                    ))
                    .color(graphics::Color::from_rgb_u32(PALETTE[6]))
                    .scale(20.),
                )
                .set_font("Retro")
                .set_wrap(true)
                .set_bounds(Vec2::new(300., 200.))
                .to_owned()
                .to_element_builder(0, ctx)
                .with_visuals(super::BUTTON_VIS)
                .build(),
            )
            .build();

            achievements.add(achievement, index % 4, index / 4)?;
        }
        achievements.vertical_spacing = 10.;
        achievements.horizontal_spacing = 10.;

        let achievements = achievements.to_element(0, ctx);

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
        credits_box.add(achievements);
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

impl scene_manager::Scene for AchievementMenu {
    fn update(
        &mut self,
        ctx: &mut ggez::Context,
    ) -> Result<mooeye::scene_manager::SceneSwitch, ggez::GameError> {
        let messages = self.gui.manage_messages(ctx, None);

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

pub fn achievement_info<T: Copy + Eq + std::hash::Hash + 'static>(
    ach: &crate::game_state::Achievement,
    ctx: &ggez::Context,
) -> Result<mooeye::UiElement<T>, GameError> {
    let mut ach_box = containers::HorizontalBox::new();

    if let Ok(trophy) = graphics::Image::from_path(ctx, "/sprites/achievements/a0_16_16.png") {
        ach_box.add(trophy.to_element_builder(0, ctx).scaled(4., 4.).build());
    }

    ach_box.add(
        graphics::Text::new(
            graphics::TextFragment::new(ach.get_name())
                .color(graphics::Color::from_rgb_u32(PALETTE[7]))
                .scale(28.),
        )
        .add("\n")
        .add(
            graphics::TextFragment::new(ach.get_desc())
                .color(graphics::Color::from_rgb_u32(PALETTE[6]))
                .scale(20.),
        )
        .add(
            graphics::TextFragment::new(format!(
                "\n  {} / {}",
                ach.get_progress(),
                ach.get_target()
            ))
            .color(graphics::Color::from_rgb_u32(PALETTE[6]))
            .scale(20.),
        )
        .set_font("Retro")
        .set_wrap(true)
        .set_bounds(Vec2::new(300., 200.))
        .to_owned()
        .to_element_builder(0, ctx)
        .build(),
    );

    if let Some(icon) = ach.get_icon() {
        ach_box.add(
            icon.clone()
                .to_element_builder(0, ctx)
                .scaled(4., 4.)
                .build(),
        );
    }

    let ach_box = ach_box
        .to_element_builder(0, ctx)
        .with_visuals(super::BUTTON_VIS)
        .build();

    Ok(containers::DurationBox::new(std::time::Duration::from_secs(15), ach_box).to_element(0, ctx))
}
