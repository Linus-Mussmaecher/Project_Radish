use std::collections::VecDeque;

use ggez::glam::Vec2;

pub type ActionQueue = VecDeque<GameAction>;

pub enum GameAction{
    Move{entity: legion::Entity, del: Vec2},
    #[allow(dead_code)]
    None,
}