use std::{time::Duration, vec};

use legion::{systems::CommandBuffer, Entity};
use mooeye::sprite::SpritePool;
use tinyvec::tiny_vec;

use crate::game_state::{
    components::{self, actions::*, graphics::Particle, spell::MAX_SPELL_SLOTS},
    game_message::MessageSet,
};

use super::Spell;

pub fn construct_fireball(spritepool: &SpritePool) -> Spell {
    Spell {
        spell_slots: tiny_vec!([f32; MAX_SPELL_SLOTS] => 1.5),
        name: "Fireball!".to_owned(),
        icon: spritepool
            .init_sprite_unchecked("/sprites/spells/fireball", Duration::from_secs_f32(1.3)),
        spell_: GameAction::spawn(|_, pos, sp, cmd| {
            cmd.push((
                pos,
                components::LifeDuration::new(Duration::from_secs(10)),
                components::Graphics::from(
                    sp.init_sprite_unchecked("/sprites/spells/fireball", Duration::from_secs_f32(0.2)),
                ),
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
        })
        .into(),
    }
}

pub fn construct_icebomb(spritepool: &SpritePool) -> Spell {
    Spell {
        spell_slots: tiny_vec!([f32; MAX_SPELL_SLOTS] => 1.5, 5.),
        name: "Ice Bomb".to_owned(),
        icon: spritepool
            .init_sprite_unchecked("/sprites/spells/icebomb", Duration::from_secs_f32(1.)),
        spell_: GameAction::spawn(|_, pos, sp, cmd| {
            cmd.push((
                pos,
                components::LifeDuration::new(Duration::from_secs(10)),
                components::Graphics::from(
                    sp.init_sprite_unchecked("/sprites/spells/icebomb", Duration::from_secs_f32(0.2)),
                ),
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
        })
        .into(),
    }
}

fn spawn_icebomb(
    _: Entity,
    pos: components::Position,
    spritepool: &SpritePool,
    cmd: &mut CommandBuffer,
) {
    cmd.push((
        pos,
        components::LifeDuration::new(Duration::from_secs(5)),
        {
            let mut sprite = spritepool
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

pub fn construct_electrobomb(spritepool: &SpritePool) -> Spell {
    Spell {
        spell_slots: tiny_vec!([f32; MAX_SPELL_SLOTS] => 2.5, 30.0),
        name: "Ice Bomb".to_owned(),
        icon: spritepool
            .init_sprite_unchecked("/sprites/spells/fireball", Duration::from_secs_f32(1.)),
        spell_: GameAction::spawn(|_, pos, sp, cmd| {
            cmd.push((
                pos,
                components::LifeDuration::new(Duration::from_secs(10)),
                components::Graphics::from(
                    sp.init_sprite_unchecked("/sprites/spells/electroorb", Duration::from_secs_f32(0.2)),
                ),
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
        })
        .into(),
    }
}

pub fn construct_conflagrate(spritepool: &SpritePool) -> Spell {
    Spell {
        spell_slots: tiny_vec!([f32; MAX_SPELL_SLOTS] => 4., 4., 10., 10.),
        name: "Conflagrate".to_owned(),
        icon: spritepool
            .init_sprite_unchecked("/sprites/spells/fireball", Duration::from_secs(1)),
        spell_: ActionEffect::once(
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
                    Particle::new(
                        spritepool
                            .init_sprite_unchecked("/sprites/spells/burning", Duration::from_secs_f32(0.25))
                    )
                    .with_duration(Duration::from_secs(5)),
                ),
            ],
        )
        .into(),
    }
}
