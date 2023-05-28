use std::time::Duration;

use legion::{system, systems::CommandBuffer, Entity};

use super::super::game_message;

use super::*;

/// The Health component track wether a unit has a life bar and can take damage.
pub struct Health {
    curr_health: i32,
    max_health: i32,
    snapshot_health: f32,
    snapshot_delay: Duration,
}

impl Health {
    /// Creates a new health struct with both the maximum and current health set to the passed value.
    pub fn new(health: i32) -> Self {
        Self {
            curr_health: health,
            max_health: health,
            snapshot_health: health as f32,
            snapshot_delay: Duration::ZERO,
        }
    }

    /// Returns the unis current health.
    pub fn get_current_health(&self) -> i32 {
        self.curr_health.max(0)
    }

    /// Returns the unis maximum health.
    pub fn get_max_health(&self) -> i32 {
        self.max_health.max(0)
    }

    /// Returns the units health before the last damage chain
    pub fn get_snapshot(&self) -> f32 {
        self.snapshot_health.max(0.)
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

#[system(for_each)]
/// Removes entities with zero health or less
pub fn remove_entities(entity: &Entity, actions: &Actions, cmd: &mut CommandBuffer) {
    if actions
        .get_actions()
        .iter()
        .any(|act| matches!(*act, actions::GameAction::Remove(_)))
    {
        cmd.remove(*entity);
    }
}

#[system(for_each)]
/// A system that removes entities when reaching 0 health and triggers a variety of death-related effects.
pub fn destroy_by_health(
    health: &Health,
    enemy: Option<&Enemy>,
    actions: &mut Actions,
    #[resource] messages: &mut game_message::MessageSet,
) {
    if health.curr_health <= 0 {
        // in case of enemies
        if let Some(enemy) = enemy {
            // gain gold
            actions.push(actions::GameAction::GainGold {
                amount: enemy.bounty,
            });
            messages.insert(mooeye::UiMessage::Extern(
                game_message::GameMessage::EnemyKilled(enemy.bounty),
            ));
        }

        actions.push(actions::GameAction::Remove(
            actions::RemoveSource::HealthLoss,
        ));
    }
}

#[system(for_each)]
pub fn enemy_death_sprite(
    _enemy: &Enemy,
    pos: &Position,
    vel: Option<&Velocity>,
    gfx: &Graphics,
    actions: &Actions,
    cmd: &mut CommandBuffer,
) {
    // add death animation
    if actions.get_actions().iter().any(|act| {
        matches!(
            *act,
            actions::GameAction::Remove(actions::RemoveSource::HealthLoss)
        )
    }) {
        cmd.push((
            *pos,
            vel.map(|v| Velocity::new((f32::EPSILON).copysign(v.get_dx()), 0.))
                .unwrap_or(Velocity::new(0., 0.)),
            LifeDuration::new(
                gfx.sprite.get_cycle_time() - gfx.sprite.get_frame_time(),
            ),
            {
                let mut death_sprite = gfx.sprite.clone();
                death_sprite.set_variant(1);
                Graphics::from(death_sprite)
            },
        ));
    }
}

#[system(for_each)]
/// Applies all [TakeDamage] actions to their respective entities.
pub fn resolve_damage(
    health: &mut Health,
    actions: &Actions,
    #[resource] ix: &super::super::controller::Interactions,
) {
    // update snapshot health
    let health_float = health.curr_health as f32;
    if health_float < health.snapshot_health && health.snapshot_delay.is_zero() {
        health.snapshot_health =
            health_float.max(health.snapshot_health - ix.delta.as_secs_f32() * 50.);
    } else {
        health.snapshot_delay = health.snapshot_delay.saturating_sub(ix.delta);
    }

    for action in actions.get_actions() {
        if let actions::GameAction::TakeDamage { dmg } = action {
            health.curr_health -= *dmg;
            health.snapshot_delay = Duration::from_secs_f32(1.5);
        } else if let actions::GameAction::TakeHealing { heal } = action {
            health.curr_health = (health.curr_health + *heal).min(health.max_health);
            health.snapshot_health = health.curr_health as f32;
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
        actions.push(actions::GameAction::TakeCityDamage { dmg: enemy.damage });
        actions.push(actions::GameAction::Remove(actions::RemoveSource::EnemyReachedBottom));
    }
}
