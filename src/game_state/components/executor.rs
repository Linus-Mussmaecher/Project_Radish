use crate::game_state::{game_action::ActionQueue, game_message::MessageSet};

use super::*;
use ggez::GameError;
use legion::*;

/// A struct that contains a vector of executive actions this entity can perform
/// An executive actions is a function pointer to a function that can read the entire world, defining its own queries
pub struct Executor {
    executive_actions:
        Vec<Box<dyn Fn(Entity, &legion::World, &mut ActionQueue, &mut MessageSet) + Send + Sync>>,
}

impl Executor {
    /// Creates a new executor component with only a single executive action
    pub fn new_single(
        executive_action: impl Fn(Entity, &legion::World, &mut ActionQueue, &mut MessageSet)
            + Send
            + Sync
            + 'static,
    ) -> Self {
        Self {
            executive_actions: vec![Box::new(executive_action)],
        }
    }

    pub fn new(
        executive_actions: Vec<
            Box<dyn Fn(Entity, &legion::World, &mut ActionQueue, &mut MessageSet) + Send + Sync>,
        >,
    ) -> Self {
        Self { executive_actions }
    }
}

pub fn resolve_executive_system(
    world: &mut World,
    resources: &mut Resources,
) -> Result<(), GameError> {
    let mut action_queue = resources
        .get_mut::<ActionQueue>()
        .ok_or_else(|| GameError::CustomError("Could not unpack action queue.".to_owned()))?;

    let mut message_set = resources
        .get_mut::<MessageSet>()
        .ok_or_else(|| GameError::CustomError("Could not unpack message set.".to_owned()))?;

    let executive_action_queue: Vec<(Entity, usize)> = action_queue
        .iter()
        .map_while(|(ent, action)| match action {
            GameAction::ExecutiveAction(i) => Some((*ent, *i)),
            _ => None,
        })
        .collect();

    for (ent, i) in executive_action_queue {
        if let Ok(entry) = world.entry_ref(ent) {
            if let Ok(executor) = entry.get_component::<Executor>() {
                if let Some(action) = executor.executive_actions.get(i) {
                    action(ent, world, &mut action_queue, &mut message_set);
                }
            }
        }
    }

    Ok(())
}
