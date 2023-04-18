use legion::{systems::CommandBuffer, *};

use crate::game_state::game_action::ActionQueue;

use super::Position;

/// The Health component track wether a unit has a life bar and can take damage.
pub struct Health {
    curr_health: i32,
    max_health: i32,
}

impl Health {
    /// Creates a new health struct with both the maximum and current health set to the passed value.
    pub fn new(health: i32) -> Self {
        Self {
            curr_health: health,
            max_health: health,
        }
    }

    /// Returns the unis current health.
    pub fn get_current_health(&self) -> i32 {
        self.curr_health
    }

    /// Returns the unis maximum health.
    pub fn get_max_health(&self) -> i32 {
        self.max_health
    }
}

pub struct Enemy {
    damage: i32,
    bounty: i32,
}

impl Enemy {
    pub fn new(damage: i32, bounty: i32) -> Self {
        Self { damage, bounty }
    }
}

#[system(for_each)]
/// Removes entities with zero health or less
pub fn remove_dead(entity: &Entity, #[resource] actions: &mut ActionQueue, commands: &mut CommandBuffer) {
    if actions.contains(&(*entity, super::GameAction::Remove)){
        commands.remove(*entity);
    }
}

#[system(for_each)]
/// Applies all [TakeDamage] actions to their respective entities.
pub fn resolve_damage(
    ent: &Entity,
    health: &mut Health,
    enemy: Option<&Enemy>,
    #[resource] actions: &mut ActionQueue,
) {
    for action in actions.iter() {
        if let (entity, super::GameAction::TakeDamage { dmg }) = action {
            if *entity == *ent {
                health.curr_health -= *dmg;
            }
        }
    }
    if health.curr_health <= 0 {
        actions.push((*ent, super::GameAction::Remove));

        if let Some(enemy) = enemy {
            actions.push((
                *ent,
                super::GameAction::GainGold {
                    amount: enemy.bounty,
                },
            ));
        }
    }
}

#[system(for_each)]
pub fn enemy(
    entity: &Entity,
    enemy: &Enemy,
    pos: Option<&Position>,
    #[resource] actions: &mut ActionQueue,
    #[resource] boundaries: &ggez::graphics::Rect,
) {
    // if enemy reaches the city border, damage the city and remove the enemy
    if match pos {
        None => false,
        Some(pos) => pos.y >= boundaries.h,
    } {
        actions.push((
            *entity,
            super::GameAction::TakeCityDamage { dmg: enemy.damage },
        ));
        actions.push((*entity, super::GameAction::Remove));
    }
}
