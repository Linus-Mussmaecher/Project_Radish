use ggez::{glam::Vec2};
use legion::{systems::CommandBuffer, Entity};
use tinyvec::TinyVec;

use super::Position;

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum GameAction{
    None,
    Remove,
    Move{delta: Vec2},
    TakeDamage{dmg: i32},
    TakeHealing{heal: i32},
    TakeCityDamage{dmg: i32},
    GainGold{amount: i32},
    AddImmunity{other: legion::Entity},
    CastSpell(usize),
    Spawn(&'static (dyn Fn(Entity, Position, &mut CommandBuffer) + Send + Sync)),
    OtherAction(&'static (dyn Fn(Entity, &mut CommandBuffer) + Send + Sync)),
}

impl PartialEq for GameAction{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Move { delta: l_delta }, Self::Move { delta: r_delta }) => l_delta == r_delta,
            (Self::TakeDamage { dmg: l_dmg }, Self::TakeDamage { dmg: r_dmg }) => l_dmg == r_dmg,
            (Self::TakeHealing { heal: l_heal }, Self::TakeHealing { heal: r_heal }) => l_heal == r_heal,
            (Self::TakeCityDamage { dmg: l_dmg }, Self::TakeCityDamage { dmg: r_dmg }) => l_dmg == r_dmg,
            (Self::GainGold { amount: l_amount }, Self::GainGold { amount: r_amount }) => l_amount == r_amount,
            (Self::AddImmunity { other: l_other }, Self::AddImmunity { other: r_other }) => l_other == r_other,
            (Self::CastSpell(l0), Self::CastSpell(r0)) => l0 == r0,
            (Self::Spawn(_), Self::Spawn(_)) => false,
            (Self::OtherAction(_), Self::OtherAction(_)) => false,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl Default for GameAction{
    fn default() -> Self {
        Self::None
    }
}

pub struct Actions {
    action_queue: TinyVec<[GameAction; 4]>,
}

impl Actions {
    pub fn new() -> Self {
        Self {
            action_queue: TinyVec::new(),
        }
    }

    pub fn push(&mut self, action: GameAction) {
        self.action_queue.push(action);
    }

    pub fn get_actions(&self) -> &TinyVec<[GameAction; 4]> {
        &self.action_queue
    }

    // pub fn get_actions_mut(&mut self) -> &mut Vec<GameAction>{
    //     &mut self.action_queue
    // }
}

impl From<TinyVec<[GameAction; 4]>> for Actions {
    fn from(value: TinyVec<[GameAction; 4]>) -> Self {
        Self {
            action_queue: value,
        }
    }
}

#[legion::system(for_each)]
pub fn clear(actions: &mut Actions) {
    actions.action_queue.clear();
}

#[legion::system(for_each)]
pub fn executive_actions(ent: &Entity, actions: &Actions, pos: Option<&Position>, cmd: &mut CommandBuffer){
    for action in actions.get_actions(){
        match action {
            GameAction::Spawn(spawner) => (spawner)(*ent, pos.map(|p| *p).unwrap_or_default(), cmd),
            GameAction::OtherAction(executor) => (executor)(*ent, cmd),
            _ => {}
        }
    }
}
