use crate::game_state::{game_action::ActionQueue};
use ggez::{glam::Vec2, Context};
use legion::*;


pub struct Control{
    move_speed: f32,
}

impl Control {
    pub fn new(move_speed: f32) -> Self{
        Self{move_speed}
    }
}

pub fn control_csystem(world: &mut World, ctx: &Context, actions: &mut ActionQueue){

    let mut del = Vec2::ZERO;

    if ctx.keyboard.is_key_pressed(ggez::winit::event::VirtualKeyCode::Up){
        del.y -= 1.;
    }
    if ctx.keyboard.is_key_pressed(ggez::winit::event::VirtualKeyCode::Down){
        del.y += 1.;
    }
    if ctx.keyboard.is_key_pressed(ggez::winit::event::VirtualKeyCode::Left){
        del.x -= 1.;
    }
    if ctx.keyboard.is_key_pressed(ggez::winit::event::VirtualKeyCode::Right){
        del.x += 1.;
    }

    for (entity, control) in <(Entity, &Control)>::query().iter(world){
        actions.push_back((*entity, super::GameAction::Move { delta: del * control.move_speed}))
    }
}