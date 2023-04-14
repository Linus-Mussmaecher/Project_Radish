use legion::{*, systems::CommandBuffer};

use crate::game_state::game_action::ActionQueue;
pub struct Health(pub i32);

#[system(for_each)]
pub fn remove_dead( entity: &Entity, health: &Health, commands: &mut CommandBuffer ){
    if health.0 <= 0 {
        commands.remove(*entity);
    }
}

#[system(for_each)]
pub fn take_damage (ent: &Entity, health: &mut Health, #[resource] actions: &ActionQueue){
    for action in actions{
        if let (entity, super::GameAction::TakeDamage { dmg }) = action{
            if *entity == *ent {
                health.0 -= *dmg;
            }
        }
    }
}