use legion::{systems::CommandBuffer, *};
use mooeye::sprite::Sprite;

use crate::game_state::game_action::ActionQueue;

use super::{Position, LifeDuration};

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

/// A struct that contains a closure that can access a command buffer of the world on death.
/// Preferably, that closure does not mutate world, but uses its state to inform the action queue of taken actions.
pub struct OnDeath {
    deathrattle: Box<dyn Fn(&mut CommandBuffer, Entity) + Send + Sync>,
}

impl OnDeath {
    /// Creates a new OnDeath component. The carrying entity will trigger the passed closure when its health reaches 0.
    pub fn new(deathrattle: impl Fn(&mut CommandBuffer, Entity) + 'static + Send + Sync) -> Self {
        Self {
            deathrattle: Box::new(deathrattle),
        }
    }
}

#[system(for_each)]
/// Removes entities with zero health or less
pub fn remove_entities(
    entity: &Entity,
    #[resource] actions: &ActionQueue,
    cmd: &mut CommandBuffer,
) {
    if actions.contains(&(*entity, super::GameAction::Remove)) {
        cmd.remove(*entity);
    }
}

#[system(for_each)]
//
pub fn destroy_by_health(
    ent: &Entity,
    health: &Health,
    enemy: Option<&Enemy>,
    sprite: Option<&Sprite>,
    pos: Option<&Position>,
    on_death: Option<&OnDeath>,
    #[resource] actions: &mut ActionQueue,
    cmd: &mut CommandBuffer,
) {
    if health.curr_health <= 0 {
        // in case of enemies
        if let Some(enemy) = enemy {
            // gain gold
            actions.push((
                *ent,
                super::GameAction::GainGold {
                    amount: enemy.bounty,
                },
            ));
            if let Some(sprite) = sprite{
                    cmd.push((
                        pos.map(|p| *p).unwrap_or_default(), 
                        LifeDuration::new(sprite.get_cycle_time() - sprite.get_frame_time()),
                        {
                            let mut death_sprite = sprite.clone();
                            death_sprite.set_variant(1);
                            death_sprite
                        }
                    ));
            }
        }

        actions.push((*ent, super::GameAction::Remove));

        // death rattle
        if let Some(on_death) = on_death {
            (on_death.deathrattle)(cmd, *ent);
        }
    }
}

#[system(for_each)]
/// Applies all [TakeDamage] actions to their respective entities.
pub fn resolve_damage(ent: &Entity, health: &mut Health, #[resource] actions: &ActionQueue) {
    for action in actions.iter() {
        if let (entity, super::GameAction::TakeDamage { dmg }) = action {
            if *entity == *ent {
                health.curr_health -= *dmg;
            }
        } else if let (entity, super::GameAction::TakeHealing { heal }) = action {
            if *entity == *ent {
                health.curr_health += *heal;
            }
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
