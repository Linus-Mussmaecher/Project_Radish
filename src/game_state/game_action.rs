use std::collections::VecDeque;

use ggez::glam::Vec2;

pub type ActionQueue = VecDeque<(legion::Entity, GameAction)>;

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum GameAction{
    Move{delta: Vec2},
    TakeDamage{dmg: i32},
    TakeCityDamage{dmg: u32},
    GainGold{amount: u32},
    ExecutiveAction(i32),
    AddImmunity{other: legion::Entity},
}

impl GameAction{

    // pub fn executive_action(lambda: impl Fn(&mut legion::World, &mut ActionQueue, &mut MessageSet) + Send + Sync + 'static) -> Self{
    //     Self::ExecutiveAction(Box::new(lambda))
    // }
}