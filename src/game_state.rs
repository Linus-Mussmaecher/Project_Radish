use std::time::Duration;

use ggez::{graphics::Color, *};
use legion::*;
use mooeye::{scene_manager::Scene, *};

use crate::PALETTE;

mod game_action;
use game_action::ActionQueue;
//use game_action::GameAction;

mod game_message;
use game_message::GameMessage;
use game_message::MessageSet;

mod game_data;
use game_data::GameData;

mod components;

pub struct GameState {
    world: World,

    resources: Resources,

    action_prod_schedule: Schedule,

    action_cons_schedule: Schedule,

    gui: UiElement<GameMessage>,
}

impl GameState {
    pub fn new(ctx: &Context) -> Result<Self, GameError> {
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
            .with_offset(-10., -10.)
            .as_shrink()
            .build();

        main_box.add(cog_icon)?;

        let main_box = main_box.to_element_builder(0, ctx).as_fill().build();

        let mut world = World::default();

        world.push((
            components::Position::new(208., 50.),
            components::Velocity::new(0., 2.),
            components::Control::new(2.),
            sprite::Sprite::from_path_fmt(
                "/sprites/skeleton_basic_16_16.png",
                ctx,
                Duration::from_secs_f32(0.25),
            )?,
            components::Collision::new_basic(16., 16.),
            components::Enemy::new(1, 10),
        ));

        world.push((
            components::Position::new(208., 200.),
            components::Velocity::new(-1., 0.),
            sprite::Sprite::from_path_fmt(
                "/sprites/skeleton_basic_16_16.png",
                ctx,
                Duration::from_secs_f32(0.25),
            )?,
            components::Collision::new_basic(16., 16.),
            components::Duration::new(3, 0),
        ));

        let mut resources = Resources::default();
        resources.insert(ActionQueue::new());
        resources.insert(MessageSet::new());
        resources.insert(GameData::default());
        resources.insert(ctx.time.delta());

        Ok(Self {
            world,
            gui: main_box,
            action_prod_schedule: Schedule::builder()
                .add_system(components::collision::collide_system())
                .add_system(components::position::update_position_system())
                .add_system(components::enemy::manage_enemies_system())
                .build(),
            action_cons_schedule: Schedule::builder()
                .add_system(components::position::position_apply_system())
                .add_system(components::health::remove_dead_system())
                .add_system(components::duration::manage_durations_system())
                .add_system(components::health::take_damage_system())
                .add_system(game_data::handle_game_data_actions_system())
                .build(),
            resources,
        })
    }
}

impl Scene for GameState {
    fn update(&mut self, ctx: &mut Context) -> Result<scene_manager::SceneSwitch, GameError> {
        // lots of systems here

        self.resources.insert(ctx.time.delta());
        {
            let mut action_queue =
                self.resources
                    .get_mut::<ActionQueue>()
                    .ok_or_else(||GameError::CustomError(
                        "Could not unpack action queue.".to_owned(),
                    ))?;

            components::control::control_csystem(&mut self.world, ctx, &mut action_queue);
        }

        // produce game actions of this frame

        self.action_prod_schedule
            .execute(&mut self.world, &mut self.resources);

        // transform game actions of this frame

        // consume game actions of this frame

        self.action_cons_schedule
            .execute(&mut self.world, &mut self.resources);

        // clear game actions

        let mut action_queue =
            self.resources
                .get_mut::<ActionQueue>()
                .ok_or_else(|| GameError::CustomError(
                    "Could not unpack action queue.".to_owned(),
                ))?;
        action_queue.clear();

        // in-game menu

        let messages = self.gui.manage_messages(ctx, None);

        // react to messages

        if messages.contains(&UiMessage::Clicked(1))
            || ctx
                .keyboard
                .is_key_just_pressed(winit::event::VirtualKeyCode::F10)
        {
            return Ok(scene_manager::SceneSwitch::push(
                crate::scenes::in_game_menu::InGameMenu::new(ctx)?,
            ));
        }

        Ok(scene_manager::SceneSwitch::none())
    }

    fn draw(&mut self, ctx: &mut Context, mouse_listen: bool) -> Result<(), GameError> {
        // Get canvas & set sampler
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from_rgb_u32(crate::PALETTE[5]));
        canvas.set_sampler(graphics::Sampler::nearest_clamp());

        // Draw game

        components::sprite::draw_sprites(&mut self.world, ctx, &mut canvas);

        // Draw GUI
        self.gui.draw_to_screen(ctx, &mut canvas, mouse_listen);

        // Finish
        canvas.finish(ctx)?;
        Ok(())
    }
}
