use legion::{systems::CommandBuffer, *};
use std::time::Duration;

use crate::game_state::controller::Interactions;


pub struct LifeDuration{
    life_duration: Duration,
    max_duration: Duration,
}

impl LifeDuration{
    pub fn new(max_duration: Duration)-> Self{
        Self { life_duration: Duration::ZERO, max_duration }
    }
}

#[system(for_each)]
pub fn manage_durations(
    entity: &Entity,
    duration: &mut LifeDuration,
    commands: &mut CommandBuffer,
    #[resource] ix: &Interactions,
) {
    duration.life_duration += ix.delta;
    if duration.life_duration >= duration.max_duration {
        commands.remove(*entity);
    }
}
