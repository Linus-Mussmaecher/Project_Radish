use ggez::{
    graphics::{self, Color, TextFragment},
    GameError,
};
use mooeye::*;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
struct Score {
    score: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ScoreList {
    scores: Vec<Score>,
}

pub fn load_highscores() -> Vec<i32>{
    if let Ok(file) = std::fs::read_to_string("./data/highscores.toml") {
        toml::from_str::<ScoreList>(&file)
            .map(|sl| sl.scores.iter().map(|s| s.score).collect())
            .unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    }
}

pub struct GameOverMenu {
    ui: UiElement<()>,
}

impl GameOverMenu {
    pub fn new(score: i32, ctx: &ggez::Context) -> Result<Self, GameError> {
        // load highscores

        let mut highscores: Vec<Score> =
            if let Ok(file) = std::fs::read_to_string("./data/highscores.toml") {
                toml::from_str::<ScoreList>(&file)
                    .map(|sl| sl.scores)
                    .unwrap_or_else(|_| Vec::new())
            } else {
                Vec::new()
            };

        // if only a small amount of scores is recorded or the worst result from the list was beaten, insert the new result
        let own_index = if highscores.len() < 25
            || score >= highscores.last().map(|a| a.score).unwrap_or_default()
        {
            // set default index: at the end of the list
            let mut index = highscores.len();
            // see if there is an appropriate earlier index
            for (i, value) in highscores.iter().enumerate() {
                if score >= value.score {
                    index = i;
                    break;
                }
            }
            // insert at appropriate index
            highscores.insert(index, Score { score });
            // if list has grown too much, cut last element
            if highscores.len() > 25 {
                highscores.pop();
            }
            // return selected index
            Some(index)
        } else {
            None
        };

        // save highscores
        if let Ok(toml_string) = toml::to_string(&ScoreList {
            scores: highscores.clone(),
        }) {
            std::fs::write("./data/highscores.toml", &toml_string)?;
        }

        // create UI
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

        let mut score_box = containers::HorizontalBox::new();
        score_box.spacing = 35.;

        let score_disp = ggez::graphics::Text::new(
            graphics::TextFragment::new("Your score:\n")
                .color(Color::from_rgb_u32(PALETTE[7]))
                .scale(36.),
        )
        .add(
            graphics::TextFragment::new(format!("  {:>5}", score))
                .color(Color::from_rgb_u32(PALETTE[6]))
                .scale(32.),
        )
        .set_font("Retro")
        .to_owned()
        .to_element_builder(0, ctx)
        .with_alignment(ui_element::Alignment::Center, ui_element::Alignment::Min)
        .build();
        score_box.add(score_disp)?;

        let mut highscore_disp = ggez::graphics::Text::new(
            graphics::TextFragment::new("Highscores:\n")
                .color(Color::from_rgb_u32(PALETTE[7]))
                .scale(36.),
        );

        for (index, value) in highscores.iter().enumerate().take(5) {
            highscore_disp.add(
                graphics::TextFragment::new(format!("  {:02}.{:>5}\n", index + 1, value.score))
                    .color(Color::from_rgb_u32(if index == own_index.unwrap_or(128) {
                        PALETTE[8]
                    } else {
                        PALETTE[6]
                    }))
                    .scale(32.),
            );
        }

        let highscore_disp = highscore_disp
            .set_font("Retro")
            .to_owned()
            .to_element_builder(0, ctx)
            .with_alignment(ui_element::Alignment::Center, ui_element::Alignment::Min)
            .build();

        score_box.add(highscore_disp)?;

        main_box.add(score_box.to_element(0, ctx))?;

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
            graphics::TextFragment::new("Return to Main Menu")
                .color(Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_builder(2, ctx)
        .with_visuals(box_vis)
        .with_hover_visuals(box_hover_vis)
        .build();
        main_box.add(main_menu)?;

        let main_box = main_box
            .to_element_builder(0, ctx)
            .with_visuals(box_vis)
            .with_alignment(ui_element::Alignment::Center, ui_element::Alignment::Center)
            .with_padding((25., 25., 25., 25.))
            .build();

        Ok(Self { ui: main_box })
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
            return Ok(mooeye::scene_manager::SceneSwitch::replace(
                super::main_menu::MainMenu::new(ctx)?,
                2,
            ));
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
