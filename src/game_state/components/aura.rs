use legion::{world::EntryRef, *};
use tinyvec::TinyVec;

use super::{actions::GameAction, Actions, Position};
/// A component that continously modifies actions applied to entities around it.
pub struct Aura {
    /// The action modification applied.
    transform: Box<dyn Fn(GameAction) -> GameAction + Send + Sync>,
    /// Manages which entities are affected by this aura.
    predicate: Box<dyn Fn(&EntryRef) -> bool + Send + Sync>,
    /// The range of the aura.
    range: f32,
}

impl Aura {
    /// Creates a new aura with the specified transform and predicate.
    pub fn new(
        range: f32,
        transform: impl Fn(GameAction) -> GameAction + 'static + Send + Sync,
        predicate: impl Fn(&EntryRef) -> bool + 'static + Send + Sync,
    ) -> Self {
        Self {
            transform: Box::new(transform),
            predicate: Box::new(predicate),
            range,
        }
    }
}

/// A special system that applies all aura-components that transform actions of other entities
pub fn aura_sytem(world: &mut World) {
    // Create a list of changes to action lists
    let mut list = Vec::new();

    // For every entity that has a position and actions
    for (ent, pos_target, actions) in <(Entity, &Position, &Actions)>::query().iter(world) {
        // compile a list of all auras affecting this entity
        let mut auras = Vec::new();
        for (aura, pos) in <(&Aura, &Position)>::query().iter(world) {
            if pos.distance(*pos_target) < aura.range
                && (aura.predicate)(&world.entry_ref(*ent).expect("Entry vanished"))
            {
                auras.push(aura);
            }
        }
        // if there are any, create a new action list by aplying all auras in order to all actions
        if auras.len() > 0 {
            let actions_new = actions.get_actions().iter().map(move |act| {
                let mut curr_act = act.clone();
                for aura in auras.iter() {
                    curr_act = (aura.transform)(curr_act);
                }
                curr_act
            });
            // push the new list to the list of changed actions
            list.push((*ent, actions_new.collect::<TinyVec<[GameAction; 4]>>()))
        }
    }

    // replace all changed action lists with their new version
    for (ent, new_actions) in list {
        if let Some(mut entry) = world.entry(ent) {
            entry.add_component(Actions::from(new_actions));
        }
    }
}
