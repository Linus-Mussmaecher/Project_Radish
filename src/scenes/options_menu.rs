use ggez::{
    graphics::{self, Color, TextFragment},
    Context, GameError,
};
use mooeye::{scene_manager::Scene, ui_element::Alignment, UiContent, UiElement};

use crate::{game_state::Controller, PALETTE};

pub struct OptionsMenu {
    gui: UiElement<()>,
    controller: crate::game_state::Controller,
}

impl OptionsMenu {
    pub fn new(ctx: &Context) -> Result<Self, GameError> {
        // title

        let title = ggez::graphics::Text::new(
            TextFragment::new("Options").color(Color::from_rgb_u32(PALETTE[8])),
        )
        .set_font("Retro")
        .set_scale(48.)
        .to_owned()
        .to_element(0, ctx);

        let reset_bindings = ggez::graphics::Text::new(
            TextFragment::new("Reset Keybindings").color(Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_builder(1, ctx)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
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

        let mut options_box = mooeye::containers::VerticalBox::new();
        options_box.add(title)?;
        options_box.add(reset_bindings)?;
        options_box.add(back)?;
        options_box.spacing = 25.;
        let credits_box = options_box
            .to_element_builder(0, ctx)
            .with_visuals(super::BUTTON_VIS)
            .with_alignment(Alignment::Min, Alignment::Min)
            .with_offset(25., 25.)
            .with_padding((25., 25., 25., 25.))
            .build();

        Ok(Self {
            gui: credits_box,
            controller: Controller::from_path("./data/keymap.toml").unwrap_or_default(),
        })
    }
}

impl Scene for OptionsMenu {
    fn update(
        &mut self,
        ctx: &mut ggez::Context,
    ) -> Result<mooeye::scene_manager::SceneSwitch, ggez::GameError> {
        let messages = self.gui.manage_messages(ctx, None);

        if messages.contains(&mooeye::UiMessage::Clicked(1))
            || ctx
                .keyboard
                .is_key_just_pressed(ggez::winit::event::VirtualKeyCode::R)
        {
            self.controller = Controller::default();
        }

        if messages.contains(&mooeye::UiMessage::Clicked(2))
            || ctx
                .keyboard
                .is_key_just_pressed(ggez::winit::event::VirtualKeyCode::C)
        {
            if self.controller.save_to_file("./data/keymap.toml").is_err() {
                println!("[WARNING] Could not save keybindings.")
            }
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
