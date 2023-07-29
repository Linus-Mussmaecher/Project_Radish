use crate::{music, options};
use ggez::{glam::Vec2, graphics, GameError};
use legion::{component, systems::CommandBuffer, Entity, IntoQuery, Resources, Schedule, World};
use mooeye::ui as mui;
use mooeye::{scene_manager, sprite};

use std::time::Duration;

mod game_message;
pub use game_message::GameMessage;
pub use game_message::MessageReceiver;
pub use game_message::MessageSet;

mod game_data;

mod director;
pub use director::EnemyTemplate;

mod components;

mod controller;
pub use controller::Controller;
pub use controller::Interactions;

pub mod achievements;
pub use achievements::Achievement;

pub mod tutorial;

mod game_config;
pub use game_config::GameConfig;

mod ui;

pub const BOUNDARIES: graphics::Rect = graphics::Rect::new(0., 0., 600., 900.);

/// The main struct representing the current game state.
/// This is the core scene rendering & updating gameplay.
pub struct GameState {
    /// The ECS worlds, containing all acting entities.
    world: World,
    /// The ECS resources, data that must be available to all systems and is not bound to certain entities.
    resources: Resources,
    /// The controller from which player interaction can be read.
    controller: Controller,
    /// The main gameplay schedule, producing and consuming actions
    action_prod_schedule: Schedule,
    /// The in-game GUI.
    gui: mui::UiElement<GameMessage>,
    /// The player for the background music
    music_player: music::MusicPlayer,
    /// The achievement set listening to achievement fulfils
    achievements: achievements::AchievementSet,
    /// The tutorial manager that shows tutorial messages when appropriate
    tutorial: tutorial::TutorialManager,
    /// The offset of the initial camera during the fly-in
    camera_offset: (f32, f32),
}

impl GameState {
    /// Creates a new game state.
    pub fn new(ctx: &ggez::Context, config: GameConfig) -> Result<Self, GameError> {
        // --- WORLD CREATION ---

        // Create world
        let mut world = World::default();

        // --- RESOURCE INITIALIZATION ---

        let options = options::OptionsConfig::from_path("./data/options.toml").unwrap_or_default();
        let tutorial = if options.tutorial {
            tutorial::TutorialManager::new()
        } else {
            tutorial::TutorialManager::new_empty()
        };
        if options.tutorial {
            let new_options = options::OptionsConfig {
                tutorial: false,
                ..options
            };
            new_options
                .save_to_file("./data/options.toml")
                .expect("[ERROR] Could not save updated options.");
        }

        let achievement_set =
            achievements::AchievementSet::load(ctx, config.achievements_unlocked.clone());
        let sprite_pool = sprite::SpritePool::new().with_folder(ctx, "/sprites", true);
        let audio_pool =
            components::audio::AudioPool::new(options).with_folder(ctx, "/audio", true);

        let boundaries = BOUNDARIES;
        let spell_pool = components::spell::init_spell_pool(&sprite_pool, &achievement_set);
        let game_data = game_data::GameData::new(config.starting_gold, config.starting_city_health);
        let director = director::Director::new(&sprite_pool, &config);

        let mut music_player = music::MusicPlayer::from_folder(ctx, "/audio/music/in_game");
        music_player.poll_options();
        music_player.next_song(ctx);

        Self::initalize_environment(&boundaries, &sprite_pool, &mut world)?;

        // Add player

        let player = world.push((
            components::Position::new(boundaries.w / 2., boundaries.h - 64.),
            components::BoundaryCollision::new(true, false, false),
            components::Control::new(config.base_speed),
            components::Graphics::from(
                sprite_pool.init_sprite("/sprites/mage2", Duration::from_secs_f32(0.25))?,
            ),
            components::SpellCaster::new(
                components::spell::init_base_spells(&spell_pool, &sprite_pool, &config.base_spells),
                config.base_slots,
            ),
        ));

        let mut message_set = MessageSet::new();
        // insert this to make sure the city health is displayed correctly
        message_set.insert(mui::UiMessage::Extern(GameMessage::UpdateCityHealth(
            game_data.city_health,
        )));

        // --- RESOURCE INSERTION ---

        let mut resources = Resources::default();
        resources.insert(player);
        resources.insert(game_data);
        resources.insert(message_set);
        resources.insert(boundaries);
        resources.insert(director);
        resources.insert(spell_pool);
        resources.insert(sprite_pool);
        resources.insert(audio_pool);

        // --- UI CREATION ---

        let mut gui = ui::game_ui::construct_game_ui(ctx, config.clone())?;
        ui::wave_menu::sync_ui(ctx, &mut gui, &mut world, &mut resources);

        // --- SYSTEM REGISTRY / UI CONSTRUCTION / CONTROLLER INITIALIZATION ---
        Ok(Self {
            world,
            camera_offset: (config.initial_camera_offset, config.initial_camera_offset),
            gui,
            music_player,
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
                .flush()
                // systems that consume (but may produce) actions
                .add_system(components::spell::spell_casting_system())
                .add_system(components::actions::handle_effects_system())
                .flush()
                // buildings
                .add_system(components::buildings::destroy_buildings_system())
                .add_system(components::buildings::create_buildings_system())
                // systems that consume actions
                .add_system(components::actions::resolve_executive_actions_system())
                .add_system(components::graphics::handle_particles_system())
                .add_system(components::audio::audio_enqueue_system())
                .add_system(components::position::resolve_move_system())
                .add_system(components::collision::boundary_collision_system())
                .add_system(components::collision::resolve_immunities_system())
                .add_system(components::health::resolve_damage_system())
                .add_system(components::actions::apply_silence_system())
                .add_system(game_data::resolve_gama_data_system())
                .add_system(components::health::enemy_death_sprite_system())
                .add_system(components::health::remove_entities_system())
                .add_system(components::actions::clear_system())
                .build(),
            achievements: achievement_set,
            tutorial,
            resources,
            controller: Controller::from_path("./data/keymap.toml").unwrap_or_default(),
        })
    }

    /// Initializes the environment by spawning house and brush sprites.
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
                    let mut cobble =
                        sprite_pool.init_sprite("/sprites/environment/cobble", Duration::ZERO)?;
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
                .expect("[ERROR] Ordering of y-coordinates in brush init failed.")
        });
        for pos in positions {
            world.push((
                pos,
                components::Graphics::from({
                    let mut tree =
                        sprite_pool.init_sprite("/sprites/environment/tree", Duration::ZERO)?;
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
                    let mut build =
                        sprite_pool.init_sprite("/sprites/environment/building", Duration::ZERO)?;
                    build.set_variant(rand::random::<u32>());
                    build
                }),
            ));
        }
        Ok(())
    }

    /// A helper function that draw the background street.
    pub fn draw_background(
        boundaries: &graphics::Rect,
        ctx: &ggez::Context,
        canvas: &mut ggez::graphics::Canvas,
    ) {
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
        // +-------------------------------------------------------+
        // |                     Preparation                       |
        // +-------------------------------------------------------+

        // create interaction struct and insert as resource
        self.resources.insert(self.controller.get_interactions(ctx));

        // make sure all entities have all default components
        self.ensure_default_components();

        // +-------------------------------------------------------+
        // |                   Action Handling                     |
        // +-------------------------------------------------------+

        // if a tutorial message is being displayed and we are not in the wave menu, pause the game
        let act = if let Some(director) = self.resources.get_mut::<director::Director>() {
            !self.tutorial.is_active() || director.is_between_waves()
        } else {
            false
        };
        if act {
            self.action_prod_schedule
                .execute(&mut self.world, &mut self.resources);
        }

        // +-------------------------------------------------------+
        // |                  Message Handling                     |
        // +-------------------------------------------------------+

        let mut switch = scene_manager::SceneSwitch::None;

        // acquire messages
        let total_messages = if let Some(mut message_set) = self.resources.get_mut::<MessageSet>() {
            // if message set can be retrieved, drain it
            self.gui
                .update(ctx, message_set.clone())
                .union(
                    &message_set
                        .drain()
                        .collect::<std::collections::HashSet<mui::UiMessage<GameMessage>>>(),
                )
                .copied()
                .collect()
        } else {
            self.gui.update(ctx, None)
        };

        // handle wave menu
        ui::wave_menu::handle_wave_menu(
            &total_messages,
            &mut self.gui,
            ctx,
            &mut self.world,
            &mut self.resources,
        );

        // handle listeners
        for message in total_messages.iter() {
            self.achievements.receive(message, &mut self.gui, ctx);
            self.tutorial.receive(message, &mut self.gui, ctx);
        }

        // Escape menu
        if total_messages.contains(&mui::UiMessage::Triggered(1)) {
            self.achievements.save();
            switch = scene_manager::SceneSwitch::push(ui::in_game_menu::InGameMenu::new(ctx)?);
        }

        if total_messages.contains(&mui::UiMessage::Triggered(tutorial::TUTORIAL_CLOSE)) {
            self.gui.remove_elements(tutorial::TUTORIAL_INNER);
        }

        // +-------------------------------------------------------+
        // |                   Game Over Check                     |
        // +-------------------------------------------------------+

        if let Some(game_data) = self.resources.get::<game_data::GameData>() {
            if game_data.city_health <= 0 {
                // stop music player
                self.music_player.stop(ctx);
                // save achieved wave to quick start file
                let mut cfg = game_config::GameConfig::from_path("/data/quick_config.toml")
                    .unwrap_or_default();
                if let Some(director) = self.resources.get::<director::Director>() {
                    if cfg.starting_wave < director.get_wave() / 2 {
                        cfg.starting_wave = director.get_wave() / 2;
                        cfg.starting_gold = game_data.get_score() / 3;
                    }
                }
                if cfg.save_to_file("/data/quick_config.toml").is_err() {
                    println!("[ERROR/Radish] Could not save quick start config.");
                };
                // create the game over menu, replacing any other attempted scene switch
                switch = scene_manager::SceneSwitch::push(
                    crate::scenes::game_over_menu::GameOverMenu::new(ctx, game_data.get_score())?,
                );
            }
        }

        Ok(switch)
    }

    fn draw(&mut self, ctx: &mut ggez::Context, mouse_listen: bool) -> Result<(), GameError> {
        // Manage music
        self.music_player.check_song(ctx);

        // Get canvas & set sampler
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from_rgb_u32(crate::PALETTE[11]));
        canvas.set_sampler(graphics::Sampler::nearest_clamp());
        let (screen_w, screen_h) = ctx.gfx.drawable_size();

        // Draw background
        Self::draw_background(
            &self
                .resources
                .get::<graphics::Rect>()
                .map(|r| *r)
                .unwrap_or_default(),
            ctx,
            &mut canvas,
        );

        // Draw world

        components::graphics::draw_sprites(
            &mut self.world,
            &mut self.resources,
            ctx,
            &mut canvas,
            mouse_listen && !self.tutorial.is_active(),
            &mut self.camera_offset,
        )?;

        // Draw GUI
        self.gui.draw_to_screen(ctx, &mut canvas, mouse_listen);

        // draw occlusion
        if self.camera_offset.0 > 0. {
            let ratio = self.camera_offset.0 / self.camera_offset.1;
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .color(graphics::Color::new(0., 0., 0., ratio))
                    .scale(Vec2::new(screen_w, screen_h)),
            );
        }

        // Finish
        canvas.finish(ctx)?;

        // Sounds

        components::audio::audio_play_system(ctx, &mut self.resources)?;

        Ok(())
    }
}
