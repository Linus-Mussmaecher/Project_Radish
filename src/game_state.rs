use ggez::graphics::Rect;
use ggez::*;
use legion::*;
use mooeye::{scene_manager::Scene, *};
use std::time::Duration;

mod game_message;
use game_message::GameMessage;
use game_message::MessageSet;

mod game_data;
use game_data::GameData;
mod director;
use self::components::spell::spell_list;
pub use self::controller::Controller;
use self::director::Director;

mod game_ui;

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
        // Create world

        let mut world = World::default();
        let boundaries = Rect::new(0., 0., 600., 800.);

        // init sprite pool

        let sprite_pool = sprite::SpritePool::new().with_folder(ctx, "/sprites", true);

        // add grass sprites

        for _i in 0..16 {
            world.push((
                components::Position::new(
                    boundaries.w * rand::random::<f32>(),
                    boundaries.h * rand::random::<f32>(),
                ),
                components::Graphics::from({
                    let mut grass =
                        sprite_pool.init_sprite("/sprites/brush", Duration::from_secs(1))?;
                    grass.set_variant(rand::random::<u32>());
                    grass
                }),
            ));
        }

        // add buildings

        let building_size = 4. * 32.;
        let building_spacing = 4. * 8.;

        for x in [
            -1.5 * (building_size + building_spacing),
            -0.5 * (building_size + building_spacing),
            boundaries.w + 1.5 * (building_size + building_spacing),
            boundaries.w + 0.5 * (building_size + building_spacing),
        ] {
            for y in 0..((boundaries.h / (building_size + building_spacing)) as usize) {
                world.push((
                    components::Position::new(
                        x + building_spacing * (rand::random::<f32>() - 0.5),
                        building_size / 2.
                            + y as f32 * (building_size + building_spacing)
                            + building_spacing * (rand::random::<f32>() - 0.5),
                    ),
                    components::Graphics::from({
                        let mut grass =
                            sprite_pool.init_sprite("/sprites/building", Duration::from_secs(1))?;
                        grass.set_variant(rand::random::<u32>());
                        grass
                    }),
                ));
            }
        }

        // add player

        world.push((
            components::Position::new(boundaries.w / 2., boundaries.h),
            components::BoundaryCollision::new(true, false, false),
            components::Control::new(150.),
            components::Graphics::from(
                sprite_pool.init_sprite("/sprites/mage", Duration::from_secs_f32(0.25))?,
            ),
            components::SpellCaster::new(vec![
                spell_list::construct_fireball(&sprite_pool),
                spell_list::construct_icebomb(&sprite_pool),
                spell_list::construct_electrobomb(&sprite_pool),
                spell_list::construct_conflagrate(&sprite_pool),
            ]),
        ));

        let mut resources = Resources::default();
        resources.insert(MessageSet::new());
        resources.insert(GameData::default());
        resources.insert(boundaries);
        resources.insert(sprite_pool);

        Ok(Self {
            world,
            gui: game_ui::construct_game_ui(ctx)?,
            action_prod_schedule: Schedule::builder()
                // sytems that produce actions
                .add_system(components::collision::collision_system())
                .add_system(components::position::velocity_system())
                .add_system(components::health::enemy_system())
                .add_system(components::control::control_system())
                .add_system(components::duration::manage_durations_system())
                .add_system(components::health::destroy_by_health_system())
                .add_system(components::actions::handle_repeaters_system())
                // systems that consume (but may produce) actions
                .add_system(components::spell::spell_casting_system())
                .build(),
            action_cons_schedule: Schedule::builder()
                // systems that consume actions
                .add_system(components::actions::resolve_executive_actions_system())
                .add_system(components::actions::register_repeaters_system())
                .add_system(components::graphics::handle_particles_system())
                .add_system(components::position::resolve_move_system())
                .add_system(components::collision::boundary_collision_system())
                .add_system(components::collision::resolve_immunities_system())
                .add_system(components::health::resolve_damage_system())
                .add_system(game_data::resolve_gama_data_system())
                .add_system(components::health::enemy_death_sprite_system())
                .add_system(components::health::remove_entities_system())
                .add_system(components::actions::clear_system())
                .build(),
            resources,
            controller: Controller::from_path("./data/keymap.toml").unwrap_or_default(),
            director: Director::new(),
        })
    }
}

impl Scene for GameState {
    fn update(&mut self, ctx: &mut Context) -> Result<scene_manager::SceneSwitch, GameError> {
        // create interaction struct and insert as resource

        self.resources.insert(self.controller.get_interactions(ctx));

        // make sure all entities have all default components

        ensure_default_components(&mut self.world);

        // produce game actions of this frame

        self.action_prod_schedule
            .execute(&mut self.world, &mut self.resources);

        // transform game actions of this frame
        components::actions::distribution_system(&mut self.world);
        components::aura::aura_system(&mut self.world);

        // consume game actions of this frame

        self.action_cons_schedule
            .execute(&mut self.world, &mut self.resources);

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

        components::graphics::draw_sprites(
            &mut self.world,
            &mut self.resources,
            ctx,
            &mut canvas,
            mouse_listen,
        )?;

        // Draw GUI
        self.gui.draw_to_screen(ctx, &mut canvas, mouse_listen);

        // Finish
        canvas.finish(ctx)?;
        Ok(())
    }
}

fn ensure_default_components(world: &mut World) {
    let mut list = Vec::new();

    for ent in <Entity>::query()
        .filter(!component::<components::Actions>())
        .iter(world)
    {
        list.push(*ent);
    }

    for ent in list {
        if let Some(mut entry) = world.entry(ent) {
            entry.add_component(components::Actions::new());
        }
    }
}
