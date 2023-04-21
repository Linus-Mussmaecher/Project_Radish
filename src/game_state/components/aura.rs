use legion::{world::EntryRef, *};

use super::{actions::GameAction, Actions, Position};
pub struct Aura {
    transform: Box<dyn Fn(GameAction) -> GameAction + Send + Sync>,
    predicate: Box<dyn Fn(&EntryRef) -> bool + Send + Sync>,
    range: f32,
}

impl Aura {
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

pub fn aura_sytem(world: &mut World) {
    let mut list = Vec::new();

    for (ent, pos_target, actions) in <(Entity, &Position, &Actions)>::query().iter(world) {
        let mut actions_new  = actions.get_actions().clone();

        for (aura, pos) in <(&Aura, &Position)>::query().iter(world) {
            if pos.distance(*pos_target) < aura.range
                && (aura.predicate)(&world.entry_ref(*ent).expect("Entry vanished"))
            {
                actions_new = actions_new.iter().map(|act| (aura.transform)(*act)).collect();
            }
        }

        list.push((
            *ent,
            actions_new
        ))
    }

    for (ent, new_actions) in list {
        if let Some(mut entry) = world.entry(ent) {
            entry.add_component(Actions::from(new_actions));
        }
    }
}
