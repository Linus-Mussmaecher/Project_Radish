use legion::{systems::CommandBuffer, IntoQuery, *};
use std::time::Duration;

use crate::game_state::{components, controller::Interactions, game_message::MessageSet};
use mooeye::sprite::SpritePool;

use super::{actions::GameAction, Actions, Enemy, Position};

pub struct SpellCaster {
    //TODO: Implement struct/trait based spells
    cooldown: Duration,
}

impl SpellCaster {
    pub fn new() -> Self {
        Self {
            cooldown: Duration::ZERO,
        }
    }
}

#[system(for_each)]
pub fn spell_casting(
    position: &Position,
    caster: &mut SpellCaster,
    actions: &Actions,
    #[resource] ix: &Interactions,
    #[resource] sp: &SpritePool,
    cmd: &mut CommandBuffer,
) {
    caster.cooldown = caster.cooldown.saturating_sub(ix.delta);

    if actions.get_actions().contains(&GameAction::CastSpell(0)) && caster.cooldown.is_zero() {
        caster.cooldown = Duration::from_secs_f32(0.4);
        cmd.push((
            components::Position::new(position.x, position.y),
            super::LifeDuration::new(Duration::from_secs(10)),
            sp.init_sprite("/sprites/fireball", Duration::from_secs_f32(0.2))
                .expect("Could not load sprite."),
            super::Velocity::new(0., -250.),
            super::Collision::new(32., 32., |e1, e2| {
                (
                    vec![
                        (e1, GameAction::Remove),
                        (e2, GameAction::TakeDamage { dmg: 2 }),
                    ],
                    MessageSet::new(),
                )
            }),
        ));
    }

    if actions.get_actions().contains(&GameAction::CastSpell(1)) && caster.cooldown.is_zero() {
        caster.cooldown = Duration::from_secs_f32(1.5);
        cmd.push((
            components::Position::new(position.x, position.y),
            super::LifeDuration::new(Duration::from_secs(10)),
            sp.init_sprite("/sprites/icebomb", Duration::from_secs_f32(0.2))
                .expect("Could not load sprite."),
            super::Velocity::new(0., -520.),
            super::Collision::new(32., 32., |e1, e2| {
                (
                    vec![
                        (e1, GameAction::Remove),
                        (e2, GameAction::TakeDamage { dmg: 3 }),
                        (e1, GameAction::Spawn(&spawn_icebomb)),
                    ],
                    MessageSet::new(),
                )
            }),
        ));
    }

    if actions.get_actions().contains(&GameAction::CastSpell(2)) && caster.cooldown.is_zero() {
        caster.cooldown = Duration::from_secs_f32(1.5);
        cmd.push((
            components::Position::new(position.x, position.y),
            super::LifeDuration::new(Duration::from_secs(10)),
            sp.init_sprite("/sprites/electroorb", Duration::from_secs_f32(0.2))
                .expect("Could not load sprite."),
            super::Velocity::new(0., -180.),
            super::Collision::new(32., 32., |e1, e2| {
                (
                    vec![
                        (e1, GameAction::AddImmunity { other: e2 }),
                        (e2, GameAction::OtherAction(&area_dmg)),
                    ],
                    MessageSet::new(),
                )
            }),
        ));
    }
}

fn area_dmg(src: Entity, cmd: &mut CommandBuffer) {
    cmd.exec_mut(move |world, _| {
        if let Ok(entry) = world.entry_ref(src) {
            if let Ok(_enemy) = entry.get_component::<Enemy>() {
                if let Ok(pos) = entry.get_component::<Position>() {
                    let pos_save = *pos;
                    for (actions, _enemy2, pos2) in
                        <(&mut Actions, &Enemy, &Position)>::query().iter_mut(world)
                    {
                        if pos_save.distance(*pos2) < 128. {
                            actions.push(GameAction::TakeDamage { dmg: 1 });
                        }
                    }
                }
            }
        }
    });
}

fn spawn_icebomb(_: Entity, pos: Position, cmd: &mut CommandBuffer) {
    cmd.exec_mut(move |world, res| {
        let spritepool = res
            .get::<SpritePool>()
            .expect("Could not unpack sprite pool in resources.");

        world.push((
            pos,
            super::LifeDuration::new(Duration::from_secs(5)),
            {
                let mut sprite = spritepool
                    .init_sprite("/sprites/icebomb", Duration::from_secs_f32(0.25))
                    .expect("Could not find sprite.");
                sprite.set_variant(1);
                sprite
            },
            super::Aura::new(
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
                    if let Ok(_) = entry.get_component::<Enemy>() {
                        true
                    } else {
                        false
                    }
                },
            ),
        ));
    });
}
