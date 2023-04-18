use crate::game_state::{
    controller::Interactions,
    game_action::{ActionQueue, GameAction},
};
use ggez::glam::Vec2;
use legion::*;

pub type Position = Vec2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Velocity {
    dx: f32,
    dy: f32,
}

impl Velocity {
    pub fn new(dx: f32, dy: f32) -> Self {
        Self { dx, dy }
    }

    pub fn get_dx(&self) -> f32 {
        self.dx
    }

    pub fn get_dy(&self) -> f32 {
        self.dy
    }
}

impl From<Vec2> for Velocity {
    fn from(value: Vec2) -> Self {
        Self {
            dx: value.x,
            dy: value.y,
        }
    }
}

impl From<Velocity> for Vec2 {
    fn from(value: Velocity) -> Self {
        Self {
            x: value.dx,
            y: value.dy,
        }
    }
}

#[legion::system(for_each)]
pub fn velocity(
    entity: &Entity,
    vel: &Velocity,
    #[resource] actions: &mut ActionQueue,
    #[resource] ix: &Interactions,
) {
    actions.push((
        *entity,
        GameAction::Move {
            delta: Vec2::from(*vel) * ix.delta.as_secs_f32(),
        },
    ))
}

#[legion::system(for_each)]
pub fn resolve_move(entity2: &Entity, pos: &mut Position, #[resource] actions: &ActionQueue, #[resource] boundaries: &ggez::graphics::Rect) {
    for action in actions {
        if let (entity, GameAction::Move { delta }) = action {
            if *entity == *entity2 {
                *pos += *delta;
                // check horizontal bounds
                pos.x = pos.x.clamp(boundaries.x, boundaries.x + boundaries.w);
            }
        }
    }
}
