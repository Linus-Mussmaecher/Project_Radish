use ggez::{graphics::Rect, glam::Vec2};
use legion::{world::SubWorld, *};

use crate::game_state::{game_action::ActionQueue, game_message::MessageSet};

use super::Position;

type CollisionHandler = dyn Fn(Entity, Entity, &SubWorld, &mut ActionQueue, &mut MessageSet) + Send + Sync;

pub struct Collision {
    w: f32,
    h: f32,
    collision_handler: Box<CollisionHandler>,
}

impl Collision {
    pub fn new(w: f32, h:f32, collision_handler: impl Fn(Entity, Entity, &SubWorld, &mut ActionQueue, &mut MessageSet) + Send + Sync + 'static) -> Self{
        Self {w,h, collision_handler: Box::new(collision_handler) }
    }

    pub fn new_basic(w: f32, h: f32) -> Self{
        Self::new(w,h, &bounce_back)
    }

    fn get_collider(&self, pos: Vec2) -> Rect{
        Rect::new(pos.x, pos.y, self.w, self.h)
    }
}

#[system]
#[read_component(Position)]
#[read_component(Collision)]
pub fn collide(world: &mut SubWorld, #[resource] actions: &mut ActionQueue, #[resource] messages: &mut MessageSet){
    for (ent1, pos1, col1) in <(Entity,&Position, &Collision)>::query().iter(world){
        for (ent2, pos2, col2) in <(Entity, &Position, &Collision)>::query().iter(world){
            if col1.get_collider(*pos1).overlaps(&col2.get_collider(*pos2)) && *ent1 != *ent2{
                (col1.collision_handler)(*ent1, *ent2, world, actions, messages);
            }
        }
    }
}

pub fn bounce_back(e1: Entity, e2: Entity, world: &SubWorld, actions: &mut ActionQueue, _messages: &mut MessageSet){
    let v1 = *world.entry_ref(e1).unwrap().into_component::<Position>().unwrap();
    let v2 = *world.entry_ref(e2).unwrap().into_component::<Position>().unwrap();
    actions.push_back((e1, super::GameAction::Move { delta: (v1 - v2).normalize_or_zero() }));
}