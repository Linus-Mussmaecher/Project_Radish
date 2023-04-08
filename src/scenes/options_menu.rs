use std::collections::HashSet;

use ggez::{
    graphics::{self, Color, TextFragment},
    Context,
};
use mooeye::{scene_manager::Scene, ui_element::Alignment, UiContent, UiElement};

use crate::PALETTE;

pub struct OptionsMenu {
    gui: UiElement<()>,
}

impl OptionsMenu {
    pub fn new(ctx: &Context) -> Self {
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
        .to_element_measured(0, ctx);

        let text = ggez::graphics::Text::new(
            TextFragment::new("None yet.")
                .color(Color::from_rgb_u32(PALETTE[6]))
                .scale(32.),
        )
        .set_font("Retro")
        .to_owned()
        .to_element_measured(0, ctx);

        let mut back = ggez::graphics::Text::new(
            TextFragment::new("Close").color(Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_measured(1, ctx);
        back.visuals = box_vis;
        back.hover_visuals = Some(box_hover_vis);

        // Container

        let mut credits_box = mooeye::containers::VerticalBox::new();
        credits_box.add(title);
        credits_box.add(text);
        credits_box.add(back);
        credits_box.spacing = 25.;
        let mut credits_box = credits_box.to_element(0);
        credits_box.visuals = box_vis;
        credits_box.layout.x_alignment = Alignment::MIN;
        credits_box.layout.y_alignment = Alignment::MIN;
        credits_box.layout.x_offset = 25.;
        credits_box.layout.y_offset = 25.;
        credits_box.layout.padding = (25., 25., 25., 25.);

        Self { gui: credits_box }
    }
}

impl Scene for OptionsMenu {
    fn update(
        &mut self,
        ctx: &mut ggez::Context,
    ) -> Result<mooeye::scene_manager::SceneSwitch, ggez::GameError> {
        let messages = self.gui.manage_messages(ctx, &HashSet::new());

        if messages.contains(&mooeye::UiMessage::Clicked(1)) {
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
