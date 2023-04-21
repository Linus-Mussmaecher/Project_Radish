use crate::game_state::{controller::Interactions, components::actions::GameAction};
use ggez::glam::Vec2;
use legion::*;

use super::Actions;

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

#[system(for_each)]
pub fn velocity(vel: &Velocity, actions: &mut Actions, #[resource] ix: &Interactions) {
    actions.push(GameAction::Move {
        delta: Vec2::from(*vel) * ix.delta.as_secs_f32(),
    })
}

#[system(for_each)]
pub fn resolve_move(pos: &mut Position, actions: &Actions) {
    for action in actions.get_actions() {
        if let GameAction::Move { delta } = action {
            *pos += *delta;
        }
    }
}
