use std::sync::Arc;

use legion::{world::EntryRef, *};

use super::{actions::GameAction, Actions, Position};
/// A component that continously modifies actions applied to entities around it.
pub struct Aura {
    /// The action modification applied.
    transform: Arc<dyn Fn(&mut GameAction) + Send + Sync>,
    /// Manages which entities are affected by this aura.
    predicate: Box<dyn Fn(&EntryRef) -> bool + Send + Sync>,
    /// The range of the aura.
    range: f32,
}

impl Aura {
    /// Creates a new aura with the specified transform and predicate.
    pub fn new(
        range: f32,
        transform: impl Fn(&mut GameAction) + 'static + Send + Sync,
        predicate: impl Fn(&EntryRef) -> bool + 'static + Send + Sync,
    ) -> Self {
        Self {
            transform: Arc::new(transform),
            predicate: Box::new(predicate),
            range,
        }
    }
}

/// A special system that applies all aura-components that transform actions of other entities
pub fn aura_system(world: &mut World) {
    // compile a list of all auras affecting entities
    let mut auras = Vec::new();

    // For every entity that has a position and actions
    for (ent_tar, pos_target, _actions) in <(Entity, &Position, &Actions)>::query().iter(world) {
        for (aura, pos) in <(&Aura, &Position)>::query().iter(world) {
            if pos.distance(*pos_target) < aura.range
                && (aura.predicate)(&world.entry_ref(*ent_tar).expect("Entry vanished"))
            {
                auras.push((*ent_tar, aura.transform.clone()));
            }
        }
    }

    // replace all changed action lists with their new version
    for (target, transform) in auras {
        if let Ok(mut entry) = world.entry_mut(target) {
            if let Ok(actions) = entry.get_component_mut::<Actions>() {
                for action in actions.get_actions_mut() {
                    (transform)(action);
                }
            }
        }
    }
}
