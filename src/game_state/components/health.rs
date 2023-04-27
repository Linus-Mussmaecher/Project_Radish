use legion::{systems::CommandBuffer, *};

use crate::game_state::game_message::MessageSet;

use super::{actions::*, Actions, Graphics, LifeDuration, Position};

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

/// The Enemy struct is both a marker struct for many interactions and contains the damage an enemy deals to the main objective and the bounty it provides on kill.
pub struct Enemy {
    /// The damage this enemy deals to the main objective if it reaches the finish line.
    damage: i32,
    /// The bounty this enemy grants on kill.
    bounty: i32,
}

impl Enemy {
    /// Creates a new enemy component.
    pub fn new(damage: i32, bounty: i32) -> Self {
        Self { damage, bounty }
    }
}

/// A struct that contains a actions and messages send by an entity on death.
pub struct OnDeath {
    death_actions: GameActionContainer,
    death_messages: MessageSet,
}

impl OnDeath {
    /// Creates a new OnDeath component. The carrying entity will trigger the passed closure when its health reaches 0.
    pub fn new(death_actions: GameActionContainer, death_messages: MessageSet) -> Self {
        Self {
            death_actions,
            death_messages,
        }
    }
}

#[system(for_each)]
/// Removes entities with zero health or less
pub fn remove_entities(entity: &Entity, actions: &Actions, cmd: &mut CommandBuffer) {
    if actions
        .get_actions()
        .iter()
        .any(|act| matches!(*act, GameAction::Remove))
    {
        cmd.remove(*entity);
    }
}

#[system(for_each)]
/// A system that removes entities when reaching 0 health and triggers a variety of death-related effects.
pub fn destroy_by_health(
    health: &Health,
    enemy: Option<&Enemy>,
    gfx: Option<&Graphics>,
    pos: Option<&Position>,
    on_death: Option<&OnDeath>,
    actions: &mut Actions,
    cmd: &mut CommandBuffer,
    #[resource] messages: &mut MessageSet,
) {
    if health.curr_health <= 0 {
        // in case of enemies
        if let Some(enemy) = enemy {
            // gain gold
            actions.push(GameAction::GainGold {
                amount: enemy.bounty,
            });
            // add death animation
            if let Some(gfx) = gfx {
                cmd.push((
                    pos.map(|p| *p).unwrap_or_default(),
                    LifeDuration::new(
                        gfx.get_sprite().get_cycle_time() - gfx.get_sprite().get_frame_time(),
                    ),
                    {
                        let mut death_sprite = gfx.get_sprite().clone();
                        death_sprite.set_variant(1);
                        Graphics::from(death_sprite)
                    },
                ));
            }
        }

        actions.push(GameAction::Remove);

        // death rattle
        if let Some(on_death) = on_death {
            actions.add(on_death.death_actions.clone());
            messages.extend(on_death.death_messages.clone());
        }
    }
}

#[system(for_each)]
/// Applies all [TakeDamage] actions to their respective entities.
pub fn resolve_damage(health: &mut Health, actions: &Actions) {
    for action in actions.get_actions() {
        if let GameAction::TakeDamage { dmg } = action {
            health.curr_health -= *dmg;
        } else if let GameAction::TakeHealing { heal } = action {
            health.curr_health = (health.curr_health + *heal).min(health.max_health);
        }
    }
}

#[system(for_each)]
/// Checks if an enemy has reached the finish and removes it while dealing damage to the main objective if that is the case.
pub fn enemy(
    enemy: &Enemy,
    pos: Option<&Position>,
    actions: &mut Actions,
    #[resource] boundaries: &ggez::graphics::Rect,
) {
    // if enemy reaches the city border, damage the city and remove the enemy
    if match pos {
        None => false,
        Some(pos) => pos.y >= boundaries.h,
    } {
        actions.push(GameAction::TakeCityDamage { dmg: enemy.damage });
        actions.push(GameAction::Remove);
    }
}
