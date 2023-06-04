use std::{fmt::Debug, sync::Arc, time::Duration};

use ggez::glam::Vec2;
use legion::{system, systems::CommandBuffer, Entity, EntityStore, IntoQuery};
use tinyvec::TinyVec;

use super::super::controller::Interactions;

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
    /// Silences the entity for a duration.
    Silence(Duration),
    /// Applies a (temporary or permanent) effect to the target
    ApplyEffect(Box<ActionEffect>),
}

/// A box that contains a spawner lambda-functor.
/// This allows implementing debug here and then deriving it at [GameAction].
#[derive(Clone)]
pub struct SpawnerBox {
    spawner: Box<fn(Entity, Position, &mut CommandBuffer)>,
}

impl Debug for SpawnerBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpawnerBox").finish()
    }
}

impl GameAction {
    /// Helper function to create a [GameAction::Spawn] without having to use Box.
    pub fn spawn(spawner: fn(Entity, Position, &mut CommandBuffer)) -> Self {
        Self::Spawn(SpawnerBox {
            spawner: Box::new(spawner),
        })
    }
}

impl Default for GameAction {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// An enum that is carried by remove actions to inform the remover of the source triggering the removal
pub enum RemoveSource {
    /// This entity has to be removed because it has reached zero health
    HealthLoss,
    /// This entity has to be removed because its [components::LifeDuration] has run out.
    TimedOut,
    /// A projectile having collided and thus being removed
    ProjectileCollision,
    /// An enemy reaching the bottom of the screen.
    EnemyReachedBottom,
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
    /// Creates a new transformation effect, that affects a certain set of entities for an unlimited amount of time, transforming all actions applied to them as specified.
    pub fn transform(target: ActionEffectTarget, transform: fn(&mut GameAction)) -> Self {
        Self {
            target,
            content: ActionEffectType::Transform(ActionTransformer {
                transform: Arc::new(transform),
            }),
            duration: None,
            alive_duration: Duration::ZERO,
        }
    }

    /// Creates a new reaction effect, triggering on received actions to possibly trigger other actions
    pub fn react(target: ActionEffectTarget, reaction: fn(&GameAction) -> ActionContainer) -> Self {
        Self {
            target,
            content: ActionEffectType::Reaction(Reaction {
                react: Arc::new(reaction),
            }),
            duration: None,
            alive_duration: Duration::ZERO,
        }
    }

    /// Creates a new repetition effect, applying the passed action(s) to a set of entities repeatedly for an unlimited amount of time.
    pub fn repeat(
        target: ActionEffectTarget,
        actions: impl Into<ActionContainer>,
        interval: Duration,
    ) -> Self {
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

    /// Creates a one-time effect that applies the passed action(s) to a set of entities once this frame.
    pub fn once(target: ActionEffectTarget, actions: impl Into<ActionContainer>) -> Self {
        Self {
            target,
            content: ActionEffectType::Once(actions.into()),
            duration: Some(Duration::ZERO),
            alive_duration: Duration::ZERO,
        }
    }

    /// Creates a one-time effect that triggers on the entities death. By default, does not expire.
    pub fn on_death(
        target: ActionEffectTarget,
        reason: RemoveSource,
        actions: impl Into<ActionContainer>,
    ) -> Self {
        Self {
            target,
            content: ActionEffectType::OnDeath(reason, actions.into()),
            duration: None,
            alive_duration: Duration::ZERO,
        }
    }

    /// Modifies an action effect to only last for a set amount of time.
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
    /// Transformation: Transforms effects applied to entities.
    Transform(ActionTransformer),
    /// Reactions: Triggers certain actions when receiving certain actions.
    Reaction(Reaction),
    /// Repetition: Repeatedly applies actions to entities.
    Repeat {
        actions: ActionContainer,
        interval: Duration,
        activations: f32,
    },
    /// One-time effect: Applies actions to entities once.
    Once(ActionContainer),
    /// One-time on-death effect. Applies actions once if the entity is killed by the specified remove source.
    OnDeath(RemoveSource, ActionContainer),
}

#[derive(Debug, Clone, Copy)]
/// An enum that describes what targets to distribute an effect or ActionModification to.
pub struct ActionEffectTarget {
    /// If only entities that have an [super::Enemy] component are affected.
    enemies_only: bool,
    /// The range from the source entity.
    range: f32,
    /// If the source entity itself is also affected.
    affect_self: bool,
    /// The max amount of entities that will be affected. None means an unlimited amount.
    /// These will be sorted by distance to source entity.
    limit: Option<usize>,
}

impl ActionEffectTarget {
    /// Creates a new effect target with default parameters.
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
/// A composite component of [ActionEffect] that transforms actions.
struct ActionTransformer {
    transform: Arc<fn(&mut GameAction)>,
}

impl Debug for ActionTransformer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ActionTransformer").finish()
    }
}

#[derive(Clone)]
/// A compositive component of [ActionEffect] that reacts to received actions.
struct Reaction {
    react: Arc<fn(&GameAction) -> ActionContainer>,
}

impl Debug for Reaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Reaction").finish()
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

impl From<Vec<GameAction>> for ActionContainer {
    fn from(value: Vec<GameAction>) -> Self {
        ActionContainer::ApplyMultiple(value)
    }
}

impl Default for ActionContainer {
    fn default() -> Self {
        Self::ApplySingle(GameAction::None)
    }
}

/// A component that handles an entities interaction with the world via an action queue
pub struct Actions {
    /// The actions to be performed on this entity.
    action_queue: TinyVec<[GameAction; 4]>,
    /// The effects that currently apply to this entity.
    effects: TinyVec<[ActionEffect; 4]>,
    /// The remaining duration this entity is silenced for, making effects not trigger.
    silence: Duration,
}

impl Actions {
    /// Creates a new, empty Actions component
    pub fn new() -> Self {
        Self {
            action_queue: TinyVec::new(),
            effects: TinyVec::new(),
            silence: Duration::ZERO,
        }
    }

    /// Modifies this components to include a certain effect from the start.
    /// Can be used to add auras to entities on construction.
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
        else if !matches!(action, GameAction::None) {
            // push the action
            self.action_queue.push(action);
        }
    }

    /// Adds one or multiple actions to this entitiy. They will be handled this game frame and then discarded.
    pub fn push_container(&mut self, actions: ActionContainer) {
        match actions {
            ActionContainer::ApplySingle(act) => self.push(act),
            ActionContainer::ApplyMultiple(act_vec) => {
                for act in act_vec {
                    self.push(act);
                }
            }
        }
    }

    /// Transforms all actions currently in this entities action queue.
    fn transform(&mut self, transform: &fn(&mut GameAction)) {
        for action in self.action_queue.iter_mut() {
            transform(action);
        }
    }

    // Reacts to all currently stored actions, potentially creating new actions
    fn react(&mut self, reaction: &fn(&GameAction) -> ActionContainer) {
        let mut to_add = Vec::new();
        for action in self.action_queue.iter() {
            to_add.push(reaction(action));
        }
        for cont in to_add {
            self.push_container(cont);
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

#[legion::system(for_each)]
/// System that clears all actions queues.
pub fn apply_silence(actions: &mut Actions) {
    for act in actions.action_queue.iter() {
        if let GameAction::Silence(duration) = act {
            actions.silence += *duration;
        }
    }
}

#[system(for_each)]
/// System that handles all spawn and other actions by executing their closures.
pub fn resolve_executive_actions(
    ent: &Entity,
    actions: &Actions,
    pos: Option<&Position>,
    cmd: &mut CommandBuffer,
) {
    for action in actions.get_actions() {
        match action {
            GameAction::Spawn(spawner) => {
                (spawner.spawner)(*ent, pos.map(|p| *p).unwrap_or_default(), cmd)
            }
            _ => {}
        }
    }
}

/// A special system that applies all aura-components that transform actions of other entities
#[system]
#[write_component(Actions)]
#[read_component(Enemy)]
#[read_component(Position)]
pub fn handle_effects(world: &mut legion::world::SubWorld, #[resource] ix: &Interactions) {
    // compile a list of all transforms & applies affecting entities
    let mut transforms = Vec::new();
    let mut reactions = Vec::new();
    let mut applies = Vec::new();

    // iterate over all sources of effects
    for (src_ent, src_pos, src_act) in <(Entity, &Position, &Actions)>::query().iter(world) {
        // skip silenced entities
        if !src_act.silence.is_zero() {
            continue;
        }
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
                    ActionEffectType::Reaction(reaction) => {
                        reactions.push((*target, reaction.react.clone()));
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
                    ActionEffectType::OnDeath(reason, actions) => {
                        if src_act.action_queue.iter().any(
                            |act| matches!(act, &GameAction::Remove(source) if source == *reason),
                        ) {
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

    // apply remembered reactions to all entities
    for (target, reaction) in reactions {
        if let Ok(mut entry) = world.entry_mut(target) {
            if let Ok(actions) = entry.get_component_mut::<Actions>() {
                actions.react(reaction.as_ref());
            }
        }
    }

    // apply all remembered action application to all entities
    for (target, new_actions) in applies {
        if let Ok(mut entry) = world.entry_mut(target) {
            if let Ok(actions) = entry.get_component_mut::<Actions>() {
                actions.push_container(new_actions);
            }
        }
    }

    // iterate over all sources of effects
    for src_act in <&mut Actions>::query().iter_mut(world) {
        // reduce silence duration
        src_act.silence = src_act.silence.saturating_sub(ix.delta);

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
