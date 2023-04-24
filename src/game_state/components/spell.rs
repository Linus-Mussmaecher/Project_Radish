use legion::{*};
use std::time::Duration;
use tinyvec::TinyVec;

use crate::game_state::{controller::Interactions};
use mooeye::sprite::{Sprite};

use super::{actions::GameAction, Actions};

pub mod spell_list;

pub struct SpellCaster {
    spell_slots: TinyVec<[Duration; MAX_SPELL_SLOTS]>,
    spells: Vec<Spell>,
}

impl SpellCaster {
    /// Returns a new spell casting component.
    pub fn new(spells: Vec<Spell>) -> Self {
        Self {
            spells,
            spell_slots: TinyVec::from([Duration::ZERO; 6]),
        }
    }
}

#[system(for_each)]
/// A system that resolves spell casting actions (most likely sent by the control component) by casting the spells.
pub fn spell_casting(
    caster_ent: &Entity,
    caster: &mut SpellCaster,
    actions: &mut Actions,
    #[resource] ix: &Interactions,
) {
        // reduce cooldowns
        for slot in caster.spell_slots.iter_mut(){
            *slot = slot.saturating_sub(ix.delta);
        }

        println!("Free slots: {}", caster.spell_slots.iter().filter(|slot| slot.is_zero()).count());

        // attempt casts


        if ix.commands.contains_key(&crate::game_state::controller::Command::Spell0){
            if let Some(spell) = caster.spells.get(0){
                actions.get_actions_mut().extend(spell.attempt_cast(*caster_ent, &mut caster.spell_slots));
            }
        }

        if ix.commands.contains_key(&crate::game_state::controller::Command::Spell1){
            if let Some(spell) = caster.spells.get(1){
                actions.get_actions_mut().extend(spell.attempt_cast(*caster_ent, &mut caster.spell_slots));
            }
        }

        if ix.commands.contains_key(&crate::game_state::controller::Command::Spell2){
            if let Some(spell) = caster.spells.get(2){
                actions.get_actions_mut().extend(spell.attempt_cast(*caster_ent, &mut caster.spell_slots));
            }
        }

        if ix.commands.contains_key(&crate::game_state::controller::Command::Spell3){
            if let Some(spell) = caster.spells.get(3){
                actions.get_actions_mut().extend(spell.attempt_cast(*caster_ent, &mut caster.spell_slots));
            }
        }
    
}

const MAX_SPELL_SLOTS: usize = 6;

pub struct Spell {
    spell_slots: TinyVec<[f32; MAX_SPELL_SLOTS]>,

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

    pub fn get_spell_slots(&self) -> &[f32] {
        &self.spell_slots
    }

    pub fn attempt_cast(
        &self,
        caster: Entity,
        available_slots: &mut TinyVec<[Duration; MAX_SPELL_SLOTS]>,
    ) -> Vec<GameAction> {
        let free_slots = available_slots.iter().filter(|slot| slot.is_zero()).count();

        if free_slots >= self.spell_slots.len() {
            let mut ind = 0;
            for slot in available_slots {
                if slot.is_zero() && ind < self.spell_slots.len() {
                    *slot = Duration::from_secs_f32(self.spell_slots[ind]);
                    ind += 1;
                }
            }
            (self.spell_)(caster)
        } else {
            Vec::new()
        }
    }
}
