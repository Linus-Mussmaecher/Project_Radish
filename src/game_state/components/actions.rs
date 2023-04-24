use ggez::glam::Vec2;
use legion::{systems::CommandBuffer, Entity, EntityStore, IntoQuery, World};
use mooeye::sprite::SpritePool;
use tinyvec::TinyVec;

use super::{Enemy, Position};

#[derive(Clone)]
#[allow(dead_code)]
/// This enum contains all possible ways for entities to affect the world around them.
pub enum GameAction {
    /// No action will be taken - useful if an action transformation should delete certain actions.
    None,
    /// Removes the entity from the world.
    Remove,
    /// Manipulates the entitiets position component.
    Move { delta: Vec2 },
    /// Reduces the value of the health component.
    TakeDamage { dmg: i32 },
    /// Increases the value of the health component (but not above max).
    TakeHealing { heal: i32 },
    /// Damages the main city.
    TakeCityDamage { dmg: i32 },
    /// Increases the players available money (and score).
    GainGold { amount: i32 },
    /// Makes the entity 'other' immune to collisions with this entity
    AddImmunity { other: legion::Entity },
    /// Instructs the spell casting component to cast a certain spell
    CastSpell(usize),
    /// Executes a closure that is supposed to spawn an entity into the world. TODO: Closure evil, somehow serialize this?
    Spawn(Box<fn(Entity, Position, &SpritePool, &mut CommandBuffer)>),
    /// Executes an arbitrary closure with full access to the command buffer.
    Other(Box<fn(Entity, &mut CommandBuffer)>),
    /// Distributes an action among other entities as described by the distributor
    Distributed(Distributor, Box<GameAction>),
}

impl GameAction {
    /// Helper function to create a [GameAction::Spawn] without having to use Box.
    pub fn spawn(spawner: fn(Entity, Position, &SpritePool, &mut CommandBuffer)) -> Self {
        Self::Spawn(Box::new(spawner))
    }

    /// Helper function to create a [GameAction::Distributed] without having to use Box.
    pub fn distributed(distributor: Distributor, action: GameAction) -> Self {
        Self::Distributed(distributor, Box::new(action))
    }

    #[allow(dead_code)]
    /// Helper function to create a [GameAction::Other] without having to use Box.
    pub fn other(executor: fn(Entity, &mut CommandBuffer)) -> Self {
        Self::Other(Box::new(executor))
    }
}

impl Default for GameAction {
    fn default() -> Self {
        Self::None
    }
}

/// A component that handles an entities interaction with the world via an action queue
pub struct Actions {
    action_queue: TinyVec<[GameAction; 4]>,
}

impl Actions {
    /// Creates a new, empty Actions component
    pub fn new() -> Self {
        Self {
            action_queue: TinyVec::new(),
        }
    }

    /// Adds an action to the action queue.
    pub fn push(&mut self, action: GameAction) {
        self.action_queue.push(action);
    }

    /// Returns all currently queued actions
    pub fn get_actions(&self) -> &TinyVec<[GameAction; 4]> {
        &self.action_queue
    }

    /// Returns a mutable accessor to all currently queued actions.
    pub fn get_actions_mut(&mut self) -> &mut TinyVec<[GameAction; 4]>{
        &mut self.action_queue
    }
}

impl From<TinyVec<[GameAction; 4]>> for Actions {
    fn from(value: TinyVec<[GameAction; 4]>) -> Self {
        Self {
            action_queue: value,
        }
    }
}

#[legion::system(for_each)]
/// System that clears all actions queues.
pub fn clear(actions: &mut Actions) {
    actions.action_queue.clear();
}

#[legion::system(for_each)]
/// System that handles all spawn and other actions by executing their closures.
pub fn resolve_executive_actions(
    ent: &Entity,
    actions: &Actions,
    pos: Option<&Position>,
    #[resource] spritepool: &SpritePool,
    cmd: &mut CommandBuffer,
) {
    for action in actions.get_actions() {
        match action {
            GameAction::Spawn(spawner) => (spawner)(*ent, pos.map(|p| *p).unwrap_or_default(), spritepool, cmd),
            GameAction::Other(executor) => (executor)(*ent, cmd),
            _ => {}
        }
    }
}

#[derive(Clone, Debug)]
/// An enum that contains multiple ways to distribute an action among entities.
pub enum Distributor {
    /// Applies the action to all entities within a certain range of the original entity, possible only the first few entities and possibly restrited to only enemies.
    InRange {
        range: f32,
        limit: Option<usize>,
        enemies_only: bool,
    },
}

/// A custom system that handles [GameAction::Distributed].
pub fn distribution_system(world: &mut World) {
    // create a list of all new actions
    let mut actions_to_apply = Vec::new();

    // for every action distributor
    for (src_pos, actions) in <(&Position, &Actions)>::query().iter(world) {
        for act in actions.get_actions() {
            // get all distributor actions
            match act {
                GameAction::Distributed(distributor, action) => {
                    // get parameters of distributor
                    let (range, limit, enemies_only) = match *distributor {
                        Distributor::InRange {
                            range,
                            limit,
                            enemies_only,
                        } => (range, limit, enemies_only),
                    };

                    // keep track of applied actions
                    let mut count = 0;
                    // iterate over possible target
                    for (tar, tar_pos, tar_enemy) in
                        <(Entity, &Position, Option<&Enemy>)>::query().iter(world)
                    {
                        // if applicable
                        if src_pos.distance(*tar_pos) < range
                            && (!enemies_only || matches!(tar_enemy, Some(_)))
                            && limit.map_or(true, |lim| lim > count)
                        {
                            // remember to push action
                            actions_to_apply.push((*tar, action.clone()));
                            count += 1;
                        }
                    }
                }
                _ => {}
            }
        }
    }

    // now, push all remembered actions to their respective lists
    for (ent, action) in actions_to_apply.into_iter() {
        if let Ok(mut entry) = world.entry_mut(ent) {
            entry
                .get_component_mut::<Actions>()
                .expect("Every element should have an actions component.")
                .push(*action);
        }
    }
}
