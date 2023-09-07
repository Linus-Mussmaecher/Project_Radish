use std::{f32::consts::PI, time::Duration, vec};

use legion::IntoQuery;
use mooeye::sprite::SpritePool;
use tinyvec::tiny_vec;

use crate::scenes::game_state::components::{self, actions::*, spell::MAX_SPELL_SLOTS};

use super::Spell;

pub(super) fn construct_gale_force(sprite_pool: &SpritePool) -> Spell {
    Spell::new(
        "Gale Force",
        "Create a gust of wind, pushing back enemies and dealing slight damage.",
        sprite_pool.init_sprite_unchecked("/sprites/spells/icons/gale_icon", Duration::ZERO),
        "/audio/sounds/spells/galeforce_cast",
        GameAction::spawn(|_, pos, cmd| {
            cmd.push((
                pos,
                components::LifeDuration::new(Duration::from_secs(3)),
                components::Graphics::new(
                    "/sprites/spells/galeforce",
                    Duration::from_secs_f32(0.2),
                ),
                components::Velocity::new(0., -300.),
                components::Collision::new(128., 16., false, |e1, e2| {
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
        tiny_vec!([f32; MAX_SPELL_SLOTS] => 2.5, 2.5, 5., 5., 8.),
    )
}

pub(super) fn construct_airburst(sprite_pool: &SpritePool) -> Spell {
    Spell::new(
        "Airburst",
        "Launch a ball of compressed air. Upon hitting an enemy, it deals area damage and pulls nearby enemies towards a point behind the target.",
        sprite_pool.init_sprite_unchecked("/sprites/spells/airburst", Duration::ZERO),
        "/audio/sounds/spells/airburst_cast",
        GameAction::spawn(|_, pos, cmd| {
            cmd.push((
                pos,
                components::LifeDuration::new(Duration::from_secs(4)),
                components::Graphics::new(
                    "/sprites/spells/airburst",
                    Duration::from_secs_f32(0.2),
                ),
                components::Velocity::new(0., -350.),
                components::Collision::new(32., 32., true, |e1, e2| {
                    vec![
                        (e1, GameAction::Remove(RemoveSource::ProjectileCollision)),
                        (e1, GameAction::play_sound("/audio/sounds/spells/airburst_hit")),
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
        tiny_vec!([f32; MAX_SPELL_SLOTS] => 8., 15.),
    )
}

pub(super) fn construct_blackhole(sprite_pool: &SpritePool) -> Spell {
    Spell::new(
        "Blackhole",
        "Launch a slow-moving ball of antimatter. When colliding with an enemy, it will spawn a blackhole that attracts enemies for 6 seconds, then damages and shortly silences close enemies.",
        sprite_pool.init_sprite_unchecked("/sprites/spells/blackhole", Duration::ZERO),
        "/audio/sounds/spells/blackhole_cast",
        GameAction::spawn(|_, pos, cmd| {
            cmd.push((
                pos,
                components::LifeDuration::new(Duration::from_secs(3)),
                components::Graphics::new(
                    "/sprites/spells/blackhole_mini",
                    Duration::from_secs_f32(0.2),
                ),
                components::Velocity::new(0., -180.),
                components::Collision::new(16., 16., true, |e1, e2| vec![
                            (e1, GameAction::Remove(RemoveSource::ProjectileCollision)),
                            (e1, GameAction::play_sound("/audio/sounds/spells/blackhole_hit")),
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
                                        .with_effect(ActionEffect::on_death(
                                            ActionEffectTarget::new_only_self(),
                                            RemoveSource::TimedOut,
                                            GameAction::play_sound("/audio/sounds/spells/blackhole_explosion"),
                                        )),
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
        "/audio/sounds/spells/mindwipe_cast",
        GameAction::spawn(|_, pos, cmd| {
            cmd.push((
                pos,
                components::LifeDuration::new(Duration::from_secs(10)),
                components::Graphics::new(
                    "/sprites/spells/mindwipe",
                    Duration::from_secs_f32(0.2),
                ),
                components::Velocity::new(0., -250.),
                components::Collision::new(32., 32., true, |e1, e2| vec![
                            (e1, GameAction::Remove(RemoveSource::ProjectileCollision)),
                            (e1, GameAction::play_sound("/audio/sounds/spells/mindwipe_hit")),
                            (e2, GameAction::TakeDamage { dmg: 42 }),
                            (e2, ActionEffect::once(ActionEffectTarget::new_only_self(), vec![
                                GameAction::TakeDamage { dmg: 42 },
                                GameAction::Silence(Duration::new(15, 0)),
                                GameAction::play_sound("/audio/sounds/spells/mindwipe_hit"),
                            ]).with_duration(Duration::new(2, 0)).into()),
                        ],),
            ));
        }),
        tiny_vec!([f32; MAX_SPELL_SLOTS] => 3., 3., 6.),
    )
}

pub(super) fn construct_arcane_missiles(sprite_pool: &SpritePool) -> Spell {
    Spell::new(
        "Arcane Missiles",
        "Infuse your self with arcane power. Every second for the next 10 seconds, launch an arcane missile towards a nearby enemy, dealing moderate damage.",
        sprite_pool.init_sprite_unchecked("/sprites/spells/icons/arcane_bolt_icon", Duration::ZERO),
        "/audio/sounds/spells/amissiles_cast",
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
                    if let Some(&target) = pos_list.get(rand::random::<usize>() % pos_list.len().min(4).max(1)){

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
                            components::Collision::new(32., 32., true, |e1, e2| vec![
                                        (e1, GameAction::Remove(RemoveSource::ProjectileCollision)),
                                        (e1, GameAction::play_sound("/audio/sounds/spells/amissiles_hit")),
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
        "/audio/sounds/spells/ablast_cast",
        GameAction::spawn(|_, pos, cmd|{
            cmd.push((
                pos,
                components::LifeDuration::new(Duration::from_secs(10)),
                components::Graphics::new(
                    "/sprites/spells/arcane_bolt",
                    Duration::from_secs_f32(0.2),
                ),
                components::Velocity::new(0., -360.),
                components::Collision::new(32., 32., true, |e1, e2| vec![
                            (e1, GameAction::Remove(RemoveSource::ProjectileCollision)),
                            (e1, GameAction::play_sound("/audio/sounds/spells/ablast_hit1")),
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
                                        components::Collision::new(8., 8., true, |e1, e2| vec![
                                                    (e1, GameAction::Remove(RemoveSource::ProjectileCollision)),
                                                    (e1, GameAction::play_sound("/audio/sounds/spells/ablast_hit2")),
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
