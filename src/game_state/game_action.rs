use ggez::glam::Vec2;

pub type ActionQueue = Vec<(legion::Entity, GameAction)>;

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum GameAction{
    Remove,
    Move{delta: Vec2},
    TakeDamage{dmg: i32},
    TakeCityDamage{dmg: i32},
    GainGold{amount: i32},
    ExecutiveAction(usize),
    AddImmunity{other: legion::Entity},
    CastSpell(usize),
}