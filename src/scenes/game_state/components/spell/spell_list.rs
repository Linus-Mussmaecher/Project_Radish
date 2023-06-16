use std::{f32::consts::PI, time::Duration, vec};

use legion::{systems::CommandBuffer, Entity, IntoQuery};
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
        vec![
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
                            (e1, GameAction::play_sound("/audio/sounds/explosion")),
                        ]
                    }),
                ));
            }),
            GameAction::play_sound("/audio/sounds/fireball_cast"),
        ],
        tiny_vec!([f32; MAX_SPELL_SLOTS] => 2.5),
    )
}

pub(super) fn construct_ice_bomb(sprite_pool: &SpritePool) -> Spell {
    Spell::new(
        "Ice Bomb",
        "Launch a fast icy projectile that deals high damage on impact and drops an ice crystal that slows nearby enemies and deals area damage when exploding.",
        sprite_pool
            .init_sprite_unchecked("/sprites/spells/icebomb", Duration::ZERO),
        vec![GameAction::spawn(|_, pos, cmd| {
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
                            (e1, GameAction::play_sound("/audio/sounds/ice_hit")),
                            (e2, GameAction::TakeDamage { dmg: 25 }),
                            (e1, GameAction::spawn(spawn_icebomb)),
                        ],
                ),
            ));
        }),
        GameAction::play_sound("/audio/sounds/ice_cast")
        ],
        tiny_vec!([f32; MAX_SPELL_SLOTS] => 1.5, 5.),
    )
}

fn spawn_icebomb(_: Entity, pos: components::Position, cmd: &mut CommandBuffer) {
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
            )),
    ));
}

pub(super) fn construct_lightning_orb(sprite_pool: &SpritePool) -> Spell {
    Spell::new(
        "Lightning Ball",
        "Launch a ball of lightning that pierces through enemies and deals area damage on every contact.",
        sprite_pool
            .init_sprite_unchecked("/sprites/spells/electroorb", Duration::ZERO),
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
                            (e1, GameAction::play_sound("/audio/sounds/shock"))
                        ],),
            ));
        }),
        tiny_vec!([f32; MAX_SPELL_SLOTS] => 1.5, 1.5, 20.)
    )
}

pub(super) fn construct_conflagrate(sprite_pool: &SpritePool) -> Spell {
    Spell::new(
        "Conflagrate",
        "Burn the three nearest enemies for 8 seconds, dealing high damage over time",
        sprite_pool.init_sprite_unchecked("/sprites/spells/icons/conflagrate_icon", Duration::ZERO),
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

pub(super) fn construct_ice_lance(sprite_pool: &SpritePool) -> Spell {
    Spell::new(
        "Ice Lance",
        "Launch a volley of 3 quick-striking ice lances, each dealing damage to a single target and increasing their damage taken.",
        sprite_pool.init_sprite_unchecked("/sprites/spells/icons/icespike_icon", Duration::ZERO),
            ActionEffect::repeat(ActionEffectTarget::new_only_self(),
                GameAction::spawn(|_, pos, cmd|{
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
            Duration::from_secs_f32(0.2))
            .with_duration(Duration::from_secs_f32(0.7)),
        tiny_vec!([f32; MAX_SPELL_SLOTS] => 2., 2., 2.))
}

pub(super) fn construct_overload(sprite_pool: &SpritePool) -> Spell {
    Spell::new(
        "Overload",
        "Shoot out an electric spark that overloads the first enemy hit. When they die within a short timeframe, nearby enemies take high damage.",
        sprite_pool.init_sprite_unchecked("/sprites/effects/overloaded", Duration::ZERO),
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
                            (e2, GameAction::TakeDamage { dmg: 10 }),
                            (e2, ActionEffect::on_death(
                                ActionEffectTarget::new()
                                    .with_enemies_only(true)
                                    .with_range(160.) ,
                                RemoveSource::HealthLoss,
                                GameAction::TakeDamage { dmg: 60 } ,
                            ).with_duration(Duration::from_secs(8)).into()),
                            (e2, GameAction::AddParticle(Particle::new("/sprites/effects/overloaded", Duration::from_secs_f32(0.4)).with_duration(Duration::from_secs(8)))),
                        ],),
            ));
        }),
        tiny_vec!([f32; MAX_SPELL_SLOTS] => 3., 5.))
}

pub(super) fn construct_scorch(sprite_pool: &SpritePool) -> Spell {
    Spell::new(
        "Scorch",
        "Hurl a short ranged fireball, dealing low impact damage but igniting the area hit for 10 seconds, dealing damage over time to all enemies inside.",
        sprite_pool.init_sprite_unchecked("/sprites/spells/scorch", Duration::ZERO),
        GameAction::spawn(|_, pos, cmd| {
            cmd.push((
                pos,
                components::LifeDuration::new(Duration::from_secs(2)),
                components::Graphics::new(
                    "/sprites/spells/scorch",
                    Duration::from_secs_f32(0.2),
                ),
                components::Velocity::new(0., -200.),
                components::Collision::new(32., 32., |e1, e2| vec![
                            (e1, GameAction::Remove(RemoveSource::ProjectileCollision)),
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

pub(super) fn construct_shard(sprite_pool: &SpritePool) -> Spell {
    Spell::new(
        "Shard of Ice",
        "Throw a shard of ice dealing moderate damage and slowing. On hit, split into three smaller shards that deal less damage but slow more.",
        {
                let mut s = sprite_pool.init_sprite_unchecked("/sprites/spells/icebomb", Duration::ZERO);
                s.set_variant(1);
                s
            },
        GameAction::spawn(|_, pos, cmd|{
            cmd.push((
                pos,
                components::Velocity::new(0., -250.),
                components::LifeDuration::new(Duration::from_secs(10)),
                components::Graphics::new("/sprites/spells/icebomb", Duration::from_secs_f32(0.25)),
                components::Collision::new(32., 32., |e1, e2| vec![
                            (e1, GameAction::Remove(RemoveSource::ProjectileCollision)),
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
                                        ).with_sprite_variant(2),
                                        components::Collision::new(32., 32., |e1, e2| vec![
                                                (e1, GameAction::Remove(RemoveSource::ProjectileCollision)),
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

pub(super) fn construct_arcane_missiles(sprite_pool: &SpritePool) -> Spell {
    Spell::new(
        "Arcane Missiles",
        "Infuse your self with arcane power. Every second for the next 10 seconds, launch an arcane missile towards the nearest enemy, dealing moderate damage.",
        sprite_pool.init_sprite_unchecked("/sprites/spells/icons/arcane_bolt_icon", Duration::ZERO),
        ActionEffect::repeat(
            ActionEffectTarget::new_only_self(),
            GameAction::spawn(|_, pos_src, cmd|{
                // execute the following every seconds:
                cmd.exec_mut(move |world, res|{
                    // get an iterator overall enemies:
                    let mut query = <(&components::Enemy, &components::Position)>::query();
                    let iter = query.iter(world);

                    // remember all enemy positions
                    let mut pos_list = Vec::with_capacity(iter.size_hint().0);
                    for (_, pos_tar) in iter {
                        // only save copies of vectors so world needs not be borrowed for too long
                        pos_list.push(*pos_tar)
                    }

                    // sort the vector by distance to source
                    pos_list.sort_by(|a,b| a.distance(pos_src).total_cmp(&b.distance(pos_src)) );

                    // get closest vector
                    if let Some(&target) = pos_list.first(){

                        // get sprite pool
                        let sp = res.get::<mooeye::sprite::SpritePool>().expect("Could not find sprite pool when spawning arcane missile.");

                        // push the missile
                        world.push((
                            pos_src,
                            components::LifeDuration::new(Duration::from_secs(10)),
                            components::Graphics::from(sp.init_sprite_unchecked(
                                "/sprites/spells/arcane_bolt_mini",
                                Duration::from_secs_f32(0.2),
                            )),
                            components::Velocity::from((target - pos_src).clamp_length(240., 240.)),
                            components::Collision::new(32., 32., |e1, e2| vec![
                                        (e1, GameAction::Remove(RemoveSource::ProjectileCollision)),
                                        (e2, GameAction::TakeDamage { dmg: 25 }),
                                    ],),
                        ));
                    }
                });
            }),
            Duration::from_secs_f32(0.4)
        ).with_duration(Duration::from_secs(10)),
        tiny_vec!([f32; MAX_SPELL_SLOTS] => 2., 2., 2., 2., 10., 10.))
}

pub(super) fn construct_arcane_blast(sprite_pool: &SpritePool) -> Spell {
    Spell::new(
        "Arcane Blast",
        "Launch an orb of arcane energy dealing medium damage. On hitting an enemy, 8 smaller orbs are created centered on the target hit and striking inwards for the same amount of damage.",
        sprite_pool.init_sprite_unchecked("/sprites/spells/arcane_bolt_mini", Duration::ZERO),
        GameAction::spawn(|_, pos, cmd|{
            cmd.push((
                pos,
                components::LifeDuration::new(Duration::from_secs(10)),
                components::Graphics::new(
                    "/sprites/spells/arcane_bolt",
                    Duration::from_secs_f32(0.2),
                ),
                components::Velocity::new(0., -360.),
                components::Collision::new(32., 32., |e1, e2| vec![
                            (e1, GameAction::Remove(RemoveSource::ProjectileCollision)),
                            (e2, GameAction::TakeDamage { dmg: 30 }),
                            (e2, GameAction::spawn(|_, pos,cmd|{
                                for i in 0..8{
                                    let rel = ggez::glam::Vec2::new(64. * (PI/4. * i as f32).cos(), 64. * (PI/4. * i as f32).sin());
                                    cmd.push((
                                        pos + rel,
                                        components::LifeDuration::new(Duration::from_secs(10)),
                                        components::Graphics::new(
                                            "/sprites/spells/arcane_bolt_mini",
                                            Duration::from_secs_f32(0.2),
                                        ),
                                        components::Velocity::from(rel.clamp_length(240., 240.) * -1.),
                                        components::Collision::new(8., 8., |e1, e2| vec![
                                                    (e1, GameAction::Remove(RemoveSource::ProjectileCollision)),
                                                    (e2, GameAction::TakeDamage { dmg: 30 }),
                                                ],),
                                    ));
                                }
                            })),
                        ],),
            ));
        }),
        tiny_vec!([f32; MAX_SPELL_SLOTS] => 5.,10., 15.))
}

pub(super) fn construct_blackhole(sprite_pool: &SpritePool) -> Spell {
    Spell::new(
        "Blackhole",
        "Launch a slow-moving ball of antimatter. When colliding with an enemy, it will spawn a blackhole that attracts enemies for 6 seconds, then damages and shortly silences close enemies.",
        sprite_pool.init_sprite_unchecked("/sprites/spells/blackhole", Duration::ZERO),
        GameAction::spawn(|_, pos, cmd| {
            cmd.push((
                pos,
                components::LifeDuration::new(Duration::from_secs(3)),
                components::Graphics::new(
                    "/sprites/spells/blackhole_mini",
                    Duration::from_secs_f32(0.2),
                ),
                components::Velocity::new(0., -180.),
                components::Collision::new(16., 16., |e1, e2| vec![
                            (e1, GameAction::Remove(RemoveSource::ProjectileCollision)),
                            (e2, GameAction::TakeDamage { dmg: 80 }),
                            (e1, GameAction::spawn(|_, pos, cmd|{
                                cmd.push((
                                    pos + ggez::glam::Vec2::new(0., -30.),
                                    components::LifeDuration::new(Duration::from_secs(6)),
                                    components::Graphics::new(
                                        "/sprites/spells/blackhole",
                                        Duration::from_secs_f32(0.1),
                                    ),
                                    components::Actions::new()
                                        .with_effect(
                                            ActionEffect::repeat(
                                                ActionEffectTarget::new_only_self(),
                                                GameAction::spawn(|_, pos_src, cmd|{
                                                    // execute the following every seconds:
                                                    cmd.exec_mut(move |world, _|{
                                                        // iterator over all (close) enemies
                                                        for (_, pos_tar, act_tar) in <(&components::Velocity, &components::Position, &mut Actions)>::query()
                                                            .iter_mut(world)
                                                            .filter(|(_, pos, _)| pos.distance(pos_src) < 175.)
                                                        {
                                                            act_tar.push(GameAction::Move { delta: (pos_src - *pos_tar).clamp_length_max(1.) })
                                                        }
                                                    });
                                                }),
                                                Duration::from_secs_f32(0.02)
                                            ).with_duration(Duration::new(6, 0))
                                        )
                                        .with_effect(ActionEffect::on_death(
                                            ActionEffectTarget::new().with_range(64.).with_enemies_only(true),
                                            RemoveSource::TimedOut,
                                            vec![
                                                GameAction::TakeDamage { dmg: 30 },
                                                GameAction::Silence(Duration::new(1, 0))
                                            ],
                                        ))
                                ));
                            }),)
                        ],),
            ));
        }),
        tiny_vec!([f32; MAX_SPELL_SLOTS] => 6., 6., 6., 6.),
    )
}

pub(super) fn construct_mind_wipe(sprite_pool: &SpritePool) -> Spell {
    Spell::new(
        "Mind wipe",
        "Launch a bolt of dark energy that deals a medium amount of damage to the first enemy hit. After a short delay, deal the same damage again and silence the target for 15 seconds.",
        sprite_pool.init_sprite_unchecked("/sprites/spells/icons/mindwipe_icon", Duration::ZERO),
        GameAction::spawn(|_, pos, cmd| {
            cmd.push((
                pos,
                components::LifeDuration::new(Duration::from_secs(10)),
                components::Graphics::new(
                    "/sprites/spells/mindwipe",
                    Duration::from_secs_f32(0.2),
                ),
                components::Velocity::new(0., -250.),
                components::Collision::new(32., 32., |e1, e2| vec![
                            (e1, GameAction::Remove(RemoveSource::ProjectileCollision)),
                            (e2, GameAction::TakeDamage { dmg: 42 }),
                            (e2, ActionEffect::once(ActionEffectTarget::new_only_self(), vec![
                                GameAction::TakeDamage { dmg: 42 },
                                GameAction::Silence(Duration::new(15, 0)),
                            ]).with_duration(Duration::new(2, 0)).into()),
                        ],),
            ));
        }),
        tiny_vec!([f32; MAX_SPELL_SLOTS] => 3., 3., 6.),
    )
}

pub(super) fn construct_mortar(sprite_pool: &SpritePool) -> Spell {
    Spell::new(
        "Fiery mortar", 
        "Launch five mortar shells that pass over enemies and impact the middle of the battlefield, dealing area damage.", 
        sprite_pool.init_sprite_unchecked("/sprites/spells/icons/mortar_icon", Duration::ZERO),
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
                            GameAction::spawn(|_,pos,cmd|{
                                cmd.push((
                                    pos,
                                    components::LifeDuration::from(Duration::from_secs_f32(0.64)),
                                    components::Graphics::new("/sprites/effects/explosion_small", Duration::ZERO),
                                ));
                            }),
                        )),
                ));
            }
        }),
        tiny_vec!([f32; MAX_SPELL_SLOTS] => 2., 2., 2., 2., 2.)
    )
}

pub(super) fn construct_lightning_ball(sprite_pool: &SpritePool) -> Spell {
    Spell::new(
        "Lightning Ball",
        "Launch a small lightning ball that passes through enemies then deploying for 10 seconds in the middle of the field. Both in flight and while deployed, the orb regularly zaps nearby enemies and significantly reduces their healing.",
        sprite_pool
            .init_sprite_unchecked("/sprites/spells/lightning_ball", Duration::ZERO),
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
                            GameAction::AddParticle(
                                Particle::new("/sprites/spells/lightning_ball", Duration::from_secs_f32(0.1))
                                    .with_sprite_variant(1)
                                    .with_duration(Duration::from_secs_f32(0.15))
                            ),
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

pub(super) fn construct_gale_force(sprite_pool: &SpritePool) -> Spell {
    Spell::new(
        "Gale Force",
        "Create a gust of wind, pushing back enemies and dealing slight damage.",
        sprite_pool.init_sprite_unchecked("/sprites/spells/fireball", Duration::ZERO),
        vec![
            GameAction::spawn(|_, pos, cmd| {
                cmd.push((
                    pos,
                    components::LifeDuration::new(Duration::from_secs(3)),
                    components::Graphics::new(
                        "/sprites/spells/fireball",
                        Duration::from_secs_f32(0.2),
                    ),
                    components::Velocity::new(0., -300.),
                    components::Collision::new(128., 16., |e1, e2| {
                        vec![
                            (e1, GameAction::AddImmunity { other: e2 }),
                            (e2, GameAction::TakeDamage { dmg: 10 }),
                            (
                                e2,
                                GameAction::ApplyEffect(Box::new(
                                    ActionEffect::repeat(
                                        ActionEffectTarget::new_only_self(),
                                        GameAction::Move {
                                            delta: ggez::glam::Vec2::new(0., -7.),
                                        },
                                        Duration::from_secs_f32(0.02),
                                    )
                                    .with_duration(Duration::from_secs_f32(0.2)),
                                )),
                            ),
                        ]
                    }),
                ));
            }),
            GameAction::play_sound("/audio/sounds/fireball_cast"),
        ],
        tiny_vec!([f32; MAX_SPELL_SLOTS] => 2.5, 2.5, 5., 5., 8.),
    )
}

pub(super) fn construct_airburst(sprite_pool: &SpritePool) -> Spell {
    Spell::new(
        "Airburst",
        "Launch a ball of compressed air. Upon hitting an enemy, it deals area damage and pulls nearby enemies towards a point behind the target.",
        sprite_pool.init_sprite_unchecked("/sprites/spells/fireball", Duration::ZERO),
        vec![
            GameAction::spawn(|_, pos, cmd| {
                cmd.push((
                    pos,
                    components::LifeDuration::new(Duration::from_secs(4)),
                    components::Graphics::new(
                        "/sprites/spells/fireball",
                        Duration::from_secs_f32(0.2),
                    ),
                    components::Velocity::new(0., -350.),
                    components::Collision::new(32., 32., |e1, e2| {
                        vec![
                            (e1, GameAction::Remove(RemoveSource::ProjectileCollision)),
                            (e2, GameAction::TakeDamage { dmg: 45 }),
                            (e2, GameAction::spawn(|_, pos, cmd|{
                                cmd.push((
                                    pos + ggez::glam::Vec2::new(0., -64.),
                                    components::LifeDuration::new(Duration::from_secs_f32(0.3)),
                                    components::Actions::new()
                                        .with_effect(
                                            ActionEffect::repeat(
                                                ActionEffectTarget::new_only_self(),
                                                GameAction::spawn(|_, pos_src, cmd|{
                                                    // execute the following every seconds:
                                                    cmd.exec_mut(move |world, _|{
                                                        // iterator over all (close) enemies
                                                        for (_, pos_tar, act_tar) in <(&components::Velocity, &components::Position, &mut Actions)>::query()
                                                            .iter_mut(world)
                                                            .filter(|(_, pos, _)| pos.distance(pos_src) < 175.)
                                                        {
                                                            act_tar.push(GameAction::Move { delta: (pos_src - *pos_tar).clamp_length_max(3.) })
                                                        }
                                                    });
                                                }),
                                                Duration::from_secs_f32(0.02)
                                            ).with_duration(Duration::from_secs_f32(0.7))
                                        )
                                ));
                            }),)
                        ]
                    }),
                ));
            }),
            GameAction::play_sound("/audio/sounds/fireball_cast"),
        ],
        tiny_vec!([f32; MAX_SPELL_SLOTS] => 8., 15.),
    )
}
