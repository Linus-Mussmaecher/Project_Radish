use std::time::Duration;

use crate::scenes::game_state::game_message;

use super::{components, components::actions};
use ggez::GameError;
use legion::systems::CommandBuffer;

use mooeye::sprite;

/// # Basic skeleton
/// ## Enemy
/// A basic skeleton that has little health and damage and moves slowly.
pub fn spawn_basic_skeleton(
    cmd: &mut CommandBuffer,
    sprite_pool: &sprite::SpritePool,
    pos: components::Position,
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
    pos: components::Position,
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

/// # Dodging skeleton
/// ## Enemy
/// A fast skeleton that regularly and does a short sprint
pub fn spawn_dodge_skeleton(
    cmd: &mut CommandBuffer,
    sprite_pool: &sprite::SpritePool,
    pos: components::Position,
) -> Result<(), GameError> {
    cmd.push((
        pos,
        components::Velocity::new(10., 22.),
        components::BoundaryCollision::new(true, false, true),
        components::Graphics::from(sprite_pool.init_sprite(
            "/sprites/enemies/skeleton_sword",
            Duration::from_secs_f32(0.25),
        )?),
        components::Actions::new().with_effect(actions::ActionEffect::repeat(
            actions::ActionEffectTarget::new_only_self(),
            actions::GameAction::ApplyEffect(
                actions::ActionEffect::transform(
                    actions::ActionEffectTarget::new_only_self(),
                    |action| {
                        match action {
                            actions::GameAction::Move { delta } => {
                                delta.x *= 10.;
                                delta.y *= 2.;
                            }
                            _ => {}
                        };
                    },
                )
                .with_duration(Duration::from_secs(2))
                .into(),
            ),
            Duration::from_secs(8),
        )),
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
    pos: components::Position,
) -> Result<(), GameError> {
    cmd.push((
        pos + ggez::glam::Vec2::new(0., 150.),
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
    pos: components::Position,
) -> Result<(), GameError> {
    cmd.push((
        pos,
        components::Velocity::new(0., 10.),
        components::Graphics::from(sprite_pool.init_sprite(
            "/sprites/enemies/skeleton_tank",
            Duration::from_secs_f32(0.25),
        )?),
        components::actions::Actions::new().with_effect(actions::ActionEffect::transform(
            actions::ActionEffectTarget::new()
                .with_range(196.)
                .with_enemies_only(true)
                .with_affect_self(true),
            |act| {
                match act {
                    // reduce dmg by 1, but if would be reduced to 0, onyl 20% chance to do so
                    actions::GameAction::TakeDamage { dmg } => {
                        *dmg = (*dmg as f32 * 0.7) as i32;
                    }
                    _ => {}
                }
            },
        )),
        components::OnDeath::new(
            actions::ActionEffect::once(
                actions::ActionEffectTarget::new()
                    .with_range(256.)
                    .with_limit(5)
                    .with_enemies_only(true),
                vec![
                    actions::GameAction::TakeHealing { heal: 40 },
                    actions::GameAction::AddParticle(
                        components::graphics::Particle::new(
                            sprite_pool.init_sprite(
                                "/sprites/effects/heal",
                                Duration::from_secs_f32(0.25),
                            )?,
                        )
                        .with_duration(Duration::from_secs(1))
                        .with_velocity(0., -15.)
                        .with_relative_position(0., -64.),
                    ),
                ],
            )
            .with_duration(Duration::ZERO),
            game_message::MessageSet::new(),
        ),
        components::Enemy::new(2, 20),
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
    pos: components::Position,
) -> Result<(), GameError> {
    cmd.push((
        pos,
        components::Velocity::new(0., 21.),
        components::Graphics::from(sprite_pool.init_sprite(
            "/sprites/enemies/skeleton_flag",
            Duration::from_secs_f32(0.25),
        )?),
        // concurrently small speed boost to nearby allies
        components::actions::Actions::new().with_effect(actions::ActionEffect::transform(
            actions::ActionEffectTarget::new()
                .with_affect_self(true)
                .with_range(256.),
            |act| {
                match act {
                    // speed up nearby allies by 50%
                    actions::GameAction::Move { delta } => *delta *= 1.5,
                    _ => {}
                };
            },
        )),
        // on death: speed up nearby allies for a time
        components::OnDeath::new(
            actions::ActionEffect::once(
                actions::ActionEffectTarget::new()
                    .with_range(196.)
                    .with_limit(8)
                    .with_enemies_only(true),
                vec![
                    actions::GameAction::AddParticle(
                        components::graphics::Particle::new(
                            sprite_pool.init_sprite(
                                "/sprites/effects/bolt",
                                Duration::from_secs_f32(0.25),
                            )?,
                        )
                        .with_duration(Duration::from_secs(5))
                        .with_velocity(0., -10.)
                        .with_relative_position(0., -24.),
                    ),
                    actions::ActionEffect::transform(
                        actions::ActionEffectTarget::new_only_self(),
                        |act| {
                            match act {
                                // speed up nearby allies by 150%
                                actions::GameAction::Move { delta } => *delta *= 2.5,
                                _ => {}
                            };
                        },
                    )
                    .with_duration(Duration::from_secs(5))
                    .into(),
                ],
            ),
            game_message::MessageSet::new(),
        ),
        components::Enemy::new(2, 20),
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
    pos: components::Position,
) -> Result<(), GameError> {
    cmd.push((
        pos,
        components::Velocity::new(0., 7.),
        components::Graphics::from(sprite_pool.init_sprite(
            "/sprites/enemies/skeleton_wizard",
            Duration::from_secs_f32(0.25),
        )?),
        // 'Spell' 1: Speed up a nearby ally for 3 seconds every 5 seconds.
        components::actions::Actions::new()
            .with_effect(actions::ActionEffect::repeat(
                actions::ActionEffectTarget::new()
                    .with_affect_self(false)
                    .with_range(512.)
                    .with_enemies_only(true)
                    .with_limit(1),
                vec![
                    actions::GameAction::AddParticle(
                        components::graphics::Particle::new(
                            sprite_pool.init_sprite(
                                "/sprites/effects/bolt",
                                Duration::from_secs_f32(0.25),
                            )?,
                        )
                        .with_duration(Duration::from_secs(3))
                        .with_velocity(0., -10.)
                        .with_relative_position(0., -24.),
                    ),
                    actions::ActionEffect::transform(
                        actions::ActionEffectTarget::new_only_self(),
                        |act| {
                            match act {
                                // speed up an ally by 250%
                                actions::GameAction::Move { delta } => *delta *= 3.5,
                                _ => {}
                            };
                        },
                    )
                    .with_duration(Duration::from_secs(3))
                    .into(),
                ],
                Duration::from_secs(5),
            ))
            // 'Spell' 2: Heal a nearby ally every 8 seconds.
            .with_effect(actions::ActionEffect::repeat(
                actions::ActionEffectTarget::new()
                    .with_affect_self(false)
                    .with_range(512.)
                    .with_enemies_only(true)
                    .with_limit(1),
                vec![
                    actions::GameAction::AddParticle(
                        components::graphics::Particle::new(
                            sprite_pool.init_sprite(
                                "/sprites/effects/heal",
                                Duration::from_secs_f32(0.25),
                            )?,
                        )
                        .with_duration(Duration::from_secs(3))
                        .with_velocity(0., -10.)
                        .with_relative_position(0., -24.),
                    ),
                    actions::GameAction::TakeHealing { heal: 75 },
                ],
                Duration::from_secs(8),
            )),
        components::Enemy::new(3, 35),
        components::Health::new(150),
        components::Collision::new_basic(64., 64.),
    ));
    Ok(())
}

/// # Wizard 2
/// ## Enemy
/// A tanky but slow caster that heals allies and gives them a damage protection aura.
pub fn spawn_wizard_skeleton2(
    cmd: &mut CommandBuffer,
    sprite_pool: &sprite::SpritePool,
    pos: components::Position,
) -> Result<(), GameError> {
    cmd.push((
        pos,
        components::Velocity::new(0., 7.),
        components::Graphics::from(sprite_pool.init_sprite(
            "/sprites/enemies/skeleton_wizard2",
            Duration::from_secs_f32(0.25),
        )?),
        // 'Spell' 1: Speed up a nearby ally for 3 seconds every 5 seconds.
        components::actions::Actions::new()
            .with_effect(actions::ActionEffect::repeat(
                actions::ActionEffectTarget::new()
                    .with_affect_self(false)
                    .with_range(512.)
                    .with_enemies_only(true)
                    .with_limit(1),
                vec![
                    actions::GameAction::AddParticle(
                        components::graphics::Particle::new(sprite_pool.init_sprite(
                            "/sprites/effects/shield",
                            Duration::from_secs_f32(0.25),
                        )?)
                        .with_duration(Duration::from_secs(3)),
                    ),
                    actions::ActionEffect::transform(
                        actions::ActionEffectTarget::new_only_self(),
                        |act| {
                            match act {
                                // speed up an ally by 250%
                                actions::GameAction::TakeDamage { dmg } => *dmg /= 3,
                                _ => {}
                            };
                        },
                    )
                    .with_duration(Duration::from_secs(3))
                    .into(),
                ],
                Duration::from_secs(8),
            ))
            // 'Spell' 2: Heal a nearby ally every 8 seconds.
            .with_effect(actions::ActionEffect::repeat(
                actions::ActionEffectTarget::new()
                    .with_affect_self(false)
                    .with_range(512.)
                    .with_enemies_only(true)
                    .with_limit(1),
                vec![
                    actions::GameAction::AddParticle(
                        components::graphics::Particle::new(
                            sprite_pool.init_sprite(
                                "/sprites/effects/heal",
                                Duration::from_secs_f32(0.25),
                            )?,
                        )
                        .with_duration(Duration::from_secs(3))
                        .with_velocity(0., -10.)
                        .with_relative_position(0., -24.),
                    ),
                    actions::GameAction::TakeHealing { heal: 60 },
                ],
                Duration::from_secs(5),
            )),
        components::Enemy::new(3, 35),
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
    pos: components::Position,
) -> Result<(), GameError> {
    cmd.push((
        pos,
        components::Velocity::new(0., 8.),
        components::Graphics::from(
            sprite_pool.init_sprite("/sprites/enemies/golem", Duration::from_secs_f32(0.25))?,
        ),
        // on death: speed up nearby allies for a time
        components::OnDeath::new(
            actions::GameAction::spawn(|_, vec, sprite_pool, cmd| {
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
            game_message::MessageSet::new(),
        ),
        components::Enemy::new(3, 20),
        components::Health::new(200),
        components::Collision::new_basic(64., 64.),
    ));
    Ok(())
}

/// # Ghost
/// ## Enemy
/// A nimble enemy. Taking damage grants it damage reduction for a time and speed permanently.
pub fn spawn_ghost(
    cmd: &mut CommandBuffer,
    sprite_pool: &sprite::SpritePool,
    pos: components::Position,
) -> Result<(), GameError> {
    cmd.push((
        pos,
        components::Velocity::new(0., 8.),
        components::Graphics::from(
            sprite_pool.init_sprite("/sprites/enemies/ghost", Duration::from_secs_f32(0.25))?,
        ),
        components::Actions::new().with_effect(actions::ActionEffect::react(
            actions::ActionEffectTarget::new_only_self(),
            |action| match action {
                // whenever taking damage
                actions::GameAction::TakeDamage { dmg: _ } => {
                    vec![
                        // gain damage reduction for 2 seconds
                        actions::GameAction::ApplyEffect(Box::new(
                            actions::ActionEffect::transform(
                                actions::ActionEffectTarget::new_only_self(),
                                |act| match act {
                                    actions::GameAction::TakeDamage { dmg } => {
                                        *dmg /= 5;
                                    }
                                    _ => {}
                                },
                            )
                            .with_duration(Duration::from_secs(2)),
                        )),
                        // and 30% speed permanently
                        actions::GameAction::ApplyEffect(Box::new(
                            actions::ActionEffect::transform(
                                actions::ActionEffectTarget::new_only_self(),
                                |act| match act {
                                    actions::GameAction::Move { delta } => *delta *= 1.3,
                                    _ => {}
                                },
                            ),
                        )),
                    ]
                    .into()
                }
                _ => actions::GameAction::None.into(),
            },
        )),
        components::Enemy::new(3, 40),
        components::Health::new(100),
        components::Collision::new_basic(64., 64.),
    ));
    Ok(())
}
