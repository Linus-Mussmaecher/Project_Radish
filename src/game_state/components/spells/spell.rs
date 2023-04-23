use std::time::Duration;

use legion::Entity;
use mooeye::sprite::Sprite;
use tinyvec::TinyVec;

use crate::game_state::components::actions::GameAction;

const MAX_SPELL_SLOTS: usize = 6;

pub struct Spell {
    spell_slots: TinyVec<[Duration; MAX_SPELL_SLOTS]>,

    name: String,

    icon: Sprite,

    spell_: fn(Entity) -> Vec<GameAction>,
}

#[allow(dead_code)]
impl Spell {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_icon(&self) -> &Sprite {
        &self.icon
    }

    pub fn get_spell_slots(&self) -> &[Duration] {
        &self.spell_slots
    }

    pub fn attempt_cast(
        &self,
        caster: Entity,
        available_slots: &mut [Duration],
    ) -> Vec<GameAction> {
        let free_slots = available_slots.iter().filter(|slot| slot.is_zero()).count();

        if free_slots <= self.spell_slots.len() {
            let mut ind = 0;
            for slot in available_slots {
                if slot.is_zero() && ind < self.spell_slots.len() {
                    ind += 1;
                    *slot = self.spell_slots[ind];
                }
            }
            (self.spell_)(caster)
        } else {
            Vec::new()
        }
    }
}
