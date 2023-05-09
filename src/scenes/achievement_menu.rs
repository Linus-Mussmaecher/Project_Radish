use ggez::{glam::Vec2, graphics, GameError};
use mooeye::*;

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

        let a_list = crate::game_state::Achievement::load_set(ctx);

        let mut achievements = mooeye::containers::GridBox::new(4, (a_list.len() - 1) / 4 + 1);
        for (index, ach) in a_list.iter().enumerate() {
            let achievement = if ach.is_achieved() && ach.get_icon().is_some() {
                ach.get_icon().clone().unwrap()
            } else {
                println!("Nope, {}/{}: {}", ach.get_progress(), ach.get_target(), ach.is_achieved());
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

            achievements.add(achievement, index % 4,  index/ 4)?;
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
        credits_box.add(title)?;
        credits_box.add(achievements)?;
        credits_box.add(back)?;
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
