use std::time::Duration;

use crate::scenes::game_state::game_data;
use legion::{system, systems::CommandBuffer};

pub enum BuildingType {
    Watchtower = 0,
    Mageguild = 1,
    Manawell = 2,
}

pub const BUILDING_MAX_LEVEL: usize = 4;
pub const BUILDING_TYPES: usize = 3;

pub struct Building {
    building_type: usize,
}

/// A struct representing the current state of the three buildable buildings
pub struct Buildings {
    pub target: [u8; BUILDING_TYPES],
    current: [u8; BUILDING_TYPES],
}

impl Buildings {
    pub fn new() -> Self {
        Self {
            target: [0; 3],
            current: [0; 3],
        }
    }
}

#[derive(Debug, Clone)]
pub struct BuildingInfo {
    pub level_costs: [u32; BUILDING_MAX_LEVEL],
    pub name: &'static str,
    pub description: &'static str,
    sprite: &'static str,
}

const BUILDING_INFO_LIST: [BuildingInfo; BUILDING_TYPES] = [
    BuildingInfo {
        level_costs: [100, 150, 200, 200],
        name: "Watchtower",
        description: "Allows you to reroll approaching enemies.",
        sprite: "/sprites/environment/watchtower",
    },
    BuildingInfo {
        level_costs: [100, 150, 200, 200],
        name: "Mage's Guild",
        description: "Allows you to purchase higher level spells.",
        sprite: "/sprites/environment/mageguild",
    },
    BuildingInfo {
        level_costs: [250, 300, 350, 400],
        name: "Mana Well",
        description: "Adds an additional spell slot per level.",
        sprite: "/sprites/environment/manawell",
    },
];

const NO_BUILDING: BuildingInfo = BuildingInfo {
    level_costs: [0; BUILDING_MAX_LEVEL],
    name: "No Building",
    description: "Does nothing. Should not exist",
    sprite: "",
};

pub fn get_building_info(building: usize) -> &'static BuildingInfo {
    BUILDING_INFO_LIST.get(building).unwrap_or(&NO_BUILDING)
}

/// A system that check wether the target for a building level fits the current level and spawns buildings and sends messages as needed.
#[system]
pub fn create_buildings(
    #[resource] data: &mut game_data::GameData,
    #[resource] message_set: &mut super::super::game_message::MessageSet,
    #[resource] boundaries: &ggez::graphics::Rect,
    cmd: &mut CommandBuffer,
) {
    // if 'target' is not 'current', spawn the appropriate building and send a message

    for (i, info) in BUILDING_INFO_LIST.iter().enumerate() {
        match data.buildings.target[i].cmp(&data.buildings.current[i]) {
            std::cmp::Ordering::Greater => {
                // if building not yet built => spawn it
                if data.buildings.current[i] == 0 {
                    cmd.push((
                        super::Position::new(
                            boundaries.w / data.buildings.target.len() as f32 / 2.
                                + boundaries.w * i as f32 / data.buildings.target.len() as f32,
                            boundaries.h + 32. + 8.,
                        ),
                        Building { building_type: i },
                        super::Collision::new(4. * 32., 2. * 32., |e1, e2| {
                            vec![
                                (
                                    e1,
                                    super::actions::GameAction::Remove(
                                        super::actions::RemoveSource::BuildingCollision,
                                    ),
                                ),
                                (
                                    e2,
                                    super::actions::GameAction::Remove(
                                        super::actions::RemoveSource::EnemyReachedBottom,
                                    ),
                                ),
                            ]
                        }),
                        super::Graphics::new(info.sprite, Duration::from_secs_f32(0.3))
                            .with_sprite_variant(2),
                    ));
                }
                // inform everyone
                message_set.insert(mooeye::UiMessage::Extern(
                    super::super::game_message::GameMessage::BuildingUp(
                        i,
                        data.buildings.target[i],
                    ),
                ));
                // update current
                data.buildings.current[i] = data.buildings.target[i];
            }
            std::cmp::Ordering::Less => {
                // inform everyone of downlevel
                message_set.insert(mooeye::UiMessage::Extern(
                    super::super::game_message::GameMessage::BuildingDown(
                        i,
                        data.buildings.target[i],
                    ),
                ));
                // update
                data.buildings.current[i] = data.buildings.target[i];
            }
            _ => {}
        }
    }
}

#[system(for_each)]
pub fn destroy_buildings(
    #[resource] data: &mut game_data::GameData,
    building: &mut Building,
    actions: &super::Actions,
) {
    if actions.get_actions().iter().any(|act| {
        matches!(
            act,
            super::actions::GameAction::Remove(super::actions::RemoveSource::BuildingCollision)
        )
    }) {
        data.buildings.target[building.building_type] = 0;
    }
}
