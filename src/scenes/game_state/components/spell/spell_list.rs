use std::{time::Duration, vec};

use legion::{systems::CommandBuffer, Entity};
use mooeye::sprite::SpritePool;
use tinyvec::tiny_vec;

use crate::scenes::game_state::{
    components::{self, actions::*, graphics::Particle, spell::MAX_SPELL_SLOTS},
    game_message::MessageSet,
};

use super::Spell;

pub fn construct_fireball(sprite_pool: &SpritePool) -> Spell {
    Spell::new(
        "Fireball",
        "Hurl a ball of fire, dealing a small amount of damage.",
        sprite_pool.init_sprite_unchecked("/sprites/spells/fireball", Duration::ZERO),
        GameAction::spawn(|_, pos, sp, cmd| {
            cmd.push((
                pos,
                components::LifeDuration::new(Duration::from_secs(10)),
                components::Graphics::from(sp.init_sprite_unchecked(
                    "/sprites/spells/fireball",
                    Duration::from_secs_f32(0.2),
                )),
                components::Velocity::new(0., -250.),
                components::Collision::new(32., 32., |e1, e2| {
                    (
                        vec![
                            (e1, GameAction::Remove(RemoveSource::Other)),
                            (e2, GameAction::TakeDamage { dmg: 20 }),
                        ],
                        MessageSet::new(),
                    )
                }),
            ));
        }),
        tiny_vec!([f32; MAX_SPELL_SLOTS] => 1.5),
    )
}

pub fn construct_ice_bomb(sprite_pool: &SpritePool) -> Spell {
    Spell::new(
        "Ice Bomb",
        "Launch a fast icy projectile that deals high damage on impact and drops an ice crystal that slows nearby enemies and deals area damage when exploding.",
        sprite_pool
            .init_sprite_unchecked("/sprites/spells/icebomb", Duration::ZERO),
        GameAction::spawn(|_, pos, sp, cmd| {
            cmd.push((
                pos,
                components::LifeDuration::new(Duration::from_secs(10)),
                components::Graphics::from(sp.init_sprite_unchecked(
                    "/sprites/spells/icebomb",
                    Duration::from_secs_f32(0.2),
                )),
                components::Velocity::new(0., -520.),
                components::Collision::new(32., 32., |e1, e2| {
                    (
                        vec![
                            (e1, GameAction::Remove(RemoveSource::Other)),
                            (e2, GameAction::TakeDamage { dmg: 25 }),
                            (e1, GameAction::spawn(spawn_icebomb)),
                        ],
                        MessageSet::new(),
                    )
                }),
            ));
        }),
        tiny_vec!([f32; MAX_SPELL_SLOTS] => 1.5, 5.),
    )
}

fn spawn_icebomb(
    _: Entity,
    pos: components::Position,
    sprite_pool: &SpritePool,
    cmd: &mut CommandBuffer,
) {
    cmd.push((
        pos,
        components::LifeDuration::new(Duration::from_secs(5)),
        {
            let mut sprite = sprite_pool
                .init_sprite_unchecked("/sprites/spells/icebomb", Duration::from_secs_f32(0.25));
            sprite.set_variant(1);
            components::Graphics::from(sprite)
        },
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
            )),
    ));
}

pub fn construct_lightning_orb(sprite_pool: &SpritePool) -> Spell {
    Spell::new(
        "Lightning Ball",
        "Launch a ball of lightning that pierces through enemies and deals area damage on every contact.",
        sprite_pool
            .init_sprite_unchecked("/sprites/spells/electroorb", Duration::ZERO),
        GameAction::spawn(|_, pos, sp, cmd| {
            cmd.push((
                pos,
                components::LifeDuration::new(Duration::from_secs(10)),
                components::Graphics::from(sp.init_sprite_unchecked(
                    "/sprites/spells/electroorb",
                    Duration::from_secs_f32(0.2),
                )),
                components::Velocity::new(0., -180.),
                components::Collision::new(32., 32., |e1, e2| {
                    (
                        vec![
                            (e1, GameAction::AddImmunity { other: e2 }),
                            (
                                e2,
                                ActionEffect::once(
                                    ActionEffectTarget::new()
                                        .with_enemies_only(true)
                                        .with_affect_self(true)
                                        .with_range(128.),
                                    GameAction::TakeDamage { dmg: 13 },
                                )
                                .into(),
                            ),
                        ],
                        MessageSet::new(),
                    )
                }),
            ));
        }),
        tiny_vec!([f32; MAX_SPELL_SLOTS] => 1.5, 1.5, 20.)   
    )
}

pub fn construct_conflagrate(sprite_pool: &SpritePool) -> Spell {
    Spell::new(
        "Conflagrate",
        "Burn the six nearest enemies for 5 seconds, dealing 30 damage per second.",
        sprite_pool.init_sprite_unchecked("/sprites/spells/fireball", Duration::ZERO),
        ActionEffect::once(
            ActionEffectTarget::new()
                .with_enemies_only(true)
                .with_limit(6),
            vec![
                ActionEffect::repeat(
                    ActionEffectTarget::new_only_self(),
                    GameAction::TakeDamage { dmg: 15 },
                    Duration::from_secs_f32(0.7),
                )
                .with_duration(Duration::from_secs(5))
                .into(),
                GameAction::AddParticle(
                    Particle::new(sprite_pool.init_sprite_unchecked(
                        "/sprites/spells/burning",
                        Duration::from_secs_f32(0.25),
                    ))
                    .with_duration(Duration::from_secs(5)),
                ),
            ],
        ),
        tiny_vec!([f32; MAX_SPELL_SLOTS] => 4., 4., 10., 10.),
    )
}

pub fn construct_ice_lance(sprite_pool: &SpritePool) -> Spell {
    Spell::new(
        "Ice Lance",
        "Launch a volley of 3 quick-striking ice lances, each dealing damage to a single target and increasing their damage taken.",
        sprite_pool.init_sprite_unchecked("/sprites/spells/icebomb", Duration::ZERO),
            ActionEffect::repeat(ActionEffectTarget::new_only_self(),
                GameAction::spawn(|_, pos, sp, cmd|{
                    cmd.push(
                        (pos,
                components::LifeDuration::new(Duration::from_secs(8)),
                components::Graphics::from(sp.init_sprite_unchecked(
                    "/sprites/spells/icebomb",
                    Duration::from_secs_f32(0.2),
                )),
                components::Velocity::new(0., -450.),
                components::Collision::new(32., 32., |e1, e2| {
                    (
                        vec![
                            (e1, GameAction::AddImmunity { other: e2 }),
                            (e1, GameAction::Remove(RemoveSource::Other)),
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
                            )
                        ],
                        MessageSet::new(),
                    )
                }),
                    ));
                }),
            Duration::from_secs_f32(0.2))
            .with_duration(Duration::from_secs_f32(0.7)),
        tiny_vec!([f32; MAX_SPELL_SLOTS] => 2., 2., 2.))
}

pub fn construct_overload(sprite_pool: &SpritePool) -> Spell {
    Spell::new(
        "Overload",
        "Shoot out an electric spark that overloads the first two enemies hit. When they die within a short timeframe, nearby enemies take high damage.",
        sprite_pool.init_sprite_unchecked("/sprites/spells/electroorb", Duration::ZERO),
        GameAction::None,
        tiny_vec!([f32; MAX_SPELL_SLOTS] => 3., 5.))
}

pub fn construct_scorch(sprite_pool: &SpritePool) -> Spell {
    Spell::new(
        "Scorch",
        "Hurl a short ranged fireball, dealing low impact damage but igniting the area hit for 10 seconds, dealing damage over time to all enemies inside.",
        sprite_pool.init_sprite_unchecked("/sprites/spells/fireball", Duration::ZERO),
        GameAction::None,
        tiny_vec!([f32; MAX_SPELL_SLOTS] => 2., 5.,10.,))
}

pub fn construct_shard(sprite_pool: &SpritePool) -> Spell {
    Spell::new(
        "Shard of Ice",
        "Throw a shard of ice dealing moderate damage and slowing. On hit, split into three smaller shards that deal less damage but slow more.",
        sprite_pool.init_sprite_unchecked("/sprites/spells/icebomb", Duration::ZERO),
        GameAction::None,
        tiny_vec!([f32; MAX_SPELL_SLOTS] => 1.5))
}

pub fn construct_arcane_missiles(sprite_pool: &SpritePool) -> Spell {
    Spell::new(
        "Arcane Missiles",
        "Infuse your self with arcane power. Every second for the next 10 seconds, launch an arcane missile towards the nearest enemy, dealing moderate damage.",
        sprite_pool.init_sprite_unchecked("/sprites/spells/electroorb", Duration::ZERO),
        GameAction::None,
        tiny_vec!([f32; MAX_SPELL_SLOTS] => 0.1, 0.1, 0.1, 5., 5., 10.))
}