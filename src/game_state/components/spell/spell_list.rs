use std::{time::Duration, vec};

use legion::{systems::CommandBuffer, Entity};
use mooeye::sprite::SpritePool;
use tinyvec::tiny_vec;

use crate::game_state::{
    components::{self, actions::*, spell::MAX_SPELL_SLOTS},
    game_message::MessageSet,
};

use super::Spell;

pub fn construct_fireball(spritepool: &SpritePool) -> Spell {
    Spell {
        spell_slots: tiny_vec!([f32; MAX_SPELL_SLOTS] => 1.5),
        name: "Fireball!".to_owned(),
        icon: spritepool
            .init_sprite("/sprites/fireball", Duration::from_secs_f32(1.))
            .expect("Could not initialize this spell."),
        spell_: |_| {
            vec![GameAction::spawn(|_, pos, sp, cmd| {
                cmd.push((
                    pos,
                    components::LifeDuration::new(Duration::from_secs(10)),
                    components::Graphics::from(sp.init_sprite("/sprites/fireball", Duration::from_secs_f32(0.2))
                        .expect("Could not load sprite.")),
                    components::Velocity::new(0., -250.),
                    components::Collision::new(32., 32., |e1, e2| {
                        (
                            vec![
                                (e1, GameAction::Remove),
                                (e2, GameAction::TakeDamage { dmg: 2 }),
                            ],
                            MessageSet::new(),
                        )
                    }),
                ));
            })]
        },
    }
}

pub fn construct_icebomb(spritepool: &SpritePool) -> Spell {
    Spell {
        spell_slots: tiny_vec!([f32; MAX_SPELL_SLOTS] => 1.5, 5.),
        name: "Ice Bomb".to_owned(),
        icon: spritepool
            .init_sprite("/sprites/icebomb", Duration::from_secs_f32(1.))
            .expect("Could not initialize this spell."),
        spell_: |_| {
            vec![GameAction::spawn(|_, pos, sp, cmd| {
                cmd.push((
                    pos,
                    components::LifeDuration::new(Duration::from_secs(10)),
                    components::Graphics::from(sp.init_sprite("/sprites/icebomb", Duration::from_secs_f32(0.2))
                        .expect("Could not load sprite.")),
                    components::Velocity::new(0., -520.),
                    components::Collision::new(32., 32., |e1, e2| {
                        (
                            vec![
                                (e1, GameAction::Remove),
                                (e2, GameAction::TakeDamage { dmg: 3 }),
                                (e1, GameAction::spawn(spawn_icebomb)),
                            ],
                            MessageSet::new(),
                        )
                    }),
                ));
            })]
        },
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
                .init_sprite("/sprites/icebomb", 
                    Duration::from_secs_f32(0.25), )
                .expect("Could not find sprite.");
            sprite.set_variant(1);
            components::Graphics::from(sprite)
        },
        components::Aura::new(
            128.,
            |act| {
                match act {
                    // slow down enemies by 90%
                    GameAction::Move { delta } => GameAction::Move { delta: delta * 0.1 },
                    other => other,
                }
            },
            // only enemies
            |entry| {
                if let Ok(_) = entry.get_component::<components::Enemy>() {
                    true
                } else {
                    false
                }
            },
        ),
    ));
}

pub fn construct_electrobomb(spritepool: &SpritePool) -> Spell {
    Spell {
        spell_slots: tiny_vec!([f32; MAX_SPELL_SLOTS] => 0.5, 2.5, 2.5, 25.0),
        name: "Ice Bomb".to_owned(),
        icon: spritepool
            .init_sprite("/sprites/fireball", Duration::from_secs_f32(1.))
            .expect("Could not initialize this spell."),
        spell_: |_| {
            vec![GameAction::spawn(|_, pos, sp, cmd| {
                cmd.push((
                    pos,
                    components::LifeDuration::new(Duration::from_secs(10)),
                    components::Graphics::from(sp.init_sprite("/sprites/electroorb", Duration::from_secs_f32(0.2))
                        .expect("Could not load sprite.")),
                    components::Velocity::new(0., -180.),
                    components::Collision::new(32., 32., |e1, e2| {
                        (
                            vec![
                                (e1, GameAction::AddImmunity { other: e2 }),
                                (
                                    e2,
                                    Distributor::new(GameActionContainer::single(GameAction::TakeDamage { dmg: 1 }))
                                    .with_range(128.)
                                    .with_enemies_only()
                                    .to_action(),
                                ),
                            ],
                            MessageSet::new(),
                        )
                    }),
                ));
            })]
        },
    }
}
