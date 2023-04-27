use legion::*;
use std::time::Duration;

use crate::game_state::controller::Interactions;

use super::{actions::GameAction, Actions};

/// A component that keeps a life duration of an entity and removes it after a certain time.
pub struct LifeDuration {
    life_duration: Duration,
    max_duration: Duration,
}

impl LifeDuration {
    /// Creates a new Duration component.
    pub fn new(max_duration: Duration) -> Self {
        Self {
            life_duration: Duration::ZERO,
            max_duration,
        }
    }
}

impl From<Duration> for LifeDuration {
    fn from(value: Duration) -> Self {
        LifeDuration::new(value)
    }
}

#[system(for_each)]
/// A system that increases the timer on all durations and removes elements if neccessary.
pub fn manage_durations(
    duration: &mut LifeDuration,
    actions: &mut Actions,
    #[resource] ix: &Interactions,
) {
    duration.life_duration += ix.delta;
    if duration.life_duration >= duration.max_duration {
        actions.push(GameAction::Remove)
    }
}
