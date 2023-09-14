use std::{f32::consts::PI, time::Duration, vec};

use good_web_game::{cgmath::MetricSpace, graphics::Vector2};
use legion::IntoQuery;
use mooeye::sprite::SpritePool;
use tinyvec::tiny_vec;

use crate::scenes::game_state::components::{self, actions::*, spell::MAX_SPELL_SLOTS};

use super::Spell;

pub(super) fn construct_gale_force(
    sprite_pool: &mut SpritePool,
    ctx: &mut good_web_game::Context,
    gfx_ctx: &mut good_web_game::event::GraphicsContext,
) -> Spell {
    Spell::new(
        "Gale Force",
        "Create a gust of wind, pushing back enemies and dealing slight damage.",
        sprite_pool.init_sprite_fmt_unchecked(
            "./sprites/spells/icons/gale_icon_8_8.png",
            ctx,
            gfx_ctx,
            Duration::ZERO,
        ),
        "./audio/sounds/spells/galeforce_cast.wav",
        GameAction::spawn(|_, pos, cmd| {
            cmd.push((
                pos,
                components::LifeDuration::new(Duration::from_secs(3)),
                components::Graphics::new(
                    "./sprites/spells/galeforce_32_4.png",
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
                                        delta: good_web_game::graphics::Vector2::new(0., -7.),
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

pub(super) fn construct_airburst(
    sprite_pool: &mut SpritePool,
    ctx: &mut good_web_game::Context,
    gfx_ctx: &mut good_web_game::event::GraphicsContext,
) -> Spell {
    Spell::new(
        "Airburst",
        "Launch a ball of compressed air. Upon hitting an enemy, it deals area damage and pulls nearby enemies towards a point behind the target.",
        sprite_pool.init_sprite_fmt_unchecked("./sprites/spells/airburst_8_8.png", ctx, gfx_ctx, Duration::ZERO),
        "./audio/sounds/spells/airburst_cast.wav",
        GameAction::spawn(|_, pos, cmd| {
            cmd.push((
                pos,
                components::LifeDuration::new(Duration::from_secs(4)),
                components::Graphics::new(
                    "./sprites/spells/airburst_8_8.png",
                    Duration::from_secs_f32(0.2),
                ),
                components::Velocity::new(0., -350.),
                components::Collision::new(32., 32., true, |e1, e2| {
                    vec![
                        (e1, GameAction::Remove(RemoveSource::ProjectileCollision)),
                        (e1, GameAction::play_sound("./audio/sounds/spells/airburst_hit.wav")),
                        (e2, GameAction::TakeDamage { dmg: 45 }),
                        (e2, GameAction::spawn(|_, pos, cmd|{
                            cmd.push((
                                pos + good_web_game::graphics::Vector2::new(0., -64.),
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
                                                        .filter(|(_, pos, _)| pos.distance2(pos_src) < 175.)
                                                    {
                                                        act_tar.push(GameAction::Move { delta: (pos_src - *pos_tar) * (3. / pos_src.distance2(*pos_tar).max(0.1)).min(1.) })
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

pub(super) fn construct_blackhole(
    sprite_pool: &mut SpritePool,
    ctx: &mut good_web_game::Context,
    gfx_ctx: &mut good_web_game::event::GraphicsContext,
) -> Spell {
    Spell::new(
        "Blackhole",
        "Launch a slow-moving ball of antimatter. When colliding with an enemy, it will spawn a blackhole that attracts enemies for 6 seconds, then damages and shortly silences close enemies.",
        sprite_pool.init_sprite_fmt_unchecked("./sprites/spells/blackhole_8_8.png", ctx, gfx_ctx, Duration::ZERO),
        "./audio/sounds/spells/blackhole_cast.wav",
        GameAction::spawn(|_, pos, cmd| {
            cmd.push((
                pos,
                components::LifeDuration::new(Duration::from_secs(3)),
                components::Graphics::new(
                    "./sprites/spells/blackhole_mini_4_4.png",
                    Duration::from_secs_f32(0.2),
                ),
                components::Velocity::new(0., -180.),
                components::Collision::new(16., 16., true, |e1, e2| vec![
                            (e1, GameAction::Remove(RemoveSource::ProjectileCollision)),
                            (e1, GameAction::play_sound("./audio/sounds/spells/blackhole_hit.wav")),
                            (e2, GameAction::TakeDamage { dmg: 80 }),
                            (e1, GameAction::spawn(|_, pos, cmd|{
                                cmd.push((
                                    pos + good_web_game::graphics::Vector2::new(0., -30.),
                                    components::LifeDuration::new(Duration::from_secs(6)),
                                    components::Graphics::new(
                                        "./sprites/spells/blackhole_8_8.png",
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
                                                            act_tar.push(GameAction::Move { delta: (pos_src - *pos_tar)* (1. / pos_src.distance2(*pos_tar).max(0.1)).min(1.) })
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
                                            GameAction::play_sound("./audio/sounds/spells/blackhole_explosion.wav"),
                                        )),
                                ));
                            }),)
                        ],),
            ));
        }),
        tiny_vec!([f32; MAX_SPELL_SLOTS] => 6., 6., 6., 6.),
    )
}

pub(super) fn construct_mind_wipe(
    sprite_pool: &mut SpritePool,
    ctx: &mut good_web_game::Context,
    gfx_ctx: &mut good_web_game::event::GraphicsContext,
) -> Spell {
    Spell::new(
        "Mind wipe",
        "Launch a bolt of dark energy that deals a medium amount of damage to the first enemy hit. After a short delay, deal the same damage again and silence the target for 15 seconds.",
        sprite_pool.init_sprite_fmt_unchecked("./sprites/spells/icons/mindwipe_icon_8_8.png", ctx, gfx_ctx, Duration::ZERO),
        "./audio/sounds/spells/mindwipe_cast.wav",
        GameAction::spawn(|_, pos, cmd| {
            cmd.push((
                pos,
                components::LifeDuration::new(Duration::from_secs(10)),
                components::Graphics::new(
                    "./sprites/spells/mindwipe_3_8.png",
                    Duration::from_secs_f32(0.2),
                ),
                components::Velocity::new(0., -250.),
                components::Collision::new(32., 32., true, |e1, e2| vec![
                            (e1, GameAction::Remove(RemoveSource::ProjectileCollision)),
                            (e1, GameAction::play_sound("./audio/sounds/spells/mindwipe_hit.wav")),
                            (e2, GameAction::TakeDamage { dmg: 42 }),
                            (e2, ActionEffect::once(ActionEffectTarget::new_only_self(), vec![
                                GameAction::TakeDamage { dmg: 42 },
                                GameAction::Silence(Duration::new(15, 0)),
                                GameAction::play_sound("./audio/sounds/spells/mindwipe_hit.wav"),
                            ]).with_duration(Duration::new(2, 0)).into()),
                        ],),
            ));
        }),
        tiny_vec!([f32; MAX_SPELL_SLOTS] => 3., 3., 6.),
    )
}

pub(super) fn construct_arcane_missiles(
    sprite_pool: &mut SpritePool,
    ctx: &mut good_web_game::Context,
    gfx_ctx: &mut good_web_game::event::GraphicsContext,
) -> Spell {
    Spell::new(
        "Arcane Missiles",
        "Infuse your self with arcane power. Every second for the next 10 seconds, launch an arcane missile towards a nearby enemy, dealing moderate damage.",
        sprite_pool.init_sprite_fmt_unchecked("./sprites/spells/icons/arcane_bolt_icon_8_8.png", ctx, gfx_ctx, Duration::ZERO),
        "./audio/sounds/spells/amissiles_cast.wav",
        ActionEffect::repeat(
            ActionEffectTarget::new_only_self(),
            GameAction::spawn(|_, pos_src, cmd|{
                // execute the following every seconds:
                cmd.exec_mut(move |world, _res|{
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


                        // push the missile
                        world.push((
                            pos_src,
                            components::LifeDuration::new(Duration::from_secs(10)),
                            components::Graphics::new(
                                "./sprites/spells/arcane_bolt_mini_4_4.png",
                                Duration::from_secs_f32(0.2),
                            ),
                            components::Velocity::from((target - pos_src)* (240. / pos_src.distance2(target).max(0.1))),
                            components::Collision::new(32., 32., true, |e1, e2| vec![
                                        (e1, GameAction::Remove(RemoveSource::ProjectileCollision)),
                                        (e1, GameAction::play_sound("./audio/sounds/spells/amissiles_hit.wav")),
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

pub(super) fn construct_arcane_blast(
    sprite_pool: &mut SpritePool,
    ctx: &mut good_web_game::Context,
    gfx_ctx: &mut good_web_game::event::GraphicsContext,
) -> Spell {
    Spell::new(
        "Arcane Blast",
        "Launch an orb of arcane energy dealing medium damage. On hitting an enemy, 8 smaller orbs are created centered on the target hit and striking inwards for the same amount of damage.",
        sprite_pool.init_sprite_fmt_unchecked("./sprites/spells/arcane_bolt_mini_4_4.png", ctx, gfx_ctx, Duration::ZERO),
        "./audio/sounds/spells/ablast_cast.wav",
        GameAction::spawn(|_, pos, cmd|{
            cmd.push((
                pos,
                components::LifeDuration::new(Duration::from_secs(10)),
                components::Graphics::new(
                    "./sprites/spells/arcane_bolt_8_8.png",
                    Duration::from_secs_f32(0.2),
                ),
                components::Velocity::new(0., -360.),
                components::Collision::new(32., 32., true, |e1, e2| vec![
                            (e1, GameAction::Remove(RemoveSource::ProjectileCollision)),
                            (e1, GameAction::play_sound("./audio/sounds/spells/ablast_hit1.wav")),
                            (e2, GameAction::TakeDamage { dmg: 30 }),
                            (e2, GameAction::spawn(|_, pos,cmd|{
                                for i in 0..8{
                                    let rel = good_web_game::graphics::Vector2::new(64. * (PI/4. * i as f32).cos(), 64. * (PI/4. * i as f32).sin());
                                    cmd.push((
                                        pos + rel,
                                        components::LifeDuration::new(Duration::from_secs(10)),
                                        components::Graphics::new(
                                            "./sprites/spells/arcane_bolt_mini_4_4.png",
                                            Duration::from_secs_f32(0.2),
                                        ),
                                        components::Velocity::from(rel* (240. / rel.distance2(Vector2::new(0.,0.)).max(f32::EPSILON)) * -1.),
                                        components::Collision::new(8., 8., true, |e1, e2| vec![
                                                    (e1, GameAction::Remove(RemoveSource::ProjectileCollision)),
                                                    (e1, GameAction::play_sound("./audio/sounds/spells/ablast_hit2.wav")),
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
