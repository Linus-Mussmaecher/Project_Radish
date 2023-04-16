use std::collections::VecDeque;

use ggez::glam::Vec2;

pub type ActionQueue = VecDeque<(legion::Entity, GameAction)>;

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum GameAction{
    Move{delta: Vec2},
    TakeDamage{dmg: i32},
    TakeCityDamage{dmg: i32},
    GainGold{amount: i32},
    ExecutiveAction(usize),
    AddImmunity{other: legion::Entity},
}