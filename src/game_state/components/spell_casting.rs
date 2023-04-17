use std::time::Duration;

use ggez::Context;
use legion::{systems::CommandBuffer, *};

use crate::game_state::{game_action::ActionQueue, game_message::MessageSet, controller::Interactions, components};

use super::{GameAction, Position};

pub struct SpellCaster {
    //TODO: Implement struct/trait based spells
    cooldown: Duration,
}

impl SpellCaster{
    pub fn new() -> Self{
        Self { cooldown: Duration::ZERO }
    }
}

pub fn spell_casting(world: &mut World, resources: &mut Resources, ctx: &mut Context) {


    let mut commands = CommandBuffer::new(world);
    {
        let action_queue = resources
            .get_mut::<ActionQueue>()
            .expect("Could not unpack action queue.");
        let ix = resources
            .get_mut::<Interactions>()
            .expect("Could not unpack interations.");        

        for (entity, position, caster) in <(Entity, &Position, &mut SpellCaster)>::query().iter_mut(world) {
            caster.cooldown = caster.cooldown.saturating_sub(ix.delta);

            if action_queue.contains(&(*entity, GameAction::CastSpell(1))) && caster.cooldown == Duration::ZERO {
                caster.cooldown = Duration::from_secs_f32(0.5);
                commands.push((
                    components::Position::new(position.x, position.y),
                    super::LifeDuration::new(Duration::from_secs(7)),
                    super::Sprite::from_path_fmt(
                        "/sprites/fireball_8_8.png",
                        ctx,
                        Duration::from_secs_f32(0.25),
                    )
                    .expect("Could not load sprite."),
                    super::Velocity::new(0., -250.),
                    super::Collision::new(32., 32., |e1, e2| (
                        vec![
                            (e1, GameAction::Remove),
                            (e2, GameAction::TakeDamage { dmg: 2 })
                        ],
                        MessageSet::new(),
                    )),
                ));
            }
        }
    }

    commands.flush(world, resources);
}
