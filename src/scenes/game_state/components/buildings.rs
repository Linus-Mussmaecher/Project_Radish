use legion::{system, systems::CommandBuffer};

pub struct Building {
    building_type: usize,
}

/// A struct representing the current state of the three buildable buildings
pub struct Buildings {
    pub target: [u8; 3],
    current: [u8; 3],
}


/// A system that check wether the target for a building level fits the current level and spawns buildings and sends messages as needed.
#[system]
pub fn create_buildings(
    #[resource] buildings: &mut Buildings,
    #[resource] message_set: &mut super::super::game_message::MessageSet,
    #[resource] boundaries: &ggez::graphics::Rect,
    cmd: &mut CommandBuffer,
) {
    // if 'target' is not 'current', spawn the appropriate building and send a message

    for i in 0..3 {
        if buildings.target[i] > buildings.current[i] {
            // if building not yet built => spawn it
            if buildings.current[i] == 0 {
                cmd.push((
                    super::Position::new(
                        boundaries.w / 6. + boundaries.w * i as f32,
                        boundaries.h + 32.,
                    ),
                    Building { building_type: i },
                ));
            }
            // inform everyone
            message_set.insert(mooeye::UiMessage::Extern(
                super::super::game_message::GameMessage::BuildingUp(i, buildings.target[i]),
            ));
            // update current
            buildings.current[i] = buildings.target[i];
        } else if buildings.target[i] < buildings.current[i]{
            // inform everyone of downlevel
            message_set.insert(mooeye::UiMessage::Extern(
                super::super::game_message::GameMessage::BuildingDown(i, buildings.target[i]),
            ));
            // update
            buildings.current[i] = buildings.target[i];
        }
    }
}

#[system(for_each)]
pub fn destroy_buildings(
    #[resource] buildings: &mut Buildings,
    building: &mut Building,
    actions: &super::Actions,
) {
    if actions.get_actions().iter().any(|act| {
        matches!(
            act,
            super::actions::GameAction::Remove(super::actions::RemoveSource::HealthLoss)
        )
    }) {
        buildings.target[building.building_type] = 0;
    }
}
