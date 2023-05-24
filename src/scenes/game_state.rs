use ggez::{glam::Vec2, graphics, GameError};
use if_chain::if_chain;
use legion::{
    component, systems::CommandBuffer, Entity, EntityStore, IntoQuery, Resources, Schedule, World,
};
use mooeye::*;

use std::time::Duration;

mod game_message;
pub use game_message::GameMessage;
pub use game_message::MessageSet;

mod game_data;

mod director;
pub use director::EnemyTemplate;

mod components;
use self::components::spell::spell_list;

mod controller;
pub use controller::Controller;
pub use controller::Interactions;

pub mod achievements;
pub use achievements::Achievement;

mod ui;

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
    gui: UiElement<GameMessage>,
    /// Listeners (such as achievements or tutorials), receiving all game messages and potentially mutating the UI.
    listeners: Vec<Box<dyn game_message::MessageReceiver>>,
}

impl GameState {
    /// Creates a new game state.
    pub fn new(ctx: &ggez::Context) -> Result<Self, GameError> {
        // --- WORLD CREATION ---

        // Create world
        let mut world = World::default();

        // Create some resources neccessary for world init
        let boundaries = graphics::Rect::new(0., 0., 600., 900.);
        let sprite_pool = sprite::SpritePool::new().with_folder(ctx, "/sprites", true);

        Self::initalize_environment(&boundaries, &sprite_pool, &mut world)?;

        // Add player

        let player = world.push((
            components::Position::new(boundaries.w / 2., boundaries.h - 64.),
            components::BoundaryCollision::new(true, false, false),
            components::Control::new(150.),
            components::Graphics::from(
                sprite_pool.init_sprite("/sprites/mage", Duration::from_secs_f32(0.25))?,
            ),
            components::SpellCaster::new(
                vec![
                    spell_list::construct_fireball(&sprite_pool),
                    spell_list::construct_icemissile(&sprite_pool),
                    spell_list::construct_electrobomb(&sprite_pool),
                    spell_list::construct_conflagrate(&sprite_pool),
                ],
                4,
            ),
        ));

        // --- RESOURCE INITIALIZATION

        let game_data = game_data::GameData::new(player);
        let mut message_set = MessageSet::new();
        message_set.insert(UiMessage::Extern(GameMessage::UpdateCityHealth(
            game_data.city_health,
        )));

        let mut resources = Resources::default();
        resources.insert(game_data);
        resources.insert(message_set);
        resources.insert(boundaries);
        resources.insert(director::Director::new(&sprite_pool));
        resources.insert(sprite_pool);

        // --- SYSTEM REGISTRY / UI CONSTRUCTION / CONTROLLER INITIALIZATION ---
        Ok(Self {
            world,
            gui: ui::game_ui::construct_game_ui(ctx)?,
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
            listeners: {
                let mut list: Vec<Box<dyn game_message::MessageReceiver>> = Vec::new();
                list.push(Box::new(achievements::AchievementSet::load(ctx)));
                list
            },
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
                .expect("[ERROR] Ordering of y-coordinates in brush init failed.")
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

        // --- MESSAGE HANDLING ---

        let mut switch = scene_manager::SceneSwitch::None;

        // retrieve game messages
        if let Some(mut message_set) = self.resources.get_mut::<MessageSet>() {
            // ======== GAME MESSAGES ========

            // check for next wave condition
            for message in message_set.iter() {
                if let &UiMessage::Extern(GameMessage::NextWave(_)) = message {
                    if let Some(director) = self.resources.get::<director::Director>() {
                        // Wave menu begin
                        self.gui.add_element(
                            0,
                            ui::wave_menu::construct_wave_menu(
                                ctx,
                                director.get_wave() as i32,
                                &director.get_enemies(),
                            ),
                        );
                    }
                }

                for listener in self.listeners.iter_mut() {
                    listener.receive(message, &mut self.gui, ctx);
                }
            }

            // communicate with UI: Insert Game Messages and retrieve UI messages
            let messages = self.gui.update(ctx, message_set.clone());

            // clear game messages
            message_set.clear();

            // ======== UI MESSAGES ========

            // Escape menu
            if messages.contains(&UiMessage::Triggered(1)) {
                switch = scene_manager::SceneSwitch::push(
                    ui::in_game_menu::InGameMenu::new(ctx)?,
                );
            }

            // -------- WAVE MENU --------

            // Exit wave menu
            if messages.contains(&UiMessage::Triggered(201)) {
                self.gui.remove_elements(200);
                if let Some(mut dir) = self.resources.get_mut::<director::Director>() {
                    // initialize next wave from director
                    dir.next_wave();
                    // create wave announcer
                    self.gui
                        .add_element(0, ui::wave_menu::construct_wave_announcer(ctx, dir.get_wave()));
                }
            }

            // Add spell slot
            if_chain! {
                if messages.contains(&UiMessage::Triggered(202));
                if let Some(mut data) = self.resources.get_mut::<self::game_data::GameData>();
                if let Ok(mut player) = self.world.entry_mut(data.get_player());
                if let Ok(sc) = player.get_component_mut::<components::SpellCaster>();
                if sc.can_add() && data.spend(250);
                then {
                    sc.add_slot();
                    self.gui.add_element(
                        50,
                        ui::game_ui::create_spellslot(
                            ctx,
                            sc.get_slots() - 1,
                        ),
                    );
                }
            }

            // reroll enemies
            if_chain! {
                if messages.contains(&UiMessage::Triggered(204));
                if let Some(mut director) = self.resources.get_mut::<director::Director>();
                if let Some(mut data) = self.resources.get_mut::<self::game_data::GameData>();
                if data.spend(50);
                then {
                    // reroll enemies
                    director.reroll_wave_enemies();
                    // recreate UI
                    self.gui.remove_elements(200);
                    self.gui
                    .add_element(0, ui::wave_menu::construct_wave_menu(ctx, director.get_wave() as i32 , &director.get_enemies()));
                }
            }
        }

        // -------- Game Over Check --------

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
