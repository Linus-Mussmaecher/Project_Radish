use std::{fmt::Debug, sync::Arc, time::Duration};

use ggez::glam::Vec2;
use legion::{system, systems::CommandBuffer, Entity, EntityStore, IntoQuery};
use mooeye::sprite;
use tinyvec::TinyVec;

use crate::game_state::controller::Interactions;

use super::{Enemy, Position};

#[derive(Clone, Debug)]
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
    Spawn(SpawnerBox),
    /// Applies a (temporary or permanent) effect to the target
    ApplyEffect(Box<ActionEffect>),
}

#[derive(Clone)]
pub struct SpawnerBox {
    spawner: Box<fn(Entity, Position, &sprite::SpritePool, &mut CommandBuffer)>,
}

impl Debug for SpawnerBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpawnerBox").finish()
    }
}

impl GameAction {
    /// Helper function to create a [GameAction::Spawn] without having to use Box.
    pub fn spawn(spawner: fn(Entity, Position, &sprite::SpritePool, &mut CommandBuffer)) -> Self {
        Self::Spawn(SpawnerBox {
            spawner: Box::new(spawner),
        })
    }

    // /// Helper function to create a [GameAction::Other] without having to use Box.
    // pub fn other(executor: fn(Entity, &mut CommandBuffer)) -> Self {
    //     Self::Other(CommandBox{ command: Box::new(executor)})
    // }
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

#[derive(Debug, Clone)]
/// A struct that can be applied to an entity and represents a temporary effect on that entity
pub struct ActionEffect {
    /// what is affected by this effect
    target: ActionEffectTarget,
    /// the effect itself
    content: ActionEffectType,
    /// how long this effect lasts
    duration: Option<Duration>,
    /// how long this effect has been alive
    alive_duration: Duration,
}

impl ActionEffect {
    pub fn transform(target: ActionEffectTarget, transform: fn(&mut GameAction)) -> Self {
        Self {
            target: target,
            content: ActionEffectType::Transform(ActionTransformer::new(transform)),
            duration: None,
            alive_duration: Duration::ZERO,
        }
    }

    pub fn repeat(target: ActionEffectTarget, actions: impl Into<ActionContainer>, interval: Duration) -> Self {
        Self {
            target,
            content: ActionEffectType::Repeat {
                actions: actions.into(),
                interval,
                activations: 0.,
            },
            duration: None,
            alive_duration: Duration::ZERO,
        }
    }

    pub fn once(target: ActionEffectTarget, actions: impl Into<ActionContainer>) -> Self {
        Self {
            target,
            content: ActionEffectType::Once(actions.into()),
            duration: Some(Duration::ZERO),
            alive_duration: Duration::ZERO,
        }
    }

    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = Some(duration);
        self
    }
}

impl Default for ActionEffect {
    fn default() -> Self {
        Self {
            target: ActionEffectTarget::new_only_self(),
            content: ActionEffectType::Once(GameAction::None.into()),
            duration: None,
            alive_duration: Duration::ZERO,
        }
    }
}

impl From<ActionEffect> for GameAction {
    fn from(value: ActionEffect) -> Self {
        GameAction::ApplyEffect(Box::new(value))
    }
}

#[derive(Debug, Clone)]
/// The different types of effects that can appear
enum ActionEffectType {
    Transform(ActionTransformer),
    Repeat {
        actions: ActionContainer,
        interval: Duration,
        activations: f32,
    },
    Once(ActionContainer),
}

#[derive(Debug, Clone, Copy)]
/// An enum that describes what targets to distribute an effect or ActionModification to.
pub struct ActionEffectTarget {
    pub enemies_only: bool,
    pub range: f32,
    pub affect_self: bool,
    pub limit: Option<usize>,
}

impl ActionEffectTarget {
    pub fn new() -> Self {
        Self {
            range: f32::INFINITY,
            limit: None,
            enemies_only: false,
            affect_self: false,
        }
    }

    /// Creates a new acton target that only affects the source of the action.
    pub fn new_only_self() -> Self {
        Self {
            enemies_only: false,
            range: 0.,
            affect_self: true,
            limit: Some(1),
        }
    }

    /// Modifies this target to only hit entities with the [components::Enemy] component. Returns self builder pattern style.
    pub fn with_enemies_only(mut self, val: bool) -> Self {
        self.enemies_only = val;
        self
    }

    /// Modifies this target to also apply to the source component. Returns self builder pattern style.
    pub fn with_affect_self(mut self, val: bool) -> Self {
        self.affect_self = val;
        self
    }

    /// Modifies this target to only hit entities within a certain range. Returns self builder pattern style.
    pub fn with_range(mut self, range: f32) -> Self {
        self.range = range;
        self
    }

    /// Modifies this distributor to only hit entities a limited amount of entities (sorted by range). Returns self builder pattern style.
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }
}

#[derive(Clone)]
pub struct ActionTransformer {
    pub transform: Arc<fn(&mut GameAction)>,
}

impl ActionTransformer {
    /// Creates a new GameActionContainer transforming all actions on a target.
    pub fn new(transform: fn(&mut GameAction)) -> Self {
        ActionTransformer {
            transform: Arc::new(transform),
        }
    }
}

impl Debug for ActionTransformer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ActionTransformer").finish()
    }
}

#[derive(Clone, Debug)]
/// Object that can contain either a single or a vector of GameActions. Mostly used to abstract away boxes and vecs from the user.
pub enum ActionContainer {
    /// Contains only a single GameAction.
    ApplySingle(GameAction),
    /// Contains multiple GameActions in a vector.
    ApplyMultiple(Vec<GameAction>),
}

impl From<GameAction> for ActionContainer {
    fn from(value: GameAction) -> Self {
        Self::ApplySingle(value)
    }
}

impl From<ActionEffect> for ActionContainer {
    fn from(value: ActionEffect) -> Self {
        ActionContainer::ApplySingle(GameAction::ApplyEffect(Box::new(value)))
    }
}

impl From<Vec<GameAction>> for ActionContainer{
    fn from(value: Vec<GameAction>) -> Self {
        ActionContainer::ApplyMultiple(value)
    }
}

/// A component that handles an entities interaction with the world via an action queue
pub struct Actions {
    action_queue: TinyVec<[GameAction; 4]>,
    effects: TinyVec<[ActionEffect; 4]>,
}

impl Actions {
    /// Creates a new, empty Actions component
    pub fn new() -> Self {
        Self {
            action_queue: TinyVec::new(),
            effects: TinyVec::new(),
        }
    }

    pub fn with_effect(mut self, effect: ActionEffect) -> Self {
        self.effects.push(effect);
        self
    }

    /// Adds an action to the action queue.
    pub fn push(&mut self, action: GameAction) {
        // immediately register effects
        if let GameAction::ApplyEffect(effect) = action {
            self.effects.push(*effect);
        }
        // ignore None
        else if !matches!(action, GameAction::None){
            self.action_queue.push(action);
        }
    }

    pub fn add(&mut self, actions: ActionContainer) {
        match actions {
            ActionContainer::ApplySingle(act) => self.push(act),
            ActionContainer::ApplyMultiple(act_vec) => for act in act_vec{
                self.push(act);
            },
        }
    }

    pub fn transform(&mut self, transform: &fn(&mut GameAction)) {
        for action in self.action_queue.iter_mut() {
            transform(action);
        }
    }

    /// Returns all currently queued actions
    pub fn get_actions(&self) -> &TinyVec<[GameAction; 4]> {
        &self.action_queue
    }
}

#[legion::system(for_each)]
/// System that clears all actions queues.
pub fn clear(actions: &mut Actions) {
    // clear action queue
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
                (spawner.spawner)(*ent, pos.map(|p| *p).unwrap_or_default(), spritepool, cmd)
            }
            _ => {}
        }
    }
}

/// A special system that applies all aura-components that transform actions of other entities
pub fn handle_effects(world: &mut legion::World, resources: &mut legion::Resources) {
    // get interactions
    let ix = resources
        .get::<Interactions>()
        .expect("Could not unpack interactions.");

    // compile a list of all transforms & applies affecting entities
    let mut transforms = Vec::new();
    let mut applies = Vec::new();

    // iterate over all sources of effects
    for (src_ent, src_pos, src_act) in <(Entity, &Position, &Actions)>::query().iter(world) {
        // iterate over all their effects
        for effect in src_act.effects.iter() {
            // generate a target list of entities affected
            let mut target_list = Vec::new();
            for (tar_ent, tar_pos, tar_ene) in
                <(Entity, &Position, Option<&Enemy>)>::query().iter(world)
            {
                if src_pos.distance(*tar_pos) <= effect.target.range
                    && (!effect.target.enemies_only || tar_ene.is_some())
                    && (effect.target.affect_self || *src_ent != *tar_ent)
                {
                    target_list.push((*tar_ent, src_pos.distance(*tar_pos)));
                }
            }

            // sort target list by distance
            target_list.sort_by(|(_, d1), (_, d2)| d1.total_cmp(d2));

            // iterate over target list and push based on effect
            for (target, _) in target_list
                .iter()
                .take(effect.target.limit.unwrap_or(target_list.len()))
            {
                match &effect.content {
                    ActionEffectType::Transform(transform) => {
                        transforms.push((*target, transform.transform.clone()));
                    }
                    ActionEffectType::Repeat {
                        actions,
                        interval: _,
                        activations,
                    } => {
                        for _i in 0..(activations.floor()) as usize {
                            applies.push((*target, actions.clone()));
                        }
                    }
                    ActionEffectType::Once(actions) => {
                        if effect
                            .duration
                            .map(|d| d <= effect.alive_duration + ix.delta)
                            .unwrap_or(false)
                        {
                            applies.push((*target, actions.clone()));
                        }
                    }
                }
            }
        }
    }

    // apply remembered transforms to all entities
    for (target, transform) in transforms {
        if let Ok(mut entry) = world.entry_mut(target) {
            if let Ok(actions) = entry.get_component_mut::<Actions>() {
                actions.transform(transform.as_ref());
            }
        }
    }

    // apply all remembered action application to all entities
    for (target, new_actions) in applies {
        if let Ok(mut entry) = world.entry_mut(target) {
            if let Ok(actions) = entry.get_component_mut::<Actions>() {
                actions.add(new_actions);
            }
        }
    }

    // iterate over all sources of effects
    for src_act in <&mut Actions>::query().iter_mut(world) {
        // iterate over all their effects
        for effect in src_act.effects.iter_mut() {
            // Increase counting durations
            effect.alive_duration += ix.delta;
            // increase internal counter of repeater
            if let ActionEffectType::Repeat {
                actions: _,
                interval,
                activations: last_activation,
            } = &mut effect.content
            {
                *last_activation =
                    last_activation.fract() + ix.delta.as_secs_f32() / interval.as_secs_f32();
            }
        }

        // remove effects that have run their course
        src_act.effects.retain(|eff| match eff.duration {
            Some(total_dur) => total_dur > eff.alive_duration,
            None => true,
        });
    }
}
