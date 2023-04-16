use crate::game_state::{game_action::ActionQueue, controller::Interactions};
use ggez::{glam::Vec2};
use legion::*;


pub struct Control{
    move_speed: f32,
}

impl Control {
    pub fn new(move_speed: f32) -> Self{
        Self{move_speed}
    }
}

#[system(for_each)]
pub fn control(entity: &Entity, control: &Control, #[resource] ix: &Interactions, #[resource] actions: &mut ActionQueue){ 

    let mut del = Vec2::ZERO;

    if let Some(true) = ix.commands.get(&crate::game_state::controller::Command::MoveLeft){
        del.x -= 1.;
    }
    if let Some(true) = ix.commands.get(&crate::game_state::controller::Command::MoveRight){
        del.x += 1.;
    }

    actions.push_back((*entity, super::GameAction::Move { delta: del * control.move_speed}))
}