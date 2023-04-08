use std::collections::HashSet;

use ggez::graphics::{self, Color, TextFragment};
use mooeye::{scene_manager::Scene, ui_element::Alignment, UiContent, UiElement};

use crate::PALETTE;

pub struct MainMenu {
    gui: UiElement<()>,
}

impl MainMenu {
    pub fn new(ctx: &ggez::Context) -> Self {
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
            TextFragment::new("PowerDefense").color(Color::from_rgb_u32(PALETTE[14])),
        )
        .set_font("Retro")
        .set_scale(48.)
        .to_owned()
        .to_element(1, ctx);

        // play
        let play = ggez::graphics::Text::new(
            TextFragment::new("Play").color(Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_builder(1, ctx)
        .with_visuals(box_vis)
        .with_hover_visuals(box_hover_vis)
        .build();

        let tutorial = ggez::graphics::Text::new(
            TextFragment::new("Tutorial").color(Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_builder(2, ctx)
        .with_visuals(box_vis)
        .with_hover_visuals(box_hover_vis)
        .build();

        // achievement

        let achievements = ggez::graphics::Text::new(
            TextFragment::new("Achievements").color(Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_builder(3, ctx)
        .with_visuals(box_vis)
        .with_hover_visuals(box_hover_vis)
        .build();

        let options = ggez::graphics::Text::new(
            TextFragment::new("Options").color(Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_builder(4, ctx)
        .with_visuals(box_vis)
        .with_hover_visuals(box_hover_vis)
        .build();

        let credits = ggez::graphics::Text::new(
            TextFragment::new("Credits").color(Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_builder(5, ctx)
        .with_visuals(box_vis)
        .with_hover_visuals(box_hover_vis)
        .build();

        let quit = ggez::graphics::Text::new(
            TextFragment::new("Quit").color(Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_builder(6, ctx)
        .with_visuals(box_vis)
        .with_hover_visuals(box_hover_vis)
        .build();

        // Container

        let mut menu_box = mooeye::containers::VerticalBox::new();
        menu_box.add(play);
        menu_box.add(tutorial);
        menu_box.add(achievements);
        menu_box.add(options);
        menu_box.add(credits);
        menu_box.add(quit);
        menu_box.spacing = 25.;
        let menu_box = menu_box
            .to_element_builder(0, ctx)
            .with_visuals(box_vis)
            .with_alignment(Alignment::CENTER, Alignment::MIN)
            .with_padding((25., 25., 25., 25.))
            .build();

        let mut big_box = mooeye::containers::VerticalBox::new();
        big_box.add(title);
        big_box.add(menu_box);
        let big_box = big_box.to_element_builder(0, ctx)
        .with_visuals(box_vis)
        .with_alignment(Alignment::MAX, Alignment::MIN)
        .with_padding((25., 25., 25., 25.))
        .build();

        Self { gui: big_box }
    }
}

impl Scene for MainMenu {
    fn update(
        &mut self,
        ctx: &mut ggez::Context,
    ) -> Result<mooeye::scene_manager::SceneSwitch, ggez::GameError> {
        let messages = self.gui.manage_messages(ctx, &HashSet::new());

        let mut res = mooeye::scene_manager::SceneSwitch::None;

        if messages.contains(&mooeye::UiMessage::Clicked(3)) {
            res = mooeye::scene_manager::SceneSwitch::Push(Box::new(
                super::achievement_menu::AchievementMenu::new(ctx),
            ));
        }

        if messages.contains(&mooeye::UiMessage::Clicked(4)) {
            res = mooeye::scene_manager::SceneSwitch::Push(Box::new(
                super::options_menu::OptionsMenu::new(ctx),
            ));
        }

        if messages.contains(&mooeye::UiMessage::Clicked(5)) {
            res = mooeye::scene_manager::SceneSwitch::Push(Box::new(
                super::credits_menu::CreditsMenu::new(ctx),
            ));
        }

        if messages.contains(&mooeye::UiMessage::Clicked(6)) {
            res = mooeye::scene_manager::SceneSwitch::Pop(1);
        }

        Ok(res)
    }

    fn draw(&mut self, ctx: &mut ggez::Context, mouse_listen: bool) -> Result<(), ggez::GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from_rgb_u32(PALETTE[5]));
        canvas.set_sampler(graphics::Sampler::nearest_clamp());

        self.gui.draw_to_screen(ctx, &mut canvas, mouse_listen);

        canvas.finish(ctx)?;
        Ok(())
    }
}
