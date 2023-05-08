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

#[derive(Clone)]
pub struct Director {
    wave: u32,
    wave_pool: u32,
    intervall: Duration,
    total: Duration,
    credits: u32,
    enemies: Vec<(
        u32,
        fn(&mut CommandBuffer, &sprite::SpritePool, Position) -> Result<(), GameError>,
    )>,
}

impl Director {
    pub fn new() -> Self {
        Self {
            wave: 0,
            wave_pool: 1000,
            intervall: Duration::ZERO,
            total: Duration::ZERO,
            credits: 0,
            enemies: vec![
                (040, spawn_basic_skeleton),
                (070, spawn_fast_skeleton),
                (150, spawn_tank_skeleton),
                (200, spawn_charge_skeleton),
                (300, spawn_loot_skeleton),
            ],
        }
    }
}

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

    // if a 1-second intervall has passed, attempt a spawn

    if director.intervall >= Duration::from_secs(1) {
        // spawn enemies
        if director.wave_pool > 0 {
            // grant credits
            director.credits += 15 + director.total.as_secs() as u32 / 20 + 5 * director.wave;
            // reset intervall
            director.intervall = Duration::ZERO;

            // randomly select an amount of available credits to spend
            let mut to_spend = (random::<f32>().powi(2) * director.credits as f32) as u32;
            // if to_spend >= 40 {
            //     println!("Spending {} of {} credits.", to_spend, director.credits);
            // }

            // while credits left to spend
            'outer: loop {
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
                        director.wave_pool = director.wave_pool.saturating_sub(*cost);
                    }
                }
            }
        } else if enemy_query.iter(subworld).count() == 0{
            // wait for enemies to despawn
            director.wave += 1;
            messages.insert(mooeye::UiMessage::Extern(GameMessage::NextWave(
                director.wave as i32,
            )));
            director.wave_pool = director.wave * 1000 + 500;
        }
    }
}

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

pub fn spawn_fast_skeleton(
    cmd: &mut CommandBuffer,
    sprite_pool: &sprite::SpritePool,
    pos: Position,
) -> Result<(), GameError> {
    cmd.push((
        pos,
        components::Velocity::new(35., 10.),
        components::BoundaryCollision::new(true, false, true),
        components::Graphics::from(sprite_pool.init_sprite(
            "/sprites/enemies/skeleton_sword",
            Duration::from_secs_f32(0.25),
        )?),
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
        components::Enemy::new(1, 15),
        components::Health::new(50),
        components::Collision::new_basic(64., 64.),
    ));
    Ok(())
}

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
                            // speed up nearby allies by 50%
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
