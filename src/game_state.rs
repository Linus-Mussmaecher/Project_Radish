use ggez::graphics::Rect;
use ggez::{graphics::Color, *};
use legion::*;
use mooeye::{scene_manager::Scene, *};
use std::time::Duration;

use crate::PALETTE;

mod game_action;
use game_action::ActionQueue;
//use game_action::GameAction;

mod game_message;
use game_message::GameMessage;
use game_message::MessageSet;

mod game_data;
use game_data::GameData;
mod director;
pub use self::controller::Controller;
use self::director::Director;

mod components;

mod controller;

pub struct GameState {
    world: World,

    resources: Resources,

    controller: Controller,

    action_prod_schedule: Schedule,

    action_cons_schedule: Schedule,

    gui: UiElement<GameMessage>,

    director: Director,
}

impl GameState {
    pub fn new(ctx: &Context) -> Result<Self, GameError> {
        let box_vis = mooeye::ui_element::Visuals {
            background: Color::from_rgb_u32(PALETTE[0]),
            border: Color::from_rgb_u32(PALETTE[7]),
            border_width: 3.,
            rounded_corners: 6.,
        };

        // main box
        let mut main_box = containers::StackBox::new();

        // options icon
        let cog_icon = graphics::Image::from_path(ctx, "/sprites/cog.png")?
            .to_element_builder(1, ctx)
            .with_visuals(box_vis)
            .with_alignment(ui_element::Alignment::Max, ui_element::Alignment::Max)
            .scaled(2., 2.)
            .with_offset(-10., -10.)
            .as_shrink()
            .build();

        main_box.add(cog_icon)?;

        // gold display
        let gold_icon = sprite::Sprite::from_path_fmt(
            "/sprites/coin_16_16.png",
            ctx,
            Duration::from_secs_f32(0.25),
        )?
        .to_element_builder(0, ctx)
        .scaled(2., 2.)
        .build();

        let gold_text = graphics::Text::new(
            graphics::TextFragment::new("0000").color(Color::from_rgb_u32(PALETTE[6])),
        )
        .set_scale(32.)
        .set_font("Retro")
        .to_owned()
        .to_element_builder(0, ctx)
        .with_message_handler(|message_set, _, transitions| {
            for message in message_set {
                if let ui_element::UiMessage::Extern(GameMessage::UpdateGold(new_gold)) = message {
                    transitions.push_back(
                        ui_element::Transition::new(Duration::ZERO).with_new_content(
                            graphics::Text::new(
                                graphics::TextFragment::new(format!("{:04}", *new_gold))
                                    .color(Color::from_rgb_u32(PALETTE[6])),
                            )
                            .set_scale(32.)
                            .set_font("Retro")
                            .to_owned(),
                        ),
                    );
                }
            }
        })
        .build();

        let mut gold_box = containers::HorizontalBox::new();
        gold_box.add(gold_icon)?;
        gold_box.add(gold_text)?;
        let gold_box = gold_box
            .to_element_builder(0, ctx)
            .with_visuals(box_vis)
            .with_alignment(ui_element::Alignment::Min, ui_element::Alignment::Min)
            .with_offset(10., 10.)
            .with_tooltip(
                graphics::Text::new(
                    graphics::TextFragment::new("Your current amount of gold.")
                        .color(Color::from_rgb_u32(PALETTE[6])),
                )
                .set_scale(24.)
                .set_font("Retro")
                .to_owned()
                .to_element_builder(0, ctx)
                .with_tooltip_layout()
                .with_visuals(box_vis)
                .build(),
            )
            .build();

        main_box.add(gold_box)?;

        // city health display

        let city_display = sprite::Sprite::from_path_fmt(
            "/sprites/city_16_16.png",
            ctx,
            Duration::from_secs_f32(0.25),
        )?
        .to_element_builder(0, ctx)
        .scaled(2., 2.)
        .build();

        let city_text = graphics::Text::new(
            graphics::TextFragment::new("100").color(Color::from_rgb_u32(PALETTE[6])),
        )
        .set_scale(32.)
        .set_font("Retro")
        .to_owned()
        .to_element_builder(0, ctx)
        .with_message_handler(|message_set, _, transitions| {
            for message in message_set {
                if let ui_element::UiMessage::Extern(GameMessage::UpdateCityHealth(new_health)) =
                    message
                {
                    transitions.push_back(
                        ui_element::Transition::new(Duration::ZERO).with_new_content(
                            graphics::Text::new(
                                graphics::TextFragment::new(format!("{:03}", *new_health))
                                    .color(Color::from_rgb_u32(PALETTE[6])),
                            )
                            .set_scale(32.)
                            .set_font("Retro")
                            .to_owned(),
                        ),
                    );
                }
            }
        })
        .build();

        let mut city_box = containers::HorizontalBox::new();
        city_box.add(city_display)?;
        city_box.add(city_text)?;
        let city_box = city_box
            .to_element_builder(0, ctx)
            .with_visuals(box_vis)
            .with_alignment(ui_element::Alignment::Max, ui_element::Alignment::Min)
            .with_offset(-10., 10.)
            .with_tooltip(
                graphics::Text::new(
                    graphics::TextFragment::new("The health your city currently has left.")
                        .color(Color::from_rgb_u32(PALETTE[6])),
                )
                .set_scale(24.)
                .set_font("Retro")
                .to_owned()
                .to_element_builder(0, ctx)
                .with_tooltip_layout()
                .with_visuals(box_vis)
                .build(),
            )
            .build();

        main_box.add(city_box)?;

        let main_box = main_box.to_element_builder(0, ctx).as_fill().build();

        let mut world = World::default();
        let boundaries = Rect::new(0., 0., 600., 800.);

        // init sprite pool

        let sprite_pool = crate::sprite_pool::SpritePool::new().with_folder_rec(ctx, "/sprites");

        // add player

        world.push((
            components::Position::new(boundaries.w / 2., boundaries.h),
            components::BoundaryCollision::new(true, false, false),
            components::Control::new(150.),
            sprite_pool.init_sprite("/sprites/mage_16_16.png", Duration::from_secs_f32(0.25))?,
            components::SpellCaster::new(),
        ));

        let mut resources = Resources::default();
        resources.insert(ActionQueue::new());
        resources.insert(MessageSet::new());
        resources.insert(GameData::default());
        resources.insert(boundaries);
        resources.insert(sprite_pool);

        Ok(Self {
            world,
            gui: main_box,
            action_prod_schedule: Schedule::builder()
                // sytems that produce actions
                .add_system(components::collision::collision_system())
                .add_system(components::position::velocity_system())
                .add_system(components::health::enemy_system())
                .add_system(components::control::control_system())
                .add_system(components::duration::manage_durations_system())
                .add_system(components::health::destroy_by_health_system())
                // systems that transform actions
                .add_system(components::aura::aura_system())
                .build(),
            action_cons_schedule: Schedule::builder()
                // systems that consume actions
                .add_system(components::spell_casting::spell_casting_system())
                .add_system(components::position::resolve_move_system())
                .add_system(components::collision::boundary_collision_system())
                .add_system(components::collision::resolve_immunities_system())
                .add_system(components::health::resolve_damage_system())
                .add_system(game_data::resolve_gama_data_system())
                .add_system(components::health::remove_entities_system())
                .build(),
            resources,
            controller: Controller::from_path("./data/keymap.toml"),
            director: Director::new(),
        })
    }
}

impl Scene for GameState {
    fn update(&mut self, ctx: &mut Context) -> Result<scene_manager::SceneSwitch, GameError> {
        // create interaction struct and insert as resource

        self.resources.insert(self.controller.get_interactions(ctx));

        // produce game actions of this frame

        self.action_prod_schedule
            .execute(&mut self.world, &mut self.resources);

        // transform game actions of this frame

        // consume game actions of this frame

        self.action_cons_schedule
            .execute(&mut self.world, &mut self.resources);

        // clear action queue

        {
            let mut action_queue = self.resources.get_mut::<ActionQueue>().ok_or_else(|| {
                GameError::CustomError("Could not unpack action queue.".to_owned())
            })?;
            action_queue.clear();
        }

        // director

        self.director
            .progress(ctx, &mut self.world, &mut self.resources)?;

        // message handling

        {
            // unpack game messages
            let mut message_set = self.resources.get_mut::<MessageSet>().ok_or_else(|| {
                GameError::CustomError("Could not unpack message set.".to_owned())
            })?;

            // communicate with UI
            let messages = self.gui.manage_messages(ctx, message_set.clone());

            // clear game messages
            message_set.clear();

            // react to UI messages
            if messages.contains(&UiMessage::Clicked(1))
                || ctx
                    .keyboard
                    .is_key_just_pressed(winit::event::VirtualKeyCode::F10)
            {
                return Ok(scene_manager::SceneSwitch::push(
                    crate::scenes::in_game_menu::InGameMenu::new(ctx)?,
                ));
            }
        }

        Ok(scene_manager::SceneSwitch::none())
    }

    fn draw(&mut self, ctx: &mut Context, mouse_listen: bool) -> Result<(), GameError> {
        // Get canvas & set sampler
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from_rgb_u32(crate::PALETTE[5]));
        canvas.set_sampler(graphics::Sampler::nearest_clamp());

        // Draw world

        components::sprite::draw_sprites(&mut self.world, &mut self.resources, ctx, &mut canvas)?;

        // Draw GUI
        self.gui.draw_to_screen(ctx, &mut canvas, mouse_listen);

        // Finish
        canvas.finish(ctx)?;
        Ok(())
    }
}
