use ggez::glam::Vec2;
use legion::*;
use crate::game_state::game_action::{ActionQueue, GameAction};


pub type Position = Vec2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Velocity{
    pub dx: f32,
    pub dy: f32,
}

impl From<Vec2> for Velocity{
    fn from(value: Vec2) -> Self {
        Self { dx: value.x, dy: value.y }
    }
}

impl From<Velocity> for Vec2 {
    fn from(value: Velocity) -> Self {
        Self { x: value.dx, y: value.dx }
    }
}


#[legion::system(for_each)]
pub fn update_position(entity: &Entity, vel: &Velocity,  #[resource] actions: &mut ActionQueue){
    actions.push_back(GameAction::Move { entity: *entity, del: Vec2::from(*vel) })
}

#[legion::system(for_each)]
pub fn position_apply(entity2: &Entity, pos: &mut Position, #[resource] actions: &ActionQueue){
    for action in actions {
        if let GameAction::Move{ entity, del } = action{
            if *entity == *entity2 {
                *pos += *del;
            }
        }
    }
}