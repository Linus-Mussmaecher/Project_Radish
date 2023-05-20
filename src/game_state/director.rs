use std::time::Duration;

use ggez::{graphics, GameError};
use legion::{system, systems::CommandBuffer};
use rand::random;

use mooeye::sprite;

use super::{
    components::{self, actions::*, graphics::Particle, Enemy, Position},
    controller::Interactions,
    game_message::MessageSet,
    GameMessage,
};

/// The state of a [Director].
/// States should be used only in sequence.
/// One rotation = one wave.
#[derive(Debug, Clone, Copy)]
enum DirectorState {
    /// The director is currently spawning enemies.
    /// The payload is the wave_pool left to spawn until this wave ends.
    Spawning(u32),
    /// The director has emptied its wave pool and is waiting for all spawned enemies to be removed.
    WaitingForDead,
    /// All enemies have despawned and the director has notified the player of the end of the wave.
    /// The director is waiting for the player to init the next wave.
    WaitingForMenu,
}

/// The director struct is responsible for spawning waves of enemies.
/// A director regularly earns credit points and spends them on units from a customizable enemy set until a wave threshhold is reached.
/// Then, the director rerolls the enemy pool and starts a new wave.
#[derive(Clone)]
pub struct Director {
    /// The current wave number.
    wave: u32,
    /// The current state of the director.
    state: DirectorState,
    /// The time since the director last spawned units.
    intervall: Duration,
    /// The entire existence duration of this director.
    total: Duration,
    /// The current amount of credits this director can spend.
    credits: u32,
    /// The enemy posse the director can select spawns from, containing their costs and a spawning function pointer.
    enemies: Vec<(
        u32,
        fn(&mut CommandBuffer, &sprite::SpritePool, Position) -> Result<(), GameError>,
    )>,
}

impl Director {
    /// Spawns a new director with default parameters.
    pub fn new() -> Self {
        Self {
            wave: 1,
            state: DirectorState::Spawning(800),
            intervall: Duration::ZERO,
            total: Duration::ZERO,
            credits: 0,
            enemies: vec![
                (040, spawn_basic_skeleton),
                (050, spawn_stormer),
                (070, spawn_fast_skeleton),
                (120, spawn_splitter),
                (150, spawn_tank_skeleton),
                (150, spawn_wizard_skeleton),
                (200, spawn_charge_skeleton),
                (300, spawn_loot_skeleton),
            ],
        }
    }

    /// Returns the current wave.
    pub fn get_wave(&self) -> u32 {
        self.wave
    }

    /// If currently in the last [DirectorState] of a wave cycle, reset to the first one, increase the wave number
    /// and grant a wave_pool for that next wave.
    pub fn next_wave(&mut self) {
        if matches!(self.state, DirectorState::WaitingForMenu) {
            self.wave += 1;
            self.state = DirectorState::Spawning(200 + 600 * self.wave);
        }
    }
}

/// A system that handles the directors interaction with the game world.
/// This increases the director credits and spends them, handles unit spawning and sends messages to initialize new waves.
#[system]
pub fn direct(
    subworld: &mut legion::world::SubWorld,
    enemy_query: &mut legion::Query<&Enemy>,
    cmd: &mut CommandBuffer,
    #[resource] spritepool: &sprite::SpritePool,
    #[resource] boundaries: &graphics::Rect,
    #[resource] director: &mut Director,
    #[resource] ix: &Interactions,
    #[resource] messages: &mut MessageSet,
) {
    // add time since last frame to counters

    director.intervall += ix.delta;
    director.total += ix.delta;

    match director.state {
        DirectorState::Spawning(wave_pool) => {
            // only spawn in 1-second intervalls
            if director.intervall >= Duration::from_secs(1) {
                // grant credits
                director.credits += 15 + director.total.as_secs() as u32 / 20 + 5 * director.wave;
                // reset intervall
                director.intervall = Duration::ZERO;

                // randomly select an amount of available credits to spend
                let mut to_spend = (random::<f32>().powi(2) * director.credits as f32) as u32;

                // while credits left to spend
                'outer: loop {
                    // select a random enemy type
                    let mut enemy_ind = random::<usize>() % director.enemies.len();
                    let mut enemy = director.enemies.get(enemy_ind);

                    // downgrade spawn until affordable
                    while match enemy {
                        Some((cost, _)) => *cost > to_spend,
                        None => true,
                    } {
                        // if no downgrade possible, end this spending round
                        if enemy_ind == 0 {
                            break 'outer;
                        }
                        // otherwise, downgrade and try next enemy
                        enemy_ind -= 1;
                        enemy = director.enemies.get(enemy_ind);
                    }

                    // unpack enemy
                    if let Some((cost, spawner)) = enemy {
                        // spawn
                        if spawner(
                            cmd,
                            spritepool,
                            ggez::glam::Vec2::new(rand::random::<f32>() * boundaries.w, -20.),
                        )
                        .is_ok()
                        {
                            // if spawning threw no error, reduce available credits
                            to_spend -= cost;
                            director.credits -= cost;

                            // possible switch state on every spawn
                            let left = wave_pool.saturating_sub(*cost);
                            director.state = if left > 0 {
                                DirectorState::Spawning(left)
                            } else {
                                DirectorState::WaitingForDead
                            }
                        }
                    }
                }
            }
        }
        DirectorState::WaitingForDead => {
            if enemy_query.iter(subworld).count() == 0 {
                messages.insert(mooeye::UiMessage::Extern(GameMessage::NextWave(
                    director.wave as i32 + 1,
                )));
                director.state = DirectorState::WaitingForMenu
            }
        }
        DirectorState::WaitingForMenu => {}
    }
}

/// # Basic skeleton
/// ## Enemy
/// A basic skeleton that has little health and damage and moves slowly.
pub fn spawn_basic_skeleton(
    cmd: &mut CommandBuffer,
    sprite_pool: &sprite::SpritePool,
    pos: Position,
) -> Result<(), GameError> {
    cmd.push((
        pos,
        components::Velocity::new(0., 10.),
        components::Graphics::from(sprite_pool.init_sprite(
            "/sprites/enemies/skeleton_basic",
            Duration::from_secs_f32(0.25),
        )?),
        components::Enemy::new(1, 10),
        components::Health::new(75),
        components::Collision::new_basic(64., 64.),
    ));
    Ok(())
}

/// # Fast skeleton
/// ## Enemy
/// A skeleton that moves faster than the basic skeleton, but also has less health.
/// Moves from side to side.
pub fn spawn_fast_skeleton(
    cmd: &mut CommandBuffer,
    sprite_pool: &sprite::SpritePool,
    pos: Position,
) -> Result<(), GameError> {
    cmd.push((
        pos,
        components::Velocity::new(40., 20.),
        components::BoundaryCollision::new(true, false, true),
        components::Graphics::from(sprite_pool.init_sprite(
            "/sprites/enemies/skeleton_sword",
            Duration::from_secs_f32(0.25),
        )?),
        components::Enemy::new(1, 15),
        components::Health::new(50),
        components::Collision::new_basic(64., 64.),
    ));
    Ok(())
}

/// # Loot goblin
/// ## Enemy
/// A skeleton that does not move down, only sideways.
/// It has lots of health and despawns after a set time, but drops lots of gold on death.
pub fn spawn_loot_skeleton(
    cmd: &mut CommandBuffer,
    sprite_pool: &sprite::SpritePool,
    pos: Position,
) -> Result<(), GameError> {
    cmd.push((
        pos,
        components::Velocity::new(50., 0.),
        components::BoundaryCollision::new(true, false, true),
        components::Graphics::from(sprite_pool.init_sprite(
            "/sprites/enemies/skeleton_loot",
            Duration::from_secs_f32(0.20),
        )?),
        components::Enemy::new(0, 100),
        components::Health::new(150),
        components::LifeDuration::new(Duration::from_secs(15)),
        components::Collision::new_basic(64., 64.),
    ));
    Ok(())
}

/// # Guardian
/// ## Enemy
/// A tanky skeleton with lots of health. Moves slowly, but deals more damage.
/// Reduces damage taken of nearby allies (and self) and heals nearby allies on death.
pub fn spawn_tank_skeleton(
    cmd: &mut CommandBuffer,
    sprite_pool: &sprite::SpritePool,
    pos: Position,
) -> Result<(), GameError> {
    cmd.push((
        pos,
        components::Velocity::new(0., 10.),
        components::Graphics::from(sprite_pool.init_sprite(
            "/sprites/enemies/skeleton_tank",
            Duration::from_secs_f32(0.25),
        )?),
        components::actions::Actions::new().with_effect(ActionEffect::transform(
            ActionEffectTarget::new()
                .with_range(196.)
                .with_enemies_only(true)
                .with_affect_self(true),
            |act| {
                match act {
                    // reduce dmg by 1, but if would be reduced to 0, onyl 20% chance to do so
                    GameAction::TakeDamage { dmg } => {
                        *dmg = (*dmg as f32 * 0.7) as i32;
                    }
                    _ => {}
                }
            },
        )),
        components::OnDeath::new(
            ActionEffect::once(
                ActionEffectTarget::new()
                    .with_range(256.)
                    .with_limit(5)
                    .with_enemies_only(true),
                vec![
                    GameAction::TakeHealing { heal: 40 },
                    GameAction::AddParticle(
                        Particle::new(
                            sprite_pool
                                .init_sprite("/sprites/heal", Duration::from_secs_f32(0.25))?,
                        )
                        .with_duration(Duration::from_secs(1))
                        .with_velocity(0., -15.)
                        .with_relative_position(0., -64.),
                    ),
                ],
            )
            .with_duration(Duration::ZERO),
            MessageSet::new(),
        ),
        components::Enemy::new(2, 25),
        components::Health::new(75),
        components::Collision::new_basic(64., 64.),
    ));
    Ok(())
}

/// # Bannerman
/// ## Enemy
/// A tanky, high-damage skeleton with decent speed.
/// Speeds up nearby allies, considerably higher on death.
pub fn spawn_charge_skeleton(
    cmd: &mut CommandBuffer,
    sprite_pool: &sprite::SpritePool,
    pos: Position,
) -> Result<(), GameError> {
    cmd.push((
        pos,
        components::Velocity::new(0., 21.),
        components::Graphics::from(sprite_pool.init_sprite(
            "/sprites/enemies/skeleton_flag",
            Duration::from_secs_f32(0.25),
        )?),
        // concurrently small speed boost to nearby allies
        components::actions::Actions::new().with_effect(ActionEffect::transform(
            ActionEffectTarget::new()
                .with_affect_self(true)
                .with_range(256.),
            |act| {
                match act {
                    // speed up nearby allies by 50%
                    GameAction::Move { delta } => *delta *= 1.5,
                    _ => {}
                };
            },
        )),
        // on death: speed up nearby allies for a time
        components::OnDeath::new(
            ActionEffect::once(
                ActionEffectTarget::new()
                    .with_range(196.)
                    .with_limit(8)
                    .with_enemies_only(true),
                vec![
                    GameAction::AddParticle(
                        Particle::new(
                            sprite_pool
                                .init_sprite("/sprites/bolt", Duration::from_secs_f32(0.25))?,
                        )
                        .with_duration(Duration::from_secs(5))
                        .with_velocity(0., -10.)
                        .with_relative_position(0., -24.),
                    ),
                    ActionEffect::transform(ActionEffectTarget::new_only_self(), |act| {
                        match act {
                            // speed up nearby allies by 150%
                            GameAction::Move { delta } => *delta *= 2.5,
                            _ => {}
                        };
                    })
                    .with_duration(Duration::from_secs(5))
                    .into(),
                ],
            ),
            MessageSet::new(),
        ),
        components::Enemy::new(2, 45),
        components::Health::new(75),
        components::Collision::new_basic(64., 64.),
    ));
    Ok(())
}

/// # Wizard
/// ## Enemy
/// A tanky but slow caster that heals and speeds up allies on the regular.
pub fn spawn_wizard_skeleton(
    cmd: &mut CommandBuffer,
    sprite_pool: &sprite::SpritePool,
    pos: Position,
) -> Result<(), GameError> {
    cmd.push((
        pos,
        components::Velocity::new(0., 7.),
        components::Graphics::from(sprite_pool.init_sprite(
            "/sprites/enemies/skeleton_flag",
            Duration::from_secs_f32(0.25),
        )?),
        // 'Spell' 1: Speed up a nearby ally for 3 seconds every 5 seconds.
        components::actions::Actions::new()
            .with_effect(ActionEffect::repeat(
                ActionEffectTarget::new()
                    .with_affect_self(false)
                    .with_range(512.)
                    .with_enemies_only(true)
                    .with_limit(1),
                vec![
                    GameAction::AddParticle(
                        Particle::new(
                            sprite_pool
                                .init_sprite("/sprites/bolt", Duration::from_secs_f32(0.25))?,
                        )
                        .with_duration(Duration::from_secs(3))
                        .with_velocity(0., -10.)
                        .with_relative_position(0., -24.),
                    ),
                    ActionEffect::transform(ActionEffectTarget::new_only_self(), |act| {
                        match act {
                            // speed up an ally by 250%
                            GameAction::Move { delta } => *delta *= 3.5,
                            _ => {}
                        };
                    })
                    .with_duration(Duration::from_secs(3))
                    .into(),
                ],
                Duration::from_secs(5),
            ))
            // 'Spell' 2: Heal a nearby ally every 8 seconds.
            .with_effect(ActionEffect::repeat(
                ActionEffectTarget::new()
                    .with_affect_self(false)
                    .with_range(512.)
                    .with_enemies_only(true)
                    .with_limit(1),
                vec![
                    GameAction::AddParticle(
                        Particle::new(
                            sprite_pool
                                .init_sprite("/sprites/heal", Duration::from_secs_f32(0.25))?,
                        )
                        .with_duration(Duration::from_secs(3))
                        .with_velocity(0., -10.)
                        .with_relative_position(0., -24.),
                    ),
                    GameAction::TakeHealing { heal: 75 },
                ],
                Duration::from_secs(8),
            )),
        components::Enemy::new(3, 75),
        components::Health::new(150),
        components::Collision::new_basic(64., 64.),
    ));
    Ok(())
}

/// # Stone Golem
/// ## Enemy
/// A very tanky and slow enemy that spawns multiple smaller skeletons on death.
pub fn spawn_splitter(
    cmd: &mut CommandBuffer,
    sprite_pool: &sprite::SpritePool,
    pos: Position,
) -> Result<(), GameError> {
    cmd.push((
        pos,
        components::Velocity::new(0., 8.),
        components::Graphics::from(sprite_pool.init_sprite(
            "/sprites/enemies/skeleton_tank",
            Duration::from_secs_f32(0.25),
        )?),
        // on death: speed up nearby allies for a time
        components::OnDeath::new(
            GameAction::spawn(|_, vec, sprite_pool, cmd| {
                for _ in 0..3 {
                    if spawn_basic_skeleton(
                        cmd,
                        sprite_pool,
                        vec + ggez::glam::Vec2::new(
                            (rand::random::<f32>() - 0.5) * 64.,
                            (rand::random::<f32>() - 0.5) * 64.,
                        ),
                    )
                    .is_err()
                    {
                        println!("[ERROR] Spawning function non-functional.");
                    };
                }
            }),
            MessageSet::new(),
        ),
        components::Enemy::new(3, 65),
        components::Health::new(200),
        components::Collision::new_basic(64., 64.),
    ));
    Ok(())
}

/// # Stormer
/// ## Enemy
/// A nimble enemy. Taking damage grants it damage reduction for a time and speed permanently.
pub fn spawn_stormer(
    cmd: &mut CommandBuffer,
    sprite_pool: &sprite::SpritePool,
    pos: Position,
) -> Result<(), GameError> {
    cmd.push((
        pos,
        components::Velocity::new(0., 8.),
        components::Graphics::from(sprite_pool.init_sprite(
            "/sprites/enemies/skeleton_sword",
            Duration::from_secs_f32(0.25),
        )?),
        components::Actions::new().with_effect(ActionEffect::react(
            ActionEffectTarget::new_only_self(),
            |action| match action {
                // whenever taking damage
                GameAction::TakeDamage { dmg: _ } => {
                    println!("Taking terrible damage!");
                    vec![
                    // gain damage reduction for 2 seconds
                    GameAction::ApplyEffect(Box::new(
                        ActionEffect::transform(
                            ActionEffectTarget::new_only_self(),
                            |act| match act {
                                GameAction::TakeDamage { dmg } => {
                                    println!("Terrible damage again!");
                                    *dmg /= 10;
                                },
                                _ => {}
                            },
                        )
                        .with_duration(Duration::from_secs(2)),
                    )),
                    // and 30% speed permanently
                    GameAction::ApplyEffect(Box::new(ActionEffect::transform(
                        ActionEffectTarget::new_only_self(),
                        |act| match act {
                            GameAction::Move { delta } => *delta *= 1.3,
                            _ => {}
                        },
                    ))),
                ]
                .into()},
                _ => GameAction::None.into(),
            },
        )),
        components::Enemy::new(3, 70),
        components::Health::new(100),
        components::Collision::new_basic(64., 64.),
    ));
    Ok(())
}
