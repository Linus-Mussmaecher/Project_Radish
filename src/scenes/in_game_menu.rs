use ggez::{
    graphics::{self, Color, TextFragment},
    GameError,
};
use mooeye::*;

use crate::PALETTE;

pub struct InGameMenu {
    gui: UiElement<()>,
}

impl InGameMenu {
    pub fn new(ctx: &ggez::Context) -> Result<Self, GameError> {
        // title
        let pause = ggez::graphics::Text::new(
            TextFragment::new("PAUSED").color(Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_builder(0, ctx)
        .build();

        let resume = ggez::graphics::Text::new(
            TextFragment::new("Resume").color(Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_builder(1, ctx)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .build();

        let main_menu = ggez::graphics::Text::new(
            TextFragment::new("Return to Main Menu").color(Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_builder(2, ctx)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .build();

        let quit = ggez::graphics::Text::new(
            TextFragment::new("Quit Game").color(Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_builder(3, ctx)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .build();

        // Container

        let mut menu_box: containers::VerticalBox<()> = containers::VerticalBox::new();
        menu_box.add(pause)?;
        menu_box.add(resume)?;
        menu_box.add(main_menu)?;
        menu_box.add(quit)?;
        menu_box.spacing = 25.;
        let menu_box = menu_box
            .to_element_builder(0, ctx)
            .with_visuals(super::BUTTON_VIS)
            .with_alignment(ui_element::Alignment::Center, ui_element::Alignment::Center)
            .with_padding((25., 25., 25., 25.))
            .build();

        Ok(Self { gui: menu_box })
    }
}

impl scene_manager::Scene for InGameMenu {
    fn update(
        &mut self,
        ctx: &mut ggez::Context,
    ) -> Result<scene_manager::SceneSwitch, ggez::GameError> {
        let messages = self.gui.manage_messages(ctx, None);

        let mut res = scene_manager::SceneSwitch::None;

        if messages.contains(&UiMessage::Clicked(1))
            || ctx
                .keyboard
                .is_key_just_pressed(ggez::winit::event::VirtualKeyCode::F10)
            || ctx
                .keyboard
                .is_key_just_pressed(ggez::winit::event::VirtualKeyCode::R)
        {
            res = scene_manager::SceneSwitch::pop(1);
        }

        if messages.contains(&UiMessage::Clicked(2))
            || ctx
                .keyboard
                .is_key_just_pressed(ggez::winit::event::VirtualKeyCode::M)
        {
            res = scene_manager::SceneSwitch::replace(super::main_menu::MainMenu::new(ctx)?, 2);
        }

        if messages.contains(&UiMessage::Clicked(3))
            || ctx
                .keyboard
                .is_key_just_pressed(ggez::winit::event::VirtualKeyCode::Q)
        {
            res = scene_manager::SceneSwitch::pop(2);
        }

        Ok(res)
    }

    fn draw(&mut self, ctx: &mut ggez::Context, mouse_listen: bool) -> Result<(), ggez::GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, None);
        canvas.set_sampler(graphics::Sampler::nearest_clamp());

        self.gui.draw_to_screen(ctx, &mut canvas, mouse_listen);

        canvas.finish(ctx)?;
        Ok(())
    }
}
