use ggez::{
    graphics::{self, Color, TextFragment},
    GameError,
};
use mooeye::{scene_manager::Scene, ui_element::Alignment, UiContent, UiElement};

use crate::PALETTE;

pub struct InGameMenu {
    gui: UiElement<()>,
}

impl InGameMenu {
    pub fn new(ctx: &ggez::Context) -> Result<Self, GameError> {
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
        .with_visuals(box_vis)
        .with_hover_visuals(box_hover_vis)
        .build();

        let main_menu = ggez::graphics::Text::new(
            TextFragment::new("Return to Main Menu").color(Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_builder(2, ctx)
        .with_visuals(box_vis)
        .with_hover_visuals(box_hover_vis)
        .build();

        let quit = ggez::graphics::Text::new(
            TextFragment::new("Quit Game").color(Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_builder(3, ctx)
        .with_visuals(box_vis)
        .with_hover_visuals(box_hover_vis)
        .build();

        // Container

        let mut menu_box = mooeye::containers::VerticalBox::new();
        menu_box.add(pause)?;
        menu_box.add(resume)?;
        menu_box.add(main_menu)?;
        menu_box.add(quit)?;
        menu_box.spacing = 25.;
        let menu_box = menu_box
            .to_element_builder(0, ctx)
            .with_visuals(box_vis)
            .with_alignment(Alignment::Center, Alignment::Center)
            .with_padding((25., 25., 25., 25.))
            .build();

        Ok(Self { gui: menu_box })
    }
}

impl Scene for InGameMenu {
    fn update(
        &mut self,
        ctx: &mut ggez::Context,
    ) -> Result<mooeye::scene_manager::SceneSwitch, ggez::GameError> {
        let messages = self.gui.manage_messages(ctx, None);

        let mut res = mooeye::scene_manager::SceneSwitch::None;

        if messages.contains(&mooeye::UiMessage::Clicked(1))
            || ctx
                .keyboard
                .is_key_just_pressed(ggez::winit::event::VirtualKeyCode::F10)
        {
            res = mooeye::scene_manager::SceneSwitch::pop(1);
        }

        if messages.contains(&mooeye::UiMessage::Clicked(2)) {
            res = mooeye::scene_manager::SceneSwitch::replace(
                super::main_menu::MainMenu::new(ctx)?,
                2,
            );
        }

        if messages.contains(&mooeye::UiMessage::Clicked(3)) {
            res = mooeye::scene_manager::SceneSwitch::pop(2);
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
