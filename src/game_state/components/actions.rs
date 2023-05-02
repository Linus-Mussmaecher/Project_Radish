use std::{fmt::Debug, time::Duration};

use ggez::glam::Vec2;
use legion::{system, systems::CommandBuffer, Entity, EntityStore, IntoQuery};
use mooeye::sprite;
use tinyvec::TinyVec;

use crate::game_state::controller;

use super::{Enemy, Position};

#[derive(Clone)]
#[allow(dead_code)]
/// This enum contains all possible ways for entities to affect the world around them.
pub enum GameAction {
    /// No action will be taken - useful if an action transformation should delete certain actions.
    None,
    /// Removes the entity from the world.
    Remove(RemoveSource),
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
    Spawn(Box<fn(Entity, Position, &sprite::SpritePool, &mut CommandBuffer)>),
    /// Executes an arbitrary closure with full access to the command buffer.
    Other(Box<fn(Entity, &mut CommandBuffer)>),
    /// Distributes an action among other entities as described by the distributor
    Distributed(Box<Distributor>),
    /// Applies an action repeatedly over a time period
    Repeated(Box<Repeater>),
}

impl Debug for GameAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Remove(arg0) => f.debug_tuple("Remove").field(arg0).finish(),
            Self::Move { delta } => f.debug_struct("Move").field("delta", delta).finish(),
            Self::TakeDamage { dmg } => f.debug_struct("TakeDamage").field("dmg", dmg).finish(),
            Self::TakeHealing { heal } => {
                f.debug_struct("TakeHealing").field("heal", heal).finish()
            }
            Self::TakeCityDamage { dmg } => {
                f.debug_struct("TakeCityDamage").field("dmg", dmg).finish()
            }
            Self::GainGold { amount } => {
                f.debug_struct("GainGold").field("amount", amount).finish()
            }
            Self::AddImmunity { other } => {
                f.debug_struct("AddImmunity").field("other", other).finish()
            }
            Self::AddParticle(arg0) => f.debug_tuple("AddParticle").field(arg0).finish(),
            Self::CastSpell(arg0) => f.debug_tuple("CastSpell").field(arg0).finish(),
            Self::Spawn(_) => f.debug_tuple("Spawn").finish(),
            Self::Other(_) => f.debug_tuple("Other").finish(),
            Self::Distributed(arg0) => f.debug_tuple("Distributed").field(arg0).finish(),
            Self::Repeated(arg0) => f.debug_tuple("Repeated").field(arg0).finish(),
        }
    }
}

impl GameAction {
    /// Helper function to create a [GameAction::Spawn] without having to use Box.
    pub fn spawn(spawner: fn(Entity, Position, &sprite::SpritePool, &mut CommandBuffer)) -> Self {
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

#[derive(Debug, Clone, Copy)]
/// An enum that is carried by remove actions to inform the remover of the source triggering the removal
pub enum RemoveSource {
    /// This entity has to be removed because it has reached zero health
    HealthLoss,
    /// This entity has to be removed because its [components::LifeDuration] has run out.
    TimedOut,
    /// Any other reasong for removal.
    Other,
}

#[derive(Clone, Debug)]
/// Object that can contain either a single or a vector of GameActions. Mostly used to abstract away boxes and vecs from the user.
pub enum GameActionContainer {
    // Contains only a single GameAction.
    Single(GameAction),
    // Contains multiple GameActions in a vector.
    Multiple(Vec<GameAction>),
}

impl GameActionContainer {
    /// Creates a new GameActionContainer with only a single element.
    pub fn single(action: GameAction) -> Self {
        Self::Single(action)
    }
}

/// Contructs a new GameActionContainer with multiple elements, similar to the vec! macro.
macro_rules! gameaction_multiple {
    ($( $x:expr ),* $(,)?) => {
        crate::game_state::components::actions::GameActionContainer::Multiple(vec![$($x),*])
    };
}
pub(crate) use gameaction_multiple;

impl From<GameAction> for GameActionContainer {
    fn from(value: GameAction) -> Self {
        Self::Single(value)
    }
}

/// A component that handles an entities interaction with the world via an action queue
pub struct Actions {
    action_queue: TinyVec<[GameAction; 4]>,
    repeat_actions: TinyVec<[Repeater; 4]>,
}

impl Actions {
    /// Creates a new, empty Actions component
    pub fn new() -> Self {
        Self {
            action_queue: TinyVec::new(),
            repeat_actions: TinyVec::new(),
        }
    }

    /// Adds an action to the action queue.
    pub fn push(&mut self, action: GameAction) {
        self.action_queue.push(action);
    }

    pub fn add(&mut self, actions: GameActionContainer) {
        match actions {
            GameActionContainer::Single(act) => self.action_queue.push(act),
            GameActionContainer::Multiple(act_vec) => self.action_queue.extend(act_vec),
        }
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

#[legion::system(for_each)]
/// System that clears all actions queues.
pub fn clear(actions: &mut Actions) {
    // if actions.action_queue.len() > 4 {
    //     println!("Cache miss! Queue length: {}", actions.action_queue.len());
    // }

    actions.action_queue.clear();
}

#[system(for_each)]
/// System that handles all spawn and other actions by executing their closures.
pub fn resolve_executive_actions(
    ent: &Entity,
    actions: &Actions,
    pos: Option<&Position>,
    #[resource] spritepool: &sprite::SpritePool,
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

impl Distributor {
    /// Creates a new action distributor with the specified action(s), applying to an unlimited amount of entities (implementing [components::Position]) at unlimited range
    pub fn new(action: GameActionContainer) -> Self {
        Self {
            range: f32::INFINITY,
            limit: None,
            enemies_only: false,
            action,
        }
    }

    /// Modifies this distributor to only hit entities with the [components::Enemy] component. Returns self builder pattern style.
    pub fn with_enemies_only(mut self) -> Self {
        self.enemies_only = true;
        self
    }

    /// Modifies this distributor to only hit entities within a certain range. Returns self builder pattern style.
    pub fn with_range(mut self, range: f32) -> Self {
        self.range = range;
        self
    }

    /// Modifies this distributor to only hit entities a limited amount of entities (sorted by range). Returns self builder pattern style.
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Turns this distributor into a [GameAction::Distributed] containing it.
    pub fn to_action(self) -> GameAction {
        GameAction::Distributed(Box::new(self))
    }
}

/// A custom system that handles [GameAction::Distributed].
pub fn distribution_system(world: &mut legion::World) {
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
                        {
                            // remember to push action
                            target_list.push((*tar, src_pos.distance(*tar_pos)));
                        }
                    }

                    target_list.sort_by(|(_, d1), (_, d2)| d1.total_cmp(d2));

                    for (target, _) in target_list.drain(match distributor.limit {
                        Some(x) => 0..(x.min(target_list.len())),
                        None => 0..target_list.len(),
                    }) {
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
                action_comp.add(action);
            }
        }
    }
}

#[derive(Clone, Debug)]
/// A struct that repeatedly or delayed applies a (set of) action(s) to its carrier.
pub struct Repeater {
    total_duration: Option<Duration>,
    alive_duration: Duration,
    last_activation_duration: Duration,
    repeat_duration: Duration,
    action: GameActionContainer,
}

impl Repeater {
    /// Creates a new repeater that applies the passed action once every second for ever.
    pub fn new(action: GameActionContainer) -> Self {
        Self {
            total_duration: None,
            alive_duration: Duration::ZERO,
            repeat_duration: Duration::from_secs(1),
            last_activation_duration: Duration::ZERO,
            action,
        }
    }

    /// Modifies this repeater to be removed after a certain duration has passed. Returns itself builder pattern style.
    pub fn with_total_duration(mut self, total_duration: Duration) -> Self {
        self.total_duration = Some(total_duration);
        self
    }

    /// Modifies this repeater to be repeatedly apply the action. Returns itself builder pattern style.
    pub fn with_repeat_duration(mut self, repeat_duration: Duration) -> Self {
        self.repeat_duration = repeat_duration;
        self
    }

    /// Modifies the repeater so it applies its action once after the passed duration and then removes itself.
    pub fn with_once_after(mut self, duration: Duration) -> Self {
        self.repeat_duration = duration;
        self.total_duration = Some(duration);
        self
    }

    /// Turns this distributor into a [GameAction::Repeated] containing it.
    pub fn to_action(self) -> GameAction {
        GameAction::Repeated(Box::new(self))
    }
}

impl Default for Repeater {
    fn default() -> Self {
        Self {
            total_duration: Default::default(),
            alive_duration: Default::default(),
            repeat_duration: Default::default(),
            last_activation_duration: Default::default(),
            action: GameActionContainer::single(GameAction::None),
        }
    }
}

#[system(for_each)]
/// A system that takes all Repeated actions and add their repeaters to the repeater list.
pub fn register_repeaters(actions: &mut Actions, #[resource] ix: &controller::Interactions) {
    for action in actions.action_queue.iter() {
        if let GameAction::Repeated(repeater) = action {
            let mut rep = *repeater.clone();
            rep.alive_duration += ix.delta;
            rep.last_activation_duration += ix.delta;
            actions.repeat_actions.push(rep);
        }
    }
}

#[system(for_each)]
/// A system that regularly looks at all repeaters and triggers their actions, progresses their durations and removes them if appropriate.
pub fn handle_repeaters(actions: &mut Actions, #[resource] ix: &controller::Interactions) {
    // iterate over repeaters
    for repeater in actions.repeat_actions.iter_mut() {
        // Increase counting durations
        repeater.alive_duration += ix.delta;
        repeater.last_activation_duration += ix.delta;

        // Check if last activation is longer ago then planned repeat duration
        while repeater.last_activation_duration >= repeater.repeat_duration {
            // decrease last activation duration (thus, if the delta was longer than multiple repeat intervalls, the while will trigger again and additional duration will be kept for next frame)
            repeater.last_activation_duration -= repeater.repeat_duration;
            // push appropriate action
            match &repeater.action {
                GameActionContainer::Single(act) => actions.action_queue.push(act.clone()),
                GameActionContainer::Multiple(ac_vec) => {
                    for act in ac_vec {
                        actions.action_queue.push(act.clone())
                    }
                }
            }
        }
    }

    actions
        .repeat_actions
        .retain(|rep| match rep.total_duration {
            Some(total_dur) => total_dur > rep.alive_duration,
            None => true,
        });
}
