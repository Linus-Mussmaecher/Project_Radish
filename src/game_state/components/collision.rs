use ggez::{glam::Vec2, graphics::Rect};
use legion::{world::SubWorld, *};

use crate::game_state::{game_action::ActionQueue, game_message::MessageSet};

use super::{GameAction, Position};
pub struct Collision {
    w: f32,
    h: f32,

    respects_boundaries: bool,

    collision_handler: Box<dyn Fn(Entity, Entity) -> (ActionQueue, MessageSet) + Send + Sync>,

    immunity: Vec<Entity>,
}

impl Collision {
    pub fn new(
        w: f32,
        h: f32,
        respects_boundaries: bool,
        collision_handler: impl Fn(Entity, Entity) -> (ActionQueue, MessageSet) + Send + Sync + 'static,
    ) -> Self {
        Self {
            w,
            h,
            collision_handler: Box::new(collision_handler),
            immunity: Vec::new(),
            respects_boundaries,
        }
    }

    pub fn new_basic(w: f32, h: f32) -> Self {
        Self::new(w, h, true,|_, _| (ActionQueue::new(), MessageSet::new()))
    }

    fn get_collider(&self, pos: Vec2) -> Rect {
        Rect::new(pos.x - self.w / 2., pos.y - self.h / 2., self.w, self.h)
    }
}

#[system]
#[read_component(Position)]
#[read_component(Collision)]
pub fn collision(
    world: &mut SubWorld,
    #[resource] actions: &mut ActionQueue,
    #[resource] messages: &mut MessageSet,
    #[resource] boundaries: &ggez::graphics::Rect,
) {
    // boundaries check
    for (pos1, col1) in <(&mut Position, &Collision)>::query().iter_mut(world) {
        if col1.respects_boundaries{
            pos1.x = pos1.x.clamp(boundaries.x + col1.w/2., boundaries.x + boundaries.w - col1.w/2.);
        }
    }
    // collision with other entities
    for (ent1, pos1, col1) in <(Entity, &Position, &Collision)>::query().iter(world) {
        
        for (ent2, pos2, col2) in <(Entity, &Position, &Collision)>::query().iter(world) {
            if col1.get_collider(*pos1).overlaps(&col2.get_collider(*pos2))
                && *ent1 != *ent2
                && !col1.immunity.contains(ent2)
            {
                //println!("Collisions: {:?}, {:?}", *ent1, *ent2);
                let (n_actions, n_messages) = (col1.collision_handler)(*ent1, *ent2);
                messages.extend(n_messages.iter());
                actions.extend(n_actions.iter());
            }
        }
    }
}

#[system(for_each)]
pub fn resolve_immunities(
    this: &Entity,
    collision: &mut Collision,
    #[resource] actions: &ActionQueue,
) {
    for (ent, action) in actions {
        if *this == *ent {
            if let GameAction::AddImmunity { other } = action {
                collision.immunity.push(*other);
            }
        }
    }
}
