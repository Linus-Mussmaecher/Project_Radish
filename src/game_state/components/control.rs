use crate::game_state::controller::Interactions;
use ggez::glam::Vec2;
use legion::*;

use super::Actions;

pub struct Control {
    move_speed: f32,
}

impl Control {
    pub fn new(move_speed: f32) -> Self {
        Self { move_speed }
    }
}

#[system(for_each)]
pub fn control(
    control: &Control,
    actions: &mut Actions,
    #[resource] ix: &Interactions,
) {
    let mut del = Vec2::ZERO;

    if let Some(true) = ix
        .commands
        .get(&crate::game_state::controller::Command::MoveLeft)
    {
        del.x -= 1.;
    }
    if let Some(true) = ix
        .commands
        .get(&crate::game_state::controller::Command::MoveRight)
    {
        del.x += 1.;
    }

    actions.push(super::actions::GameAction::Move {
        delta: del * control.move_speed * ix.delta.as_secs_f32(),
    });

    if let Some(true) = ix
        .commands
        .get(&crate::game_state::controller::Command::Spell0)
    {
        actions.push(super::actions::GameAction::CastSpell(0));
    }
    if let Some(true) = ix
        .commands
        .get(&crate::game_state::controller::Command::Spell1)
    {
        actions.push(super::actions::GameAction::CastSpell(1));
    }
    if let Some(true) = ix
        .commands
        .get(&crate::game_state::controller::Command::Spell2)
    {
        actions.push(super::actions::GameAction::CastSpell(2));
    }
    if let Some(true) = ix
        .commands
        .get(&crate::game_state::controller::Command::Spell3)
    {
        actions.push(super::actions::GameAction::CastSpell(3));
    }
}
