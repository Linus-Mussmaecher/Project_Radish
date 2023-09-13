use good_web_game::{graphics, GameError};
use mooeye::{scene_manager, ui, ui::UiContent};

use crate::PALETTE;

pub struct InGameMenu {
    gui: ui::UiElement<()>,
}

impl InGameMenu {
    pub fn new(
        ctx: &mut good_web_game::Context,
        gfx_ctx: &mut good_web_game::event::GraphicsContext,
    ) -> Result<Self, GameError> {
        // title
        let pause = graphics::Text::new(
            graphics::TextFragment::new("PAUSED")
                .color(graphics::Color::from_rgb_u32(PALETTE[6]))
                .scale(32.)
                .font(crate::RETRO.with(|f| f.borrow().unwrap())),
        )
        .to_owned()
        .to_element_builder(0, ctx)
        .build();

        let resume = graphics::Text::new(
            graphics::TextFragment::new("Resume")
                .color(graphics::Color::from_rgb_u32(PALETTE[6]))
                .scale(32.)
                .font(crate::RETRO.with(|f| f.borrow().unwrap())),
        )
        .to_owned()
        .to_element_builder(1, ctx)
        .with_trigger_key(good_web_game::input::keyboard::KeyCode::F10)
        .with_trigger_key(good_web_game::input::keyboard::KeyCode::R)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .with_trigger_sound(
            good_web_game::audio::Source::new(ctx, "./audio/sounds/ui/blipSelect.wav").ok(),
        )
        .build();

        let achievements = graphics::Text::new(
            graphics::TextFragment::new("Achievements")
                .color(graphics::Color::from_rgb_u32(PALETTE[6]))
                .scale(32.)
                .font(crate::RETRO.with(|f| f.borrow().unwrap())),
        )
        .to_owned()
        .to_element_builder(2, ctx)
        .with_trigger_key(good_web_game::input::keyboard::KeyCode::A)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .with_trigger_sound(
            good_web_game::audio::Source::new(ctx, "./audio/sounds/ui/blipSelect.wav").ok(),
        )
        .build();

        let options = graphics::Text::new(
            graphics::TextFragment::new("Options")
                .color(graphics::Color::from_rgb_u32(PALETTE[6]))
                .scale(32.)
                .font(crate::RETRO.with(|f| f.borrow().unwrap())),
        )
        .to_owned()
        .to_element_builder(3, ctx)
        .with_trigger_key(good_web_game::input::keyboard::KeyCode::O)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .with_trigger_sound(
            good_web_game::audio::Source::new(ctx, "./audio/sounds/ui/blipSelect.wav").ok(),
        )
        .build();

        let main_menu = graphics::Text::new(
            graphics::TextFragment::new("Return to Main Menu")
                .color(graphics::Color::from_rgb_u32(PALETTE[6]))
                .scale(32.)
                .font(crate::RETRO.with(|f| f.borrow().unwrap())),
        )
        .to_owned()
        .to_element_builder(4, ctx)
        .with_trigger_key(good_web_game::input::keyboard::KeyCode::M)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .with_trigger_sound(
            good_web_game::audio::Source::new(ctx, "./audio/sounds/ui/blipSelect.wav").ok(),
        )
        .build();

        // Container

        let menu_box = ui::containers::VerticalBox::new_spaced(25.)
            .to_element_builder(0, ctx)
            .with_child(pause)
            .with_child(resume)
            .with_child(achievements)
            .with_child(options)
            .with_child(main_menu)
            .with_visuals(super::BUTTON_VIS)
            .with_alignment(ui::Alignment::Center, ui::Alignment::Center)
            .with_padding((25., 25., 25., 25.))
            .build();

        Ok(Self { gui: menu_box })
    }
}

impl scene_manager::Scene for InGameMenu {
    fn update(
        &mut self,
        ctx: &mut good_web_game::Context,
        gfx_ctx: &mut good_web_game::event::GraphicsContext,
    ) -> Result<scene_manager::SceneSwitch, good_web_game::GameError> {
        let messages = self.gui.manage_messages(ctx, None);

        let mut res = scene_manager::SceneSwitch::None;

        if messages.contains(&ui::UiMessage::Triggered(1)) {
            res = scene_manager::SceneSwitch::pop(1);
        }

        if messages.contains(&ui::UiMessage::Triggered(2)) {
            res = mooeye::scene_manager::SceneSwitch::push(
                crate::scenes::main_menu::achievement_menu::AchievementMenu::new(ctx)?,
            );
        }

        if messages.contains(&ui::UiMessage::Triggered(3)) {
            res = scene_manager::SceneSwitch::push(
                crate::scenes::main_menu::options_menu::OptionsMenu::new(ctx)?,
            );
        }

        if messages.contains(&ui::UiMessage::Triggered(4)) {
            res = scene_manager::SceneSwitch::replace(
                crate::scenes::main_menu::MainMenu::new(ctx, gfx_ctx)?,
                2,
            );
        }

        Ok(res)
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
