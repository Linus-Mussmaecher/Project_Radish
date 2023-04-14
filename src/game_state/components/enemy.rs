use legion::*;
use crate::game_state::game_action::ActionQueue;

use super::Health;
use super::Position;

const CITY_BORDER: f32 = 850f32;

pub struct Enemy {
    damage: u32,
    bounty: u32,
}

impl Enemy {
    pub fn new(damage: u32, bounty: u32) -> Self {
        Self { damage, bounty }
    }
}


#[system(for_each)]
pub fn manage_enemies(entity: &Entity, enemy: &Enemy, health: Option<&Health>, pos: Option<&Position>, #[resource] actions: &mut ActionQueue){
    if match health {
        None => false,
        Some(health) => health.0 <= 0,
    } {
        actions.push_back((*entity, super::GameAction::GainGold { amount: enemy.bounty }));
    }

    if match pos {
        None => false,
        Some(pos) => pos.y >= CITY_BORDER,
    }{
        actions.push_back((*entity, super::GameAction::TakeCityDamage { dmg: enemy.damage }));
    }
}