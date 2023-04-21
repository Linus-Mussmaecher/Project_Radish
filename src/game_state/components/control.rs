use crate::game_state::{controller::Interactions, game_action::ActionQueue};
use ggez::glam::Vec2;
use legion::*;

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
    entity: &Entity,
    control: &Control,
    #[resource] ix: &Interactions,
    #[resource] actions: &mut ActionQueue,
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

    actions.push((
        *entity,
        super::GameAction::Move {
            delta: del * control.move_speed * ix.delta.as_secs_f32(),
        },
    ));

    if let Some(true) = ix
        .commands
        .get(&crate::game_state::controller::Command::Spell0)
    {
        actions.push((*entity, super::GameAction::CastSpell(0)));
    }
    if let Some(true) = ix
        .commands
        .get(&crate::game_state::controller::Command::Spell1)
    {
        actions.push((*entity, super::GameAction::CastSpell(1)));
    }
    if let Some(true) = ix
        .commands
        .get(&crate::game_state::controller::Command::Spell2)
    {
        actions.push((*entity, super::GameAction::CastSpell(2)));
    }
    if let Some(true) = ix
        .commands
        .get(&crate::game_state::controller::Command::Spell3)
    {
        actions.push((*entity, super::GameAction::CastSpell(3)));
    }
}
