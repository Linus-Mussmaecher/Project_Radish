use std::{time::Duration, vec};

use mooeye::sprite::SpritePool;
use tinyvec::tiny_vec;

use crate::scenes::game_state::components::{
    self, actions::*, graphics::Particle, spell::MAX_SPELL_SLOTS,
};

use super::Spell;


pub(super) fn construct_fireball(sprite_pool: &SpritePool) -> Spell {
    Spell::new(
        "Fireball",
        "Hurl a ball of fire, dealing a small amount of damage.",
        sprite_pool.init_sprite_unchecked("/sprites/spells/fireball", Duration::ZERO),
        "/audio/sounds/spells/fireball_cast",
        GameAction::spawn(|_, pos, cmd| {
            cmd.push((
                pos,
                components::LifeDuration::new(Duration::from_secs(10)),
                components::Graphics::new(
                    "/sprites/spells/fireball",
                    Duration::from_secs_f32(0.2),
                ),
                components::Velocity::new(0., -250.),
                components::Collision::new(32., 32., |e1, e2| {
                    vec![
                        (e1, GameAction::Remove(RemoveSource::ProjectileCollision)),
                        (e2, GameAction::TakeDamage { dmg: 20 }),
                        (e1, GameAction::play_sound("/audio/sounds/spells/fireball_hit")),
                    ]
                }),
            ));
        }),
    tiny_vec!([f32; MAX_SPELL_SLOTS] => 2.5),
    )
}

pub(super) fn construct_scorch(sprite_pool: &SpritePool) -> Spell {
    Spell::new(
        "Scorch",
        "Hurl a short ranged fireball, dealing low impact damage but igniting the area hit for 10 seconds, dealing damage over time to all enemies inside.",
        sprite_pool.init_sprite_unchecked("/sprites/spells/scorch", Duration::ZERO),
        "/audio/sounds/spells/scorch_cast",
        GameAction::spawn(|_, pos, cmd| {
            cmd.push((
                pos,
                components::LifeDuration::new(Duration::from_secs(3)),
                components::Graphics::new(
                    "/sprites/spells/scorch",
                    Duration::from_secs_f32(0.2),
                ),
                components::Velocity::new(0., -200.),
                components::Collision::new(32., 32., |e1, e2| vec![
                            (e1, GameAction::Remove(RemoveSource::ProjectileCollision)),
                            (e1, GameAction::play_sound("/audio/sounds/spells/scorch_hit")),
                            (e1, GameAction::spawn(|_, pos, cmd|{
                                cmd.push((
                                    pos,
                                    components::LifeDuration::from(Duration::from_secs(10)),
                                    components::Graphics::new(
                                        "/sprites/spells/burning_ground",
                                        Duration::from_secs_f32(0.2),
                                    ),
                                    components::Actions::new()
                                    .with_effect(
                                        ActionEffect::repeat(ActionEffectTarget::new().with_enemies_only(true).with_range(128.),
                                        GameAction::TakeDamage { dmg: 5 }, Duration::from_secs_f32(0.5))
                                    )
                                ));
                            })),
                            (e2, GameAction::TakeDamage { dmg: 20 }),
                        ],),
            ));
        }),
        tiny_vec!([f32; MAX_SPELL_SLOTS] => 2., 5.,10.,))
}


pub(super) fn construct_mortar(sprite_pool: &SpritePool) -> Spell {
    Spell::new(
        "Fiery mortar", 
        "Launch five mortar shells that pass over enemies and impact the middle of the battlefield, dealing area damage.", 
        sprite_pool.init_sprite_unchecked("/sprites/spells/icons/mortar_icon", Duration::ZERO),
        "/audio/sounds/spells/mortar_cast",
        GameAction::spawn(|_, pos, cmd| {
            for _ in 0..5{
                cmd.push((
                    pos,
                    components::LifeDuration::new(Duration::from_secs_f32(1.9)),
                    components::Graphics::new("/sprites/spells/mortar", Duration::from_secs_f32(0.25)),
                    components::Velocity::new(rand::random::<f32>() * 96. - 48., -270. + rand::random::<f32>() * 96.),
                    components::Actions::new()
                        .with_effect(ActionEffect::on_death(
                            ActionEffectTarget::new().with_range(64.).with_enemies_only(true),
                            RemoveSource::TimedOut,
                            GameAction::TakeDamage { dmg: 45 },
                        ))
                        .with_effect(ActionEffect::on_death(
                            ActionEffectTarget::new_only_self(),
                            RemoveSource::TimedOut,
                            vec![
                                GameAction::spawn(|_,pos,cmd|{
                                    cmd.push((
                                        pos,
                                        components::LifeDuration::from(Duration::from_secs_f32(0.64)),
                                        components::Graphics::new("/sprites/effects/explosion_small", Duration::ZERO),
                                    ));
                                }),
                                GameAction::play_sound("/audio/sounds/spells/mortar_hit"),
                            ],
                        )),
                ));
            }
        }),
        tiny_vec!([f32; MAX_SPELL_SLOTS] => 2., 2., 2., 2., 2.)
    )
}

pub(super) fn construct_flameorb(sprite_pool: &SpritePool) -> Spell {
    Spell::new(
        "Flame Orb",
        "Hurl an orb of flame, dealing a not-quite-as-small amount of damage and igniting enemies near the target.",
        sprite_pool.init_sprite_unchecked("/sprites/spells/flameorb", Duration::ZERO),
        "/audio/sounds/fireball_cast",
        GameAction::spawn(|_, pos, cmd| {
            cmd.push((
                pos,
                components::LifeDuration::new(Duration::from_secs(10)),
                components::Graphics::new(
                    "/sprites/spells/flameorb",
                    Duration::from_secs_f32(0.2),
                ),
                components::Velocity::new(0., -250.),
                components::Collision::new(24., 24., |e1, e2| {
                    vec![
                        (e1, GameAction::Remove(RemoveSource::ProjectileCollision)),
                        (e1, GameAction::play_sound("/audio/sounds/spells/flameorb_hit")),
                        (e2, GameAction::TakeDamage { dmg: 20 }),
                        (e2, GameAction::ApplyEffect(Box::new(ActionEffect::once(
                            ActionEffectTarget::new()
                            .with_affect_self(true)
                            .with_enemies_only(true)
                            .with_range(128.),
                            vec![
                                ActionEffect::repeat(
                                    ActionEffectTarget::new_only_self(),
                                    GameAction::TakeDamage { dmg: 4 },
                                    Duration::from_secs_f32(0.5),
                                )
                                .with_duration(Duration::from_secs(4))
                                .into(),
                                GameAction::AddParticle(
                                    Particle::new("/sprites/spells/burning", Duration::from_secs_f32(0.25))
                                        .with_duration(Duration::from_secs(4)),
                                ),
                            ]
                        )))),
                        (e1, GameAction::play_sound("/audio/sounds/explosion")),
                    ]
                }),
            ));
        }),
        tiny_vec!([f32; MAX_SPELL_SLOTS] => 2.5, 5.),
    )
}

pub(super) fn construct_conflagrate(sprite_pool: &SpritePool) -> Spell {
    Spell::new(
        "Conflagrate",
        "Burn the three nearest enemies for 8 seconds, dealing high damage over time",
        sprite_pool.init_sprite_unchecked("/sprites/spells/icons/conflagrate_icon", Duration::ZERO),
        "/audio/sounds/spells/conflagrate_cast",
        ActionEffect::once(
            ActionEffectTarget::new()
                .with_enemies_only(true)
                .with_limit(3),
            vec![
                ActionEffect::repeat(
                    ActionEffectTarget::new_only_self(),
                    GameAction::TakeDamage { dmg: 8 },
                    Duration::from_secs_f32(0.5),
                )
                .with_duration(Duration::from_secs(10))
                .into(),
                GameAction::AddParticle(
                    Particle::new("/sprites/spells/burning", Duration::from_secs_f32(0.25))
                        .with_duration(Duration::from_secs(8)),
                ),
            ],
        ),
        tiny_vec!([f32; MAX_SPELL_SLOTS] => 4., 4., 10., 10.),
    )
}

pub(super) fn construct_phoenix(sprite_pool: &SpritePool) -> Spell {
    Spell::new(
        "Summon Phoenix",
        "Summons a phoenix in front of you for 20 seconds. It regularly flaps its wings, dealing damage to nearby enemies and launching fireballs.",
        sprite_pool.init_sprite_unchecked("/sprites/spells/icons/phoenix_icon", Duration::ZERO),
        "/audio/sounds/spells/phoenix_cast",
        GameAction::spawn(|_, pos, cmd| {
            cmd.push((
                pos + ggez::glam::Vec2::new(0., -64.),
                components::LifeDuration::new(Duration::from_secs(20)),
                components::Graphics::new(
                    "/sprites/spells/phoenix",
                    Duration::from_secs_f32(0.2),
                ),
                components::actions::Actions::new()
                .with_effect(ActionEffect::repeat(
                    ActionEffectTarget::new_only_self(), 
                    vec![
                        GameAction::spawn(|_, pos, cmd| {
                            cmd.push((
                                pos,
                                components::LifeDuration::new(Duration::from_secs(10)),
                                components::Graphics::new(
                                    "/sprites/spells/fireball",
                                    Duration::from_secs_f32(0.3),
                                ),
                                components::Velocity::new(0., -250.),
                                components::Collision::new(32., 32., |e1, e2| {
                                    vec![
                                        (e1, GameAction::Remove(RemoveSource::ProjectileCollision)),
                                        (e2, GameAction::TakeDamage { dmg: 20 }),
                                        (e1, GameAction::play_sound("/audio/sounds/spells/fireball_hit")),
                                    ]
                                }),
                            ));
                        }),
                        GameAction::play_sound("/audio/sounds/spells/fireball_cast"),
                    ],
                    Duration::new(1,0),
                ))
                .with_effect(ActionEffect::repeat(
                    ActionEffectTarget::new().with_affect_self(false).with_range(96.).with_enemies_only(true),
                    GameAction::TakeDamage { dmg: 15 },
                    Duration::new(1,0),
                )),
                
            ));
        }),
        tiny_vec!([f32; MAX_SPELL_SLOTS] => 5., 15., 25.),
    )
}