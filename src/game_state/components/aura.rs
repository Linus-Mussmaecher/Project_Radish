use legion::{world::EntryRef, *};
use tinyvec::TinyVec;

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

        let actions_new = actions.get_actions().iter().map(|act| {
            let mut curr_act = *act;
            for (aura, pos) in <(&Aura, &Position)>::query().iter(world) {
                if pos.distance(*pos_target) < aura.range
                    && (aura.predicate)(&world.entry_ref(*ent).expect("Entry vanished"))
                {
                    curr_act = (aura.transform)(curr_act);
                }
            }
            curr_act
        });

        list.push((*ent, actions_new.collect::<TinyVec<[GameAction;4]>>()))
    }

    for (ent, new_actions) in list {
        if let Some(mut entry) = world.entry(ent) {
            entry.add_component(Actions::from(new_actions));
        }
    }
}
