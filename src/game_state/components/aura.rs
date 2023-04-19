use legion::{world::SubWorld, *};

use crate::game_state::game_action::ActionQueue;

use super::{GameAction, Position};
pub struct Aura {
    transform: Box<dyn Fn(GameAction) -> GameAction + Send + Sync>,
    range: f32,
}

impl Aura {
    pub fn new(
        range: f32,
        transform: impl Fn(GameAction) -> GameAction + 'static + Send + Sync,
    ) -> Self {
        Self {
            transform: Box::new(transform),
            range,
        }
    }
}

#[system]
pub fn aura(
    world: &mut SubWorld,
    query: &mut Query<(&Aura, &Position)>,
    query_target: &mut Query<(Entity, &Position)>,
    #[resource] actions: &mut ActionQueue,
) {
    for (aura, pos) in query.iter(world) {
        for (ent, pos_target) in query_target.iter(world) {
            if pos.distance(*pos_target) < aura.range {
                *actions = actions
                    .iter()
                    .map(|(ent_a, act)| {
                        if *ent_a == *ent {
                            (*ent_a, (aura.transform)(*act))
                        } else {
                            (*ent_a, *act)
                        }
                    })
                    .collect();
            }
        }
    }
}
