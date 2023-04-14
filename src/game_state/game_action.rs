use std::collections::VecDeque;

use ggez::glam::Vec2;

pub type ActionQueue = VecDeque<(legion::Entity, GameAction)>;

pub enum GameAction{
    Move{delta: Vec2},
    #[allow(dead_code)]
    TakeDamage{dmg: i32},
    TakeCityDamage{dmg: u32},
    GainGold{amount: u32},
}