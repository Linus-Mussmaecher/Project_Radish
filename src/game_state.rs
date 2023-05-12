use ggez::{glam::Vec2, graphics, GameError};
use legion::{component, systems::CommandBuffer, Entity, IntoQuery, Resources, Schedule, World};
use mooeye::*;

use std::time::Duration;

mod game_message;
pub use game_message::GameMessage;
pub use game_message::MessageSet;

mod game_data;

mod director;

mod components;
use self::components::spell::spell_list;

mod controller;
pub use controller::Controller;

mod achievements;
pub use achievements::Achievement;

pub struct GameState {
    world: World,

    resources: Resources,

    controller: Controller,

    action_prod_schedule: Schedule,

    action_cons_schedule: Schedule,

    gui: UiElement<GameMessage>,
}

impl GameState {
    pub fn new(ctx: &ggez::Context) -> Result<Self, GameError> {
        // --- WORLD CREATION ---

        // Create world
        let mut world = World::default();

        // Create some resources neccessary for world init
        let boundaries = graphics::Rect::new(0., 0., 600., 800.);
        let sprite_pool = sprite::SpritePool::new().with_folder(ctx, "/sprites", true);

        Self::initalize_environment(&boundaries, &sprite_pool, &mut world)?;

        // Add player

        world.push((
            components::Position::new(boundaries.w / 2., boundaries.h),
            components::BoundaryCollision::new(true, false, false),
            components::Control::new(150.),
            components::Graphics::from(
                sprite_pool.init_sprite("/sprites/mage", Duration::from_secs_f32(0.25))?,
            ),
            components::SpellCaster::new(
                vec![
                    spell_list::construct_fireball(&sprite_pool),
                    spell_list::construct_icebomb(&sprite_pool),
                    spell_list::construct_electrobomb(&sprite_pool),
                    spell_list::construct_conflagrate(&sprite_pool),
                ],
                6,
            ),
        ));

        // --- RESOURCE INITIALIZATION

        let mut resources = Resources::default();
        let game_data = game_data::GameData::default();
        let mut message_set = MessageSet::new();
        message_set.insert(UiMessage::Extern(GameMessage::UpdateCityHealth(
            game_data.city_health,
        )));
        message_set.insert(UiMessage::Extern(GameMessage::NextWave(0)));
        resources.insert(game_data);
        resources.insert(message_set);
        resources.insert(boundaries);
        resources.insert(sprite_pool);
        resources.insert(director::Director::new());

        // --- SYSTEM REGISTRY / UI CONSTRUCTION / CONTROLLER INITIALIZATION ---
        Ok(Self {
            world,
            gui: crate::scenes::game_ui::construct_game_ui(ctx)?,
            action_prod_schedule: Schedule::builder()
                // director
                .add_system(director::direct_system())
                // sytems that produce actions
                .add_system(components::collision::collision_system())
                .add_system(components::position::velocity_system())
                .add_system(components::health::enemy_system())
                .add_system(components::control::control_system())
                .add_system(components::duration::manage_durations_system())
                .add_system(components::health::destroy_by_health_system())
                // systems that consume (but may produce) actions
                .add_system(components::spell::spell_casting_system())
                .build(),
            action_cons_schedule: Schedule::builder()
                // systems that consume actions
                .add_system(components::actions::resolve_executive_actions_system())
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
        })
    }

    fn initalize_environment(
        boundaries: &graphics::Rect,
        sprite_pool: &sprite::SpritePool,
        world: &mut World,
    ) -> Result<(), GameError> {
        // Create cobble sprites
        for _i in 0..48 {
            world.push((
                components::Position::new(
                    boundaries.w * rand::random::<f32>(),
                    boundaries.h * (rand::random::<f32>() * 2. - 0.5),
                ),
                components::Graphics::from({
                    let mut cobble = sprite_pool
                        .init_sprite("/sprites/environment/cobble", Duration::from_secs(1))?;
                    cobble.set_variant(rand::random::<u32>());
                    cobble
                }),
            ));
        }

        let building_size = 4. * 32.;

        // Add tree sprites
        let mut positions = Vec::new();
        for _i in 0..12 {
            let rand_x = rand::random::<f32>() * 8. - 4.;
            positions.push(components::Position::new(
                (rand_x) * building_size + if rand_x > 0. { boundaries.w } else { 0. },
                (rand::random::<f32>() * 0.7 - 0.2) * boundaries.h,
            ));
        }
        positions.sort_by(|p1, p2| {
            p1.y.partial_cmp(&p2.y)
                .expect("Ordering of y-coordinates in brush init failed.")
        });
        for pos in positions {
            world.push((
                pos,
                components::Graphics::from({
                    let mut tree = sprite_pool
                        .init_sprite("/sprites/environment/tree", Duration::from_secs(1))?;
                    tree.set_variant(rand::random::<u32>());
                    tree
                }),
            ));
        }

        // Add building sprites
        for (x, y) in [
            // left side
            (-1.2, -1.8),
            (-0.55, -0.6),
            (-2.6, -0.65),
            (-2.1, 0.5),
            (-0.9, 0.67),
            (-1.6, -0.55),
            // right side
            (0.8, -1.6),
            (2.1, -1.2),
            (0.7, -0.3),
            (2.4, -0.5),
            (0.8, 0.8),
            (2.3, 0.6),
        ] {
            world.push((
                components::Position::new(
                    building_size * x + if x > 0. { boundaries.w } else { 0. },
                    y * building_size + boundaries.h,
                ),
                components::Graphics::from({
                    let mut build = sprite_pool
                        .init_sprite("/sprites/environment/building", Duration::from_secs(1))?;
                    build.set_variant(rand::random::<u32>());
                    build
                }),
            ));
        }
        Ok(())
    }

    /// A helper function that ensures every entity in the world has a certain component subset
    fn ensure_default_components(&mut self) {
        // running buffer of added components
        let mut buffer = CommandBuffer::new(&self.world);

        // query for all elements not equipped with an ACTIONS module
        for ent in <Entity>::query()
            .filter(!component::<components::Actions>())
            .iter(&self.world)
        {
            buffer.add_component(*ent, components::Actions::new());
        }

        buffer.flush(&mut self.world, &mut self.resources);
    }
}

impl scene_manager::Scene for GameState {
    fn update(&mut self, ctx: &mut ggez::Context) -> Result<scene_manager::SceneSwitch, GameError> {
        // --- PREPARATION ---

        // create interaction struct and insert as resource
        self.resources.insert(self.controller.get_interactions(ctx));

        // make sure all entities have all default components
        self.ensure_default_components();

        // --- ACTION HANDLING & SYSTEMS ---

        // produce game actions of this frame
        self.action_prod_schedule
            .execute(&mut self.world, &mut self.resources);

        // transform game actions of this frame
        components::actions::handle_effects(&mut self.world, &mut self.resources);

        // consume game actions of this frame
        self.action_cons_schedule
            .execute(&mut self.world, &mut self.resources);

        // --- MESSAGE HANDLING ---

        let mut switch = scene_manager::SceneSwitch::None;

        // retrieve game messages
        if let Some(mut message_set) = self.resources.get_mut::<MessageSet>() {
            // check for next wave condition

            for message in message_set.iter() {
                if let &UiMessage::Extern(GameMessage::NextWave(wave)) = message {
                    if wave > 0 {
                        switch = scene_manager::SceneSwitch::push(
                            crate::scenes::wave_menu::WaveMenu::new(ctx, wave)?,
                        );
                        self.gui.add_element(
                            100,
                            super::scenes::achievement_menu::achievement_info(Achievement::new(
                                "To Dust",
                                "Kill 50 enemies.",
                                graphics::Image::from_path(ctx, "/sprites/achievements/a3_16_16.png").ok(),
                                50,
                                |msg| matches!(msg, GameMessage::UpdateGold(_)),
                            ), ctx)?,
                        );
                    }
                }
            }

            // communicate with UI
            let messages = self.gui.update(ctx, message_set.clone());

            // clear game messages
            message_set.clear();

            // react to UI messages
            if messages.contains(&UiMessage::Triggered(1)) {
                switch = scene_manager::SceneSwitch::push(
                    crate::scenes::in_game_menu::InGameMenu::new(ctx)?,
                );
            }
        }

        // check for game over condition

        if let Some(game_data) = self.resources.get::<game_data::GameData>() {
            if game_data.city_health <= 0 {
                switch = scene_manager::SceneSwitch::push(
                    crate::scenes::game_over_menu::GameOverMenu::new(ctx, game_data.get_score())?,
                );
            }
        }

        Ok(switch)
    }

    fn draw(&mut self, ctx: &mut ggez::Context, mouse_listen: bool) -> Result<(), GameError> {
        // Get canvas & set sampler
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from_rgb_u32(crate::PALETTE[11]));
        canvas.set_sampler(graphics::Sampler::nearest_clamp());

        // Draw background
        {
            // Draw street
            let boundaries = self
                .resources
                .get::<graphics::Rect>()
                .map(|r| *r)
                .unwrap_or_default();

            let (screen_w, screen_h) = ctx.gfx.drawable_size();
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .color(graphics::Color::from_rgb_u32(crate::PALETTE[10]))
                    .scale(Vec2::new(boundaries.w, screen_h))
                    .dest(Vec2::new((screen_w - boundaries.w) / 2., 0.)),
            );
            // street edges
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .color(graphics::Color::from_rgb_u32(crate::PALETTE[12]))
                    .scale(Vec2::new(8., screen_h))
                    .dest(Vec2::new((screen_w - boundaries.w) / 2. - 4., 0.)),
            );
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .color(graphics::Color::from_rgb_u32(crate::PALETTE[12]))
                    .scale(Vec2::new(8., screen_h))
                    .dest(Vec2::new((screen_w + boundaries.w) / 2. - 4., 0.)),
            );
        }

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
