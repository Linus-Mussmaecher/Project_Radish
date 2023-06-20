use std::time::Duration;

use super::{components, components::actions};
use legion::systems::CommandBuffer;

/// # Basic skeleton
/// ## Enemy
/// A basic skeleton that has little health and damage and moves slowly.
pub fn spawn_basic_skeleton(cmd: &mut CommandBuffer, pos: components::Position) {
    cmd.push((
        pos,
        components::Velocity::new(0., 10.),
        components::Graphics::new(
            "/sprites/enemies/skeleton_basic",
            Duration::from_secs_f32(0.25),
        ),
        components::Enemy::new(1, 10, 0),
        components::Health::new(75),
        components::Collision::new_basic(64., 64.),
    ));
}

/// # Fast skeleton
/// ## Enemy
/// A skeleton that moves faster than the basic skeleton, but also has less health.
/// Moves from side to side.
pub fn spawn_fast_skeleton(cmd: &mut CommandBuffer, pos: components::Position) {
    cmd.push((
        pos,
        components::Velocity::new(40., 20.),
        components::BoundaryCollision::new(true, false, true),
        components::Graphics::new(
            "/sprites/enemies/skeleton_sword",
            Duration::from_secs_f32(0.25),
        ),
        components::Enemy::new(1, 15, 1),
        components::Health::new(50),
        components::Collision::new_basic(64., 64.),
    ));
}

/// # Dodging skeleton
/// ## Enemy
/// A fast skeleton that regularly and does a short sprint
pub fn spawn_dodge_skeleton(cmd: &mut CommandBuffer, pos: components::Position) {
    cmd.push((
        pos,
        components::Velocity::new(10., 22.),
        components::BoundaryCollision::new(true, false, true),
        components::Graphics::new(
            "/sprites/enemies/skeleton_sword",
            Duration::from_secs_f32(0.25),
        ),
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
        components::Enemy::new(1, 15, 2),
        components::Health::new(50),
        components::Collision::new_basic(64., 64.),
    ));
}

/// #Jump Skeleton
/// ## Enemy
/// A fast enemy that runs sideways when hit.
pub fn spawn_jump_skeleton(cmd: &mut CommandBuffer, pos: components::Position) {
    cmd.push((
        pos,
        components::Velocity::new(5., 25.),
        components::BoundaryCollision::new(true, false, true),
        components::Graphics::new(
            "/sprites/enemies/skeleton_sword",
            Duration::from_secs_f32(0.25),
        ),
        components::Actions::new().with_effect(actions::ActionEffect::react(
            actions::ActionEffectTarget::new_only_self(),
            |act| match act {
                actions::GameAction::TakeDamage { dmg: _ } => actions::GameAction::ApplyEffect(
                    actions::ActionEffect::transform(
                        actions::ActionEffectTarget::new_only_self(),
                        |action| {
                            match action {
                                actions::GameAction::Move { delta } => {
                                    delta.x *= 25.;
                                    delta.y *= 0.;
                                }
                                _ => {}
                            };
                        },
                    )
                    .with_duration(Duration::from_secs(2))
                    .into(),
                )
                .into(),
                _ => actions::GameAction::None.into(),
            },
        )),
        components::Enemy::new(1, 15, 3),
        components::Health::new(50),
        components::Collision::new_basic(64., 64.),
    ));
}

/// # Dynamite skeleton
/// ## Enemy
/// A simple enemy that blows up on death..
pub fn spawn_dynamite_skeleton(cmd: &mut CommandBuffer, pos: components::Position) {
    cmd.push((
        pos,
        components::Velocity::new(0., 10.),
        components::Graphics::new(
            "/sprites/enemies/skeleton_basic",
            Duration::from_secs_f32(0.25),
        ),
        components::Actions::new().with_effect(actions::ActionEffect::on_death(
            actions::ActionEffectTarget::new()
                .with_range(128.)
                .with_enemies_only(true),
            actions::RemoveSource::HealthLoss,
            actions::GameAction::TakeDamage { dmg: 30 },
        )),
        components::Enemy::new(3, 30, 7),
        components::Health::new(150),
        components::Collision::new_basic(64., 64.),
    ));
}

/// # Loot goblin
/// ## Enemy
/// A skeleton that does not move down, only sideways.
/// It has lots of health and despawns after a set time, but drops lots of gold on death.
pub fn spawn_loot_skeleton(cmd: &mut CommandBuffer, pos: components::Position) {
    cmd.push((
        pos + ggez::glam::Vec2::new(0., 120.),
        components::Velocity::new(50., 0.),
        components::BoundaryCollision::new(true, false, true),
        components::Graphics::new(
            "/sprites/enemies/skeleton_loot",
            Duration::from_secs_f32(0.20),
        ),
        components::Enemy::new(0, 100, 8),
        components::Health::new(150),
        components::LifeDuration::new(Duration::from_secs(15)),
        components::Collision::new_basic(64., 64.),
    ));
}

/// # Catapult 
/// ## Enemy
/// An enemy that stays put, but regularly accelerates other enemies passing over it.
pub fn spawn_catapult(cmd: &mut CommandBuffer, pos: components::Position){
    cmd.push((
        pos + ggez::glam::Vec2::new(0., 180.),
        components::Graphics::new(
            "/sprites/enemies/skeleton_loot",
            Duration::from_secs_f32(0.20),
        ),
        components::Actions::new()
            .with_effect(actions::ActionEffect::repeat(
                actions::ActionEffectTarget::new().with_limit(1).with_range(64.).with_affect_self(false),
                actions::GameAction::ApplyEffect(Box::new(
                    actions::ActionEffect::repeat(
                        actions::ActionEffectTarget::new_only_self(),
                        actions::GameAction::Move {
                            delta: ggez::glam::Vec2::new(0., 5.),
                        },
                        Duration::from_secs_f32(0.02),
                    )
                    .with_duration(Duration::from_secs_f32(1.)),
                )),
                Duration::from_secs(1),
            )),
        components::Enemy::new(0, 20, 9),
        components::Health::new(250),
        components::Collision::new_basic(64., 64.),
    ));
}


/// # Guardian
/// ## Enemy
/// A tanky skeleton with lots of health. Moves slowly, but deals more damage.
/// Reduces damage taken of nearby allies (and self) and heals nearby allies on death.
pub fn spawn_tank_skeleton(cmd: &mut CommandBuffer, pos: components::Position) {
    cmd.push((
        pos,
        components::Velocity::new(0., 10.),
        components::Graphics::new(
            "/sprites/enemies/skeleton_tank",
            Duration::from_secs_f32(0.25),
        ),
        components::actions::Actions::new()
            .with_effect(actions::ActionEffect::transform(
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
            ))
            .with_effect(actions::ActionEffect::on_death(
                actions::ActionEffectTarget::new()
                    .with_range(256.)
                    .with_limit(5)
                    .with_enemies_only(true),
                actions::RemoveSource::HealthLoss,
                vec![
                    actions::GameAction::TakeHealing { heal: 40 },
                    actions::GameAction::AddParticle(
                        components::graphics::Particle::new(
                            "/sprites/effects/heal",
                            Duration::from_secs_f32(0.25),
                        )
                        .with_duration(Duration::from_secs(1))
                        .with_velocity(0., -15.)
                        .with_relative_position(0., -64.),
                    ),
                ],
            )),
        components::Enemy::new(2, 20, 10),
        components::Health::new(75),
        components::Collision::new_basic(64., 64.),
    ));
}

/// # Bannerman
/// ## Enemy
/// A tanky, high-damage skeleton with decent speed.
/// Speeds up nearby allies, considerably higher on death.
pub fn spawn_charge_skeleton(cmd: &mut CommandBuffer, pos: components::Position) {
    cmd.push((
        pos,
        components::Velocity::new(0., 21.),
        components::Graphics::new(
            "/sprites/enemies/skeleton_flag",
            Duration::from_secs_f32(0.25),
        ),
        // concurrently small speed boost to nearby allies
        components::actions::Actions::new()
            .with_effect(actions::ActionEffect::transform(
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
            ))
            // on death: speed up nearby allies for a time
            .with_effect(actions::ActionEffect::on_death(
                actions::ActionEffectTarget::new()
                    .with_range(196.)
                    .with_limit(8)
                    .with_enemies_only(true),
                actions::RemoveSource::HealthLoss,
                vec![
                    actions::GameAction::AddParticle(
                        components::graphics::Particle::new(
                            "/sprites/effects/bolt",
                            Duration::from_secs_f32(0.25),
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
            )),
        components::Enemy::new(2, 20, 11),
        components::Health::new(75),
        components::Collision::new_basic(64., 64.),
    ));
}

/// # Wizard
/// ## Enemy
/// A tanky but slow caster that heals and speeds up allies on the regular.
pub fn spawn_wizard_skeleton(cmd: &mut CommandBuffer, pos: components::Position) {
    cmd.push((
        pos,
        components::Velocity::new(0., 7.),
        components::Graphics::new(
            "/sprites/enemies/skeleton_wizard",
            Duration::from_secs_f32(0.25),
        ),
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
                            "/sprites/effects/bolt",
                            Duration::from_secs_f32(0.25),
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
                            "/sprites/effects/heal",
                            Duration::from_secs_f32(0.25),
                        )
                        .with_duration(Duration::from_secs(3))
                        .with_velocity(0., -10.)
                        .with_relative_position(0., -24.),
                    ),
                    actions::GameAction::TakeHealing { heal: 75 },
                ],
                Duration::from_secs(8),
            )),
        components::Enemy::new(2, 25, 12),
        components::Health::new(150),
        components::Collision::new_basic(64., 64.),
    ));
}

/// # Wizard 2
/// ## Enemy
/// A tanky but slow caster that heals allies and gives them a damage protection aura.
pub fn spawn_wizard_skeleton2(cmd: &mut CommandBuffer, pos: components::Position) {
    cmd.push((
        pos,
        components::Velocity::new(0., 7.),
        components::Graphics::new(
            "/sprites/enemies/skeleton_wizard2",
            Duration::from_secs_f32(0.25),
        ),
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
                            "/sprites/effects/shield",
                            Duration::from_secs_f32(0.25),
                        )
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
                            "/sprites/effects/heal",
                            Duration::from_secs_f32(0.25),
                        )
                        .with_duration(Duration::from_secs(3))
                        .with_velocity(0., -10.)
                        .with_relative_position(0., -24.),
                    ),
                    actions::GameAction::TakeHealing { heal: 60 },
                ],
                Duration::from_secs(5),
            )),
        components::Enemy::new(2, 25, 13),
        components::Health::new(150),
        components::Collision::new_basic(64., 64.),
    ));
}

/// # Wizard 3
/// ## Enemy
/// A tanky but slow caster that spawns skeletons and damages nearby allies for an area speed boost.
pub fn spawn_wizard_skeleton3(cmd: &mut CommandBuffer, pos: components::Position) {
    cmd.push((
        pos,
        components::Velocity::new(0., 7.),
        components::Graphics::new(
            "/sprites/enemies/skeleton_wizard2",
            Duration::from_secs_f32(0.25),
        ),
        // 'Spell' 1: Spawn a skeleton every 30 seconds.
        components::actions::Actions::new()
            .with_effect(actions::ActionEffect::repeat(
                actions::ActionEffectTarget::new_only_self(),
                actions::GameAction::spawn(|_, pos, cmd| {
                    spawn_basic_skeleton(
                        cmd,
                        pos + ggez::glam::Vec2 {
                            x: -16. + 32. * rand::random::<f32>(),
                            y: -32.,
                        },
                    );
                }),
                Duration::from_secs(30),
            ))
            // 'Spell' 2: Damage a group of nearby allies and speed them up
            .with_effect(actions::ActionEffect::repeat(
                actions::ActionEffectTarget::new()
                    .with_affect_self(false)
                    .with_range(512.)
                    .with_enemies_only(true)
                    .with_limit(5),
                vec![
                    actions::GameAction::TakeDamage { dmg: 25 },
                    actions::GameAction::AddParticle(
                        components::graphics::Particle::new(
                            "/sprites/effects/bolt",
                            Duration::from_secs_f32(0.25),
                        )
                        .with_duration(Duration::from_secs(3))
                        .with_velocity(0., -10.)
                        .with_relative_position(0., -24.),
                    ),
                    actions::ActionEffect::transform(
                        actions::ActionEffectTarget::new_only_self(),
                        |act| {
                            match act {
                                // speed up an ally by 150%
                                actions::GameAction::Move { delta } => *delta *= 2.5,
                                _ => {}
                            };
                        },
                    )
                    .with_duration(Duration::from_secs(3))
                    .into(),
                ],
                Duration::from_secs(7),
            )),
        components::Enemy::new(2, 25, 13),
        components::Health::new(150),
        components::Collision::new_basic(64., 64.),
    ));
}

/// # Stone Golem
/// ## Enemy
/// A very tanky and slow enemy that spawns multiple smaller skeletons on death.
pub fn spawn_splitter(cmd: &mut CommandBuffer, pos: components::Position) {
    cmd.push((
        pos,
        components::Velocity::new(0., 8.),
        components::Graphics::new("/sprites/enemies/golem", Duration::from_secs_f32(0.25)),
        components::actions::Actions::new().with_effect(actions::ActionEffect::on_death(
            actions::ActionEffectTarget::new_only_self(),
            actions::RemoveSource::HealthLoss,
            actions::GameAction::spawn(|_, vec, cmd| {
                for _ in 0..3 {
                    spawn_basic_skeleton(
                        cmd,
                        vec + ggez::glam::Vec2::new(
                            (rand::random::<f32>() - 0.5) * 64.,
                            (rand::random::<f32>() - 0.5) * 64.,
                        ),
                    );
                }
            }),
        )),
        components::Enemy::new(2, 20, 15),
        components::Health::new(200),
        components::Collision::new_basic(64., 64.),
    ));
}

/// # Ghost
/// ## Enemy
/// A nimble enemy. Taking damage grants it damage reduction for a time and speed permanently.
pub fn spawn_ghost(cmd: &mut CommandBuffer, pos: components::Position) {
    cmd.push((
        pos,
        components::Velocity::new(0., 8.),
        components::Graphics::new("/sprites/enemies/ghost", Duration::from_secs_f32(0.25)),
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
        components::Enemy::new(2, 30, 16),
        components::Health::new(100),
        components::Collision::new_basic(64., 64.),
    ));
}

/// # Blood fiend
/// ## Enemy
/// A creature that distributes damage taken on nearby enemies
pub fn spawn_blood_fiend(cmd: &mut CommandBuffer, pos: components::Position) {
    cmd.push((
        pos,
        components::Velocity::new(0., 12.),
        components::Graphics::new(
            "/sprites/enemies/skeleton_basic",
            Duration::from_secs_f32(0.25),
        ),
        components::Actions::new()
            // reduce damage taken by 60%
            .with_effect(actions::ActionEffect::transform(
                actions::ActionEffectTarget::new_only_self(),
                |act| {
                    match act {
                        actions::GameAction::TakeDamage { dmg } => {
                            *dmg = (*dmg as f32 * 0.4) as i32
                        }
                        _ => {}
                    };
                },
            ))
            // distribute damage to nearby allies
            .with_effect(actions::ActionEffect::react(
                actions::ActionEffectTarget::new_only_self(),
                |act| match act {
                    actions::GameAction::TakeDamage { dmg } => actions::ActionEffect::once(
                        actions::ActionEffectTarget::new()
                            .with_range(256.)
                            .with_limit(3)
                            .with_affect_self(false),
                        actions::GameAction::TakeDamage { dmg: dmg / 5 },
                    )
                    .into(),
                    _ => actions::GameAction::None.into(),
                },
            ))
            // heal on death
            .with_effect(actions::ActionEffect::on_death(
                actions::ActionEffectTarget::new()
                    .with_range(256.)
                    .with_limit(5)
                    .with_enemies_only(true),
                actions::RemoveSource::HealthLoss,
                vec![
                    actions::GameAction::TakeHealing { heal: 40 },
                    actions::GameAction::AddParticle(
                        components::graphics::Particle::new(
                            "/sprites/effects/heal",
                            Duration::from_secs_f32(0.25),
                        )
                        .with_duration(Duration::from_secs(1))
                        .with_velocity(0., -15.)
                        .with_relative_position(0., -64.),
                    ),
                ],
            )),
        components::Enemy::new(2, 25, 17),
        components::Health::new(100),
        components::Collision::new_basic(64., 64.),
    ));
}

pub fn spawn_legionnaire(cmd: &mut CommandBuffer, pos: components::Position) {
    cmd.push((
        pos,
        components::Velocity::new(0., 15.),
        components::Graphics::new(
            "/sprites/enemies/skeleton_tank",
            Duration::from_secs_f32(0.25),
        ),
        components::Actions::new().with_effect(actions::ActionEffect::repeat(
            actions::ActionEffectTarget::new_only_self(),
            actions::GameAction::ApplyEffect(
                actions::ActionEffect::transform(
                    actions::ActionEffectTarget::new_only_self(),
                    |action| {
                        match action {
                            actions::GameAction::Move { delta } => {
                                delta.x *= 0.;
                                delta.y *= 0.3;
                            },
                            actions::GameAction::TakeDamage { dmg } => {
                                *dmg /= 8;
                            },
                            _ => {}
                        };
                    },
                )
                .with_duration(Duration::from_secs(5))
                .into(),
            ),
            Duration::from_secs(8),
        )),
        components::Enemy::new(2, 30, 18),
        components::Health::new(120),
        components::Collision::new_basic(64., 64.),
    ));
}