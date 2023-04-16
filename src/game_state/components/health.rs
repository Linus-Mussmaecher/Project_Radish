use legion::{*, systems::CommandBuffer};

use crate::game_state::game_action::ActionQueue;

/// The Health component track wether a unit has a life bar and can take damage.
pub struct Health{
    curr_health: i32,
    max_health: i32,
}

impl Health{
    /// Creates a new health struct with both the maximum and current health set to the passed value.
    pub fn new(health: i32) -> Self{
        Self { curr_health: health, max_health: health }
    }

    /// Returns the unis current health.
    pub fn get_current_health(&self)-> i32{
        self.curr_health
    }

    /// Returns the unis maximum health.
    pub fn get_max_health(&self) -> i32{
        self.max_health
    }
}

#[system(for_each)]
/// Removes entities with zero health or less
pub fn remove_dead( entity: &Entity, health: &Health, commands: &mut CommandBuffer ){
    if health.curr_health <= 0 {
        commands.remove(*entity);
    }
}

#[system(for_each)]
/// Applies all [TakeDamage] actions to their respective entities.
pub fn resolve_damage (ent: &Entity, health: &mut Health, #[resource] actions: &ActionQueue){
    for action in actions{
        if let (entity, super::GameAction::TakeDamage { dmg }) = action{
            if *entity == *ent {
                health.curr_health -= *dmg;
            }
        }
    }
}