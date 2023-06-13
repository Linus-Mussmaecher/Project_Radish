use std::time::Duration;

use legion::{system, systems::CommandBuffer};

pub struct Building {
    building_type: usize,
}

/// A struct representing the current state of the three buildable buildings
pub struct Buildings {
    pub target: [u8; 3],
    current: [u8; 3],
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
pub struct BuildingInfo{
    pub level_costs: [u32; 6],
    pub name: &'static str,
    pub description: &'static str,
    sprite: &'static str,
}

const BUILDING_INFO_LIST: [BuildingInfo; 3] = [
    BuildingInfo{
        level_costs: [100, 150, 200, 200, 200, 200],
        name: "Watchtower",
        description: "Allows you to reroll approaching enemies.",
        sprite: "/sprites/environment/watchtower",
    },
    BuildingInfo{
        level_costs: [100, 150, 200, 200, 200, 200],
        name: "Mage's Guild",
        description: "Allows you to purchase higher level spells.",
        sprite: "/sprites/environment/mageguild",
    },
    BuildingInfo{
        level_costs: [250, 250, 300, 300, 400, 400],
        name: "Mana Well",
        description: "Adds an additional spell slot per level.",
        sprite: "/sprites/environment/manawell",
    },
];

const NO_BUILDING: BuildingInfo = BuildingInfo{
    level_costs: [0; 6],
    name: "No Building",
    description: "Does nothing. Should not exist",
    sprite: "",
};

pub fn get_building_info(building: usize) -> &'static BuildingInfo{
    BUILDING_INFO_LIST.get(building).unwrap_or(&NO_BUILDING)
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

    for i in 0..buildings.target.len() {
        if buildings.target[i] > buildings.current[i] {
            // if building not yet built => spawn it
            if buildings.current[i] == 0 {
                cmd.push((
                    super::Position::new(
                        boundaries.w / buildings.target.len() as f32 / 2.
                            + boundaries.w * i as f32 / buildings.target.len() as f32,
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
                    super::Graphics::new(BUILDING_INFO_LIST[i].sprite, Duration::from_secs_f32(0.3))
                        .with_sprite_variant(2),
                ));
            }
            // inform everyone
            message_set.insert(mooeye::UiMessage::Extern(
                super::super::game_message::GameMessage::BuildingUp(i, buildings.target[i]),
            ));
            // update current
            buildings.current[i] = buildings.target[i];
        } else if buildings.target[i] < buildings.current[i] {
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
            super::actions::GameAction::Remove(super::actions::RemoveSource::BuildingCollision)
        )
    }) {
        buildings.target[building.building_type] = 0;
    }
}
