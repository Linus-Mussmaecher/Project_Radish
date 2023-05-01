use ggez::{
    graphics::{self, Color, TextFragment},
    GameError,
};
use mooeye::{*};

use crate::PALETTE;

pub struct InGameMenu {
    gui: UiElement<()>,
}

impl InGameMenu {
    pub fn new(ctx: &ggez::Context) -> Result<Self, GameError> {
        let box_vis = ui_element::Visuals {
            background: Color::from_rgb_u32(PALETTE[0]),
            border: Color::from_rgb_u32(PALETTE[7]),
            border_width: 3.,
            rounded_corners: 6.,
        };

        let box_hover_vis = ui_element::Visuals {
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

        let mut menu_box: containers::VerticalBox<()> = containers::VerticalBox::new();
        menu_box.add(pause)?;
        menu_box.add(resume)?;
        menu_box.add(main_menu)?;
        menu_box.add(quit)?;
        menu_box.spacing = 25.;
        let menu_box = menu_box
            .to_element_builder(0, ctx)
            .with_visuals(box_vis)
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
            res = scene_manager::SceneSwitch::replace(
                super::main_menu::MainMenu::new(ctx)?,
                2,
            );
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


pub struct GameOverMenu {
    ui: UiElement<()>,
}

impl GameOverMenu {
    pub fn new(score: i32, ctx: &ggez::Context) -> Result<Self, GameError> {
        let box_vis = ui_element::Visuals {
            background: Color::from_rgb_u32(PALETTE[0]),
            border: Color::from_rgb_u32(PALETTE[7]),
            border_width: 3.,
            rounded_corners: 6.,
        };
        let box_hover_vis = ui_element::Visuals {
            background: Color::from_rgb_u32(PALETTE[1]),
            border: Color::from_rgb_u32(PALETTE[7]),
            border_width: 3.,
            rounded_corners: 6.,
        };

        let mut main_box = containers::VerticalBox::new();
        main_box.spacing = 25.;

        let game_over = ggez::graphics::Text::new(
            graphics::TextFragment::new("Game Over!").color(Color::from_rgb_u32(PALETTE[8])),
        )
        .set_font("Retro")
        .set_scale(54.)
        .to_owned()
        .to_element_builder(0, ctx)
        .build();
        main_box.add(game_over)?;

        let score = ggez::graphics::Text::new(
            graphics::TextFragment::new(format!("Score: {}", score)).color(Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_builder(0, ctx)
        .build();
        main_box.add(score)?;

        let restart = ggez::graphics::Text::new(
            graphics::TextFragment::new("Restart").color(Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_builder(1, ctx)
        .with_visuals(box_vis)
        .with_hover_visuals(box_hover_vis)
        .build();
        main_box.add(restart)?;

        let main_menu = ggez::graphics::Text::new(
            graphics::TextFragment::new("Return to Main Menu").color(Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_builder(2, ctx)
        .with_visuals(box_vis)
        .with_hover_visuals(box_hover_vis)
        .build();
        main_box.add(main_menu)?;

        let main_box = main_box.to_element_builder(0, ctx)
        .with_visuals(box_vis)
        .with_alignment(ui_element::Alignment::Center, ui_element::Alignment::Center)
        .with_padding((25., 25., 25., 25.))
        .build();

        Ok(Self {
            ui: main_box,
        })
    }
}

impl scene_manager::Scene for GameOverMenu {
    fn update(&mut self, ctx: &mut ggez::Context) -> Result<scene_manager::SceneSwitch, GameError> {
        let messages = self.ui.manage_messages(ctx, None);

        if messages.contains(&mooeye::UiMessage::Clicked(1))
            || ctx
                .keyboard
                .is_key_just_pressed(ggez::winit::event::VirtualKeyCode::R)
        {
            return Ok(mooeye::scene_manager::SceneSwitch::replace(
                crate::game_state::GameState::new(ctx)?,
                2,
            ));
        }

        if messages.contains(&mooeye::UiMessage::Clicked(2))
            || ctx
                .keyboard
                .is_key_just_pressed(ggez::winit::event::VirtualKeyCode::M)
        {
            return Ok(mooeye::scene_manager::SceneSwitch::replace(super::main_menu::MainMenu::new(ctx)? , 2));
        }
        Ok(mooeye::scene_manager::SceneSwitch::None)
    }

    fn draw(&mut self, ctx: &mut ggez::Context, mouse_listen: bool) -> Result<(), GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, None);
        canvas.set_sampler(graphics::Sampler::nearest_clamp());

        self.ui.draw_to_screen(ctx, &mut canvas, mouse_listen);

        canvas.finish(ctx)?;
        Ok(())
    }
}