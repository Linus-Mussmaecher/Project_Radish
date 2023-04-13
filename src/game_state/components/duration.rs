use legion::{systems::CommandBuffer, *};
use std::time::Duration;

#[system(for_each)]
pub fn manage_durations(
    entity: &Entity,
    duration: &mut Duration,
    commands: &mut CommandBuffer,
    #[resource] delta: &Duration,
) {
    *duration = duration.saturating_sub(*delta);
    if duration.is_zero(){
        commands.remove(*entity);
    }
}
