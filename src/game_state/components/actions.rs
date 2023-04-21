use ggez::glam::Vec2;
use tinyvec::TinyVec;

#[derive(Clone, Copy, Debug, PartialEq)]
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
