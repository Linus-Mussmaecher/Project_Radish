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
        let box_vis = mooeye::ui_element::Visuals {
            background: Color::from_rgb_u32(PALETTE[0]),
            border: Color::from_rgb_u32(PALETTE[7]),
            border_width: 3.,
            rounded_corners: 6.,
        };
        let box_hover_vis = mooeye::ui_element::Visuals {
            background: Color::from_rgb_u32(PALETTE[1]),
            border: Color::from_rgb_u32(PALETTE[7]),
            border_width: 3.,
            rounded_corners: 6.,
        };

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
        .with_visuals(box_vis)
        .with_hover_visuals(box_hover_vis)
        .build();

        let back = ggez::graphics::Text::new(
            TextFragment::new("Close").color(Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_builder(2, ctx)
        .with_visuals(box_vis)
        .with_hover_visuals(box_hover_vis)
        .build();

        // Container

        let mut credits_box = mooeye::containers::VerticalBox::new();
        credits_box.add(title)?;
        credits_box.add(reset_bindings)?;
        credits_box.add(back)?;
        credits_box.spacing = 25.;
        let credits_box = credits_box
            .to_element_builder(0, ctx)
            .with_visuals(box_vis)
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

        if messages.contains(&mooeye::UiMessage::Clicked(1)) {
            self.controller = Controller::default();
        }

        if messages.contains(&mooeye::UiMessage::Clicked(2)) {
            self.controller.save_to_file("./data/keymap.toml").expect("Could not save keybindings.");
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
