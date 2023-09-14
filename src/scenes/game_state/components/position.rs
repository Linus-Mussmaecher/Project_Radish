use legion::system;

use super::Actions;

pub type Position = good_web_game::graphics::Vector2;

#[derive(Clone, Copy, Debug, PartialEq)]
/// A system that manages a fixed movement of an entity each second.
pub struct Velocity {
    /// The amount of pixels this unit travels horizontally each second.
    dx: f32,
    /// The amount of pixels this unit travels vertically each second.
    dy: f32,
}

impl Velocity {
    /// Creates a new velocity component.
    pub fn new(dx: f32, dy: f32) -> Self {
        Self { dx, dy }
    }

    /// Returns the amount of pixels this unit travels horizontally each second.
    pub fn get_dx(&self) -> f32 {
        self.dx
    }

    /// Returns the amount of pixels this unit travels vertically each second.
    pub fn get_dy(&self) -> f32 {
        self.dy
    }
}

impl From<good_web_game::graphics::Vector2> for Velocity {
    fn from(value: good_web_game::graphics::Vector2) -> Self {
        Self {
            dx: value.x,
            dy: value.y,
        }
    }
}

impl From<Velocity> for good_web_game::graphics::Vector2 {
    fn from(value: Velocity) -> Self {
        Self {
            x: value.dx,
            y: value.dy,
        }
    }
}

#[system(for_each)]
/// Moves entities with the velocity component.
pub fn velocity(
    vel: &Velocity,
    actions: &mut Actions,
    #[resource] ix: &super::super::Interactions,
) {
    actions.push(super::actions::GameAction::Move {
        delta: good_web_game::graphics::Vector2::from(*vel) * ix.delta.as_secs_f32(),
    })
}

#[system(for_each)]
/// Resolves movement events.
pub fn resolve_move(pos: &mut Position, actions: &Actions) {
    for action in actions.get_actions() {
        if let super::actions::GameAction::Move { delta } = action {
            *pos += *delta;
        }
    }
}
