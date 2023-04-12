use std::collections::VecDeque;

use legion::*;
use mooeye::{*, scene_manager::Scene};
use ggez::{*, graphics::Color};

use crate::PALETTE;

mod game_action;
use game_action::GameAction;


pub struct GameState{
    world: World,

    resources: Resources,

    gui: UiElement<()>,
}


impl GameState {

    pub fn new(ctx: &Context) -> Result<Self, GameError>{
        

        let box_vis = mooeye::ui_element::Visuals {
            background: Color::from_rgb_u32(PALETTE[0]),
            border: Color::from_rgb_u32(PALETTE[7]),
            border_width: 3.,
            rounded_corners: 6.,
        };

        let mut main_box = containers::StackBox::new();

        let cog_icon = graphics::Image::from_path(ctx, "/sprites/cog.png")?
        .to_element_builder(1, ctx)
        .with_visuals(box_vis)
        .with_alignment(ui_element::Alignment::Max, ui_element::Alignment::Max)
        .scaled(2., 2.)
        .as_shrink()
        .build();

        main_box.add(cog_icon)?;

        let main_box = main_box
        .to_element_builder(0, ctx)
        .as_fill()
        .build();

        let mut resources = Resources::default();
        resources.insert(VecDeque::<GameAction>::new());

        Ok(Self { world: World::default(), gui: main_box, resources})
    }
}


impl Scene for GameState{
    fn update(&mut self, ctx: &mut Context) -> Result<scene_manager::SceneSwitch, GameError> {
        // lots of systems here




        // in-game menu

        let messages = self.gui.manage_messages(ctx, None);

        // react to messages

        if messages.contains(&UiMessage::Clicked(1)) || ctx.keyboard.is_key_just_pressed(winit::event::VirtualKeyCode::F10){
            return Ok(scene_manager::SceneSwitch::push(crate::scenes::in_game_menu::InGameMenu::new(ctx)?));
        }

        Ok(scene_manager::SceneSwitch::none())
    }

    fn draw(&mut self, ctx: &mut Context, mouse_listen: bool) -> Result<(), GameError> {
        // Get canvas & set sampler
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::from_rgb_u32(crate::PALETTE[5]));
        canvas.set_sampler(graphics::Sampler::nearest_clamp());

        // Draw game


        // Draw GUI
        self.gui.draw_to_screen(ctx, &mut canvas, mouse_listen);

        // Finish
        canvas.finish(ctx)?;
        Ok(())
    }
}


