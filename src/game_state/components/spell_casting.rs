use std::time::Duration;
use legion::{systems::CommandBuffer, *};

use crate::{
    game_state::{
        components, controller::Interactions, game_action::ActionQueue, game_message::MessageSet,
    },
    sprite_pool::SpritePool,
};

use super::{GameAction, Position};

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
    entity: &Entity,
    position: &Position,
    caster: &mut SpellCaster,
    #[resource] action_queue: &ActionQueue,
    #[resource] ix: &Interactions,
    #[resource] sp: &SpritePool,
    cmd: &mut CommandBuffer,
) {
    caster.cooldown = caster.cooldown.saturating_sub(ix.delta);

    if action_queue.contains(&(*entity, GameAction::CastSpell(1)))
        && caster.cooldown == Duration::ZERO
    {
        caster.cooldown = Duration::from_secs_f32(0.5);
        cmd.push((
            components::Position::new(position.x, position.y),
            super::LifeDuration::new(Duration::from_secs(7)),
            sp.init_sprite("/sprites/fireball_8_8.png", Duration::from_secs_f32(0.2))
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
}
