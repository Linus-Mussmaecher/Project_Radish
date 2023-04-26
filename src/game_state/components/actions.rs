use std::fmt::Debug;

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
    /// Adds a particle to the entities graphics struct
    AddParticle(super::graphics::Particle),
    /// Instructs the spell casting component to cast a certain spell
    CastSpell(usize),
    /// Executes a closure that is supposed to spawn an entity into the world. TODO: Closure evil, somehow serialize this?
    Spawn(Box<fn(Entity, Position, &SpritePool, &mut CommandBuffer)>),
    /// Executes an arbitrary closure with full access to the command buffer.
    Other(Box<fn(Entity, &mut CommandBuffer)>),
    /// Distributes an action among other entities as described by the distributor
    Distributed(Distributor),
    /// Applies an action repeatedly over a time period
    Repeated(Repeater),
}

impl Debug for GameAction{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Remove => write!(f, "Remove"),
            Self::Move { delta } => f.debug_struct("Move").field("delta", delta).finish(),
            Self::TakeDamage { dmg } => f.debug_struct("TakeDamage").field("dmg", dmg).finish(),
            Self::TakeHealing { heal } => f.debug_struct("TakeHealing").field("heal", heal).finish(),
            Self::TakeCityDamage { dmg } => f.debug_struct("TakeCityDamage").field("dmg", dmg).finish(),
            Self::GainGold { amount } => f.debug_struct("GainGold").field("amount", amount).finish(),
            Self::AddImmunity { other } => f.debug_struct("AddImmunity").field("other", other).finish(),
            Self::AddParticle(arg0) => f.debug_tuple("AddParticle").field(arg0).finish(),
            Self::CastSpell(arg0) => f.debug_tuple("CastSpell").field(arg0).finish(),
            Self::Spawn(_) => f.debug_tuple("Spawn").finish(),
            Self::Other(_) => f.debug_tuple("Other").finish(),
            Self::Distributed(arg0) => f.debug_tuple("Distributed").field(arg0).finish(),
            Self::Repeated(arg0) => f.debug_tuple("Repeated").field(arg0).finish(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum GameActionContainer {
    Single(Box<GameAction>),
    Multiple(Vec<GameAction>),
}

impl GameActionContainer {
    pub fn single(action: GameAction) -> Self {
        Self::Single(Box::new(action))
    }
}

macro_rules! gameaction_multiple {
    ($( $x:expr ),* $(,)?) => {
        crate::game_state::components::actions::GameActionContainer::Multiple(vec![$($x),*])
    };
}
pub(crate) use gameaction_multiple;

impl GameAction {
    /// Helper function to create a [GameAction::Spawn] without having to use Box.
    pub fn spawn(spawner: fn(Entity, Position, &SpritePool, &mut CommandBuffer)) -> Self {
        Self::Spawn(Box::new(spawner))
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
    pub fn get_actions_mut(&mut self) -> &mut TinyVec<[GameAction; 4]> {
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
            GameAction::Spawn(spawner) => {
                (spawner)(*ent, pos.map(|p| *p).unwrap_or_default(), spritepool, cmd)
            }
            GameAction::Other(executor) => (executor)(*ent, cmd),
            _ => {}
        }
    }
}

#[derive(Clone, Debug)]
/// Applies the action to all entities within a certain range of the original entity, possible only the first few entities and possibly restrited to only enemies.
pub struct Distributor {
    range: f32,
    limit: Option<usize>,
    enemies_only: bool,
    action: GameActionContainer,
}

impl Distributor{

    /// Creates a new action distributor with the specified action(s), applying to an unlimited amount of entities (implementing [components::Position]) at unlimited range
    pub fn new(action: GameActionContainer) -> Self{
        Self { range: f32::INFINITY, limit: None, enemies_only: false, action }
    }

    /// Modifies this distributor to only hit entities with the [components::Enemy] component. Returns self builder pattern style.
    pub fn with_enemies_only(mut self) -> Self{
        self.enemies_only = true;
        self
    }

    /// Modifies this distributor to only hit entities within a certain range. Returns self builder pattern style.
    pub fn with_range(mut self, range: f32) -> Self{
        self.range = range;
        self
    }

    /// Modifies this distributor to only hit entities a limited amount of entities (sorted by range). Returns self builder pattern style.
    pub fn with_limit(mut self, limit: usize) -> Self{
        self.limit = Some(limit);
        self
    }

    /// Turns this distributor into a [GameAction::Distributed] containing it.
    pub fn to_action(self) -> GameAction{
        GameAction::Distributed(self)
    }
}

#[derive(Clone, Debug)]
pub struct Repeater {}

/// A custom system that handles [GameAction::Distributed].
pub fn distribution_system(world: &mut World) {
    // create a list of all new actions
    let mut total_actions = Vec::new();

    // for every action distributor
    for (src_pos, actions) in <(&Position, &Actions)>::query().iter(world) {
        for act in actions.get_actions() {
            // get all distributor actions
            match act {
                GameAction::Distributed(distributor) => {
                    let mut target_list = Vec::new();
                    // iterate over possible target
                    for (tar, tar_pos, tar_enemy) in
                        <(Entity, &Position, Option<&Enemy>)>::query().iter(world)
                    {
                        // if applicable
                        if src_pos.distance(*tar_pos) < distributor.range
                            && (!distributor.enemies_only || matches!(tar_enemy, Some(_)))
                            //&& distributor.limit.map_or(true, |lim| lim > count)
                        {
                            // remember to push action
                            target_list.push((*tar, src_pos.distance(*tar_pos)));
                        }
                    }

                    target_list.sort_by(|(_, d1), (_, d2)| d1.total_cmp(d2));

                    for (target, _) in target_list.drain(match distributor.limit{
                        Some(x) => 0..x,
                        None => 0..target_list.len(),
                    }){
                        total_actions.push((target, distributor.action.clone()))
                    }
                }
                _ => {}
            }
        }
    }

    // now, push all remembered actions to their respective lists
    for (ent, action) in total_actions.into_iter() {
        if let Ok(mut entry) = world.entry_mut(ent) {
            if let Ok(action_comp) = entry.get_component_mut::<Actions>() {
                match action {
                    GameActionContainer::Single(action) => action_comp.push(*action),
                    GameActionContainer::Multiple(action_vec) => {
                        action_comp.get_actions_mut().extend(action_vec)
                    }
                }
            }
        }
    }
}
