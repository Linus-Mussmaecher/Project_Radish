use std::{time::Duration, vec};

use mooeye::sprite::SpritePool;
use tinyvec::tiny_vec;

use crate::scenes::game_state::components::{
    self, actions::*, graphics::Particle, spell::MAX_SPELL_SLOTS,
};

use super::Spell;


pub(super) fn construct_ice_bomb(sprite_pool: &SpritePool) -> Spell {
    Spell::new(
        "Ice Bomb",
        "Launch a fast icy projectile that deals high damage on impact and drops an ice crystal that slows nearby enemies and deals area damage when exploding.",
        sprite_pool
            .init_sprite_unchecked("/sprites/spells/icebomb", Duration::ZERO),
            "/audio/sounds/spells/icebomb_cast",
        GameAction::spawn(|_, pos, cmd| {
            cmd.push((
                pos,
                components::LifeDuration::new(Duration::from_secs(10)),
                components::Graphics::new(
                    "/sprites/spells/icebomb",
                    Duration::from_secs_f32(0.2),
                ),
                components::Velocity::new(0., -520.),
                components::Collision::new(32., 32., |e1, e2|
                        vec![
                            (e1, GameAction::Remove(RemoveSource::ProjectileCollision)),
                            (e1, GameAction::play_sound("/audio/sounds/spells/icebomb_hit")),
                            (e2, GameAction::TakeDamage { dmg: 25 }),
                            (e1, GameAction::spawn(|_, pos, cmd|{
                                cmd.push((
                                    pos,
                                    components::LifeDuration::new(Duration::from_secs(5)),
                                    components::Graphics::new("/sprites/spells/icepulse", Duration::from_secs_f32(0.25))
                                        .with_sprite_variant(1),
                                    components::actions::Actions::new()
                                        .with_effect(
                                            ActionEffect::once(
                                                ActionEffectTarget::new()
                                                    .with_enemies_only(true)
                                                    .with_range(128.),
                                                GameAction::TakeDamage { dmg: 15 },
                                            )
                                            .with_duration(Duration::from_secs_f32(5.)),
                                        )
                                        .with_effect(ActionEffect::transform(
                                            ActionEffectTarget::new()
                                                .with_enemies_only(true)
                                                .with_range(128.),
                                            |action| match action {
                                                GameAction::Move { delta } => *delta *= 0.35,
                                                _ => {}
                                            },
                                        ))
                                        .with_effect(ActionEffect::on_death(
                                            ActionEffectTarget::new_only_self(),
                                            RemoveSource::TimedOut, 
                                            GameAction::play_sound("/audio/sounds/spells/icebomb_explosion"),
                                        )),
                                ));
                            })),
                        ],
                ),
            ));
        }),
        tiny_vec!([f32; MAX_SPELL_SLOTS] => 1.5, 5.),
    )
}



pub(super) fn construct_shard(sprite_pool: &SpritePool) -> Spell {
    Spell::new(
        "Shard of Ice",
        "Throw a shard of ice dealing moderate damage and slowing. On hit, split into three smaller shards that deal less damage but slow more.",
        {
                let mut s = sprite_pool.init_sprite_unchecked("/sprites/spells/icebomb", Duration::ZERO);
                s.set_variant(1);
                s
            },
            "/audio/sounds/spells/shard_cast",
        GameAction::spawn(|_, pos, cmd|{
            cmd.push((
                pos,
                components::Velocity::new(0., -250.),
                components::LifeDuration::new(Duration::from_secs(10)),
                components::Graphics::new("/sprites/spells/icebomb", Duration::from_secs_f32(0.25)),
                components::Collision::new(32., 32., |e1, e2| vec![
                            (e1, GameAction::Remove(RemoveSource::ProjectileCollision)),
                            (e1, GameAction::play_sound("/audio/sounds/spells/shard_hit")),
                            (e2, GameAction::TakeDamage { dmg: 20 }),
                            (e2, ActionEffect::transform(ActionEffectTarget::new_only_self(), |action| match action {
                                GameAction::Move { delta } => *delta *= 0.9,
                                _ => {}
                            }).with_duration(Duration::from_secs(3)).into()),
                            (e2, GameAction::spawn(|enemy, pos, cmd| {
                                for i in -1..2{
                                    cmd.push((
                                        pos + ggez::glam::Vec2::new( 34. * i as f32, 0.),
                                        components::Velocity::new((30 * i) as f32, -250.),
                                        components::LifeDuration::new(Duration::from_secs_f32(0.4)),
                                        components::Graphics::new(
                                            "/sprites/spells/icebomb",
                                            Duration::ZERO,
                                        ).with_sprite_variant(if i == 0 {0} else {2}),
                                        components::Collision::new(32., 32., |e1, e2| vec![
                                                (e1, GameAction::Remove(RemoveSource::ProjectileCollision)),
                                                (e1, GameAction::play_sound("/audio/sounds/spells/shard_hit")),
                                                (e2, GameAction::TakeDamage { dmg: 8 }),
                                                (e2, ActionEffect::transform(ActionEffectTarget::new_only_self(), |action| match action {
                                                    GameAction::Move { delta } => *delta *= 0.7,
                                                    _ => {}
                                                }).with_duration(Duration::from_secs(3)).into()),
                                            ]).with_immunity(enemy),
                                    ));
                                }
                            })),
                        ],)
            ));
        }),
        tiny_vec!([f32; MAX_SPELL_SLOTS] => 3.))
}


pub(super) fn construct_ice_lance(sprite_pool: &SpritePool) -> Spell {
    Spell::new(
        "Ice Lance",
        "Launch a volley of 3 quick-striking ice lances, each dealing damage to a single target and increasing their damage taken.",
        sprite_pool.init_sprite_unchecked("/sprites/spells/icons/icespike_icon", Duration::ZERO),
        "/audio/sounds/spells/lance_cast",
            ActionEffect::repeat(ActionEffectTarget::new_only_self(),
                vec![GameAction::spawn(|_, pos, cmd|{
                    cmd.push(
                        (pos,
                components::LifeDuration::new(Duration::from_secs(8)),
                components::Graphics::new(
                    "/sprites/spells/icespike",
                    Duration::from_secs_f32(0.2),
                ),
                components::Velocity::new(0., -450.),
                components::Collision::new(32., 32., move |e1, e2| vec![
                            (e1, GameAction::AddImmunity { other: e2 }),
                            (e1, GameAction::play_sound("/audio/sounds/spells/lance_hit")),
                            (e1, GameAction::Remove(RemoveSource::ProjectileCollision)),
                            (
                                e2,
                                GameAction::TakeDamage { dmg: 8 }
                            ),
                            (
                                e2,
                                ActionEffect::transform(ActionEffectTarget::new_only_self(), |act|{
                                    match act {
                                        GameAction::TakeDamage{dmg} => {*dmg = (*dmg as f32 * 1.5) as i32;},
                                        _ => {}
                                    }
                                }).into()
                            ),
                        ],),
                    ));
                }),
                GameAction::play_sound("/audio/sounds/spells/lance_cast"),
                ],
            Duration::from_secs_f32(0.2))
            .with_duration(Duration::from_secs_f32(0.7)),
        tiny_vec!([f32; MAX_SPELL_SLOTS] => 2., 2., 2.))
}


pub(super) fn construct_lightning_orb(sprite_pool: &SpritePool) -> Spell {
    Spell::new(
        "Lightning Ball",
        "Launch a ball of lightning that pierces through enemies and deals area damage on every contact.",
        sprite_pool
            .init_sprite_unchecked("/sprites/spells/electroorb", Duration::ZERO),
        None,
        GameAction::spawn(|_, pos, cmd| {
            cmd.push((
                pos,
                components::LifeDuration::new(Duration::from_secs(10)),
                components::Graphics::new(
                    "/sprites/spells/electroorb",
                    Duration::from_secs_f32(0.2),
                ),
                components::Velocity::new(0., -180.),
                components::Collision::new(32., 32., |e1, e2| vec![
                            (e1, GameAction::AddImmunity { other: e2 }),
                            (e1, GameAction::play_sound("/audio/sounds/spells/electroorb_hit")),
                            (
                                e2,
                                ActionEffect::once(
                                    ActionEffectTarget::new()
                                        .with_enemies_only(true)
                                        .with_affect_self(true)
                                        .with_range(128.),
                                    GameAction::TakeDamage { dmg: 20 },
                                )
                                .into(),
                            ),
                        ],),
            ));
        }),
        tiny_vec!([f32; MAX_SPELL_SLOTS] => 1.5, 1.5, 20.)
    )
}


pub(super) fn construct_overload(sprite_pool: &SpritePool) -> Spell {
    Spell::new(
        "Overload",
        "Shoot out an electric spark that overloads the first enemy hit. When they die within a short timeframe, nearby enemies take high damage.",
        sprite_pool.init_sprite_unchecked("/sprites/effects/overloaded", Duration::ZERO),
        "/audio/sounds/spells/overload_cast",
        GameAction::spawn(|_, pos, cmd| {

            cmd.push((
                pos,
                components::LifeDuration::new(Duration::from_secs(10)),
                components::Graphics::new(
                    "/sprites/spells/overload",
                    Duration::from_secs_f32(0.2),
                ),
                components::Velocity::new(0., -300.),
                components::Collision::new(12., 32., move |e1, e2| vec![
                            (e1, GameAction::Remove(RemoveSource::ProjectileCollision)),
                            (e1, GameAction::play_sound("/audio/sounds/spells/overload_hit")),
                            (e2, GameAction::TakeDamage { dmg: 10 }),
                            (e2, ActionEffect::on_death(
                                ActionEffectTarget::new()
                                    .with_enemies_only(true)
                                    .with_range(160.) ,
                                RemoveSource::HealthLoss,
                                GameAction::TakeDamage { dmg: 60 },
                            ).with_duration(Duration::from_secs(8)).into()),
                            (e2, ActionEffect::on_death(
                                ActionEffectTarget::new_only_self(),
                                RemoveSource::HealthLoss,
                                GameAction::play_sound("/audio/sounds/spells/overload_trigger"),
                            ).with_duration(Duration::from_secs(8)).into()),
                            (e2, GameAction::AddParticle(Particle::new("/sprites/effects/overloaded", Duration::from_secs_f32(0.4)).with_duration(Duration::from_secs(8)))),
                        ],),
            ));
        }),
        tiny_vec!([f32; MAX_SPELL_SLOTS] => 3., 5.))
}



pub(super) fn construct_lightning_ball(sprite_pool: &SpritePool) -> Spell {
    Spell::new(
        "Lightning Ball",
        "Launch a small lightning ball that passes through enemies then deploying for 10 seconds in the middle of the field. Both in flight and while deployed, the orb regularly zaps nearby enemies and significantly reduces their healing.",
        sprite_pool
            .init_sprite_unchecked("/sprites/spells/lightning_ball", Duration::ZERO),
            None,
        GameAction::spawn(|_, pos, cmd| {
            cmd.push((
                pos,
                components::LifeDuration::new(Duration::from_secs(8)),
                components::Graphics::new(
                    "/sprites/spells/lightning_ball",
                    Duration::from_secs_f32(0.2),
                ),
                components::Velocity::new(0., -200.),
                components::Actions::new()
                    // after 3 seconds of travel, stop.
                    .with_effect(ActionEffect::once(
                        ActionEffectTarget::new_only_self(),
                        ActionEffect::transform(
                            ActionEffectTarget::new_only_self(),
                            |act| {
                                match act {
                                    GameAction::Move { delta } => {*delta = ggez::glam::Vec2::ZERO},
                                    _ => {},
                                }
                            }
                        )
                    ).with_duration(Duration::from_secs(3)))
                    // regular zaps
                    .with_effect(
                        ActionEffect::repeat(
                            ActionEffectTarget::new().with_range(128.).with_enemies_only(true),
                            GameAction::TakeDamage { dmg: 15 },
                            Duration::from_secs_f32(0.4),
                        )
                    )
                    // zap fx
                    .with_effect(
                        ActionEffect::repeat(
                            ActionEffectTarget::new_only_self(),
                            vec![
                                GameAction::AddParticle(
                                    Particle::new("/sprites/spells/lightning_ball", Duration::from_secs_f32(0.1))
                                        .with_sprite_variant(1)
                                        .with_duration(Duration::from_secs_f32(0.15))
                                ),
                                GameAction::play_sound("/audio/sounds/spells/lball_hit"),
                            ],
                            Duration::from_secs_f32(0.4),
                        )
                    )
                    // healing prevention
                    .with_effect(ActionEffect::transform(
                        ActionEffectTarget::new().with_range(128.).with_enemies_only(true),
                        |act| {
                            match act {
                                GameAction::TakeHealing { heal } => {*heal /= 10},
                                _ => {}
                            }
                        }
                    ))
            ));
        }),
        tiny_vec!([f32; MAX_SPELL_SLOTS] => 8., 8., 16.),
    )
}