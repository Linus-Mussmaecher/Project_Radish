use legion::system;
use std::time::Duration;
use tinyvec::TinyVec;

use crate::game_state::{controller::Interactions, game_message::MessageSet};
use mooeye::sprite::Sprite;

use super::{
    actions::{GameAction, ActionContainer},
    Actions,
};

pub mod spell_list;

pub struct SpellCaster {
    spell_slots: TinyVec<[(Duration, Duration); MAX_SPELL_SLOTS]>,
    spells: Vec<Spell>,
}

impl SpellCaster {
    /// Returns a new spell casting component.
    pub fn new(spells: Vec<Spell>) -> Self {
        Self {
            spells,
            spell_slots: TinyVec::from([(Duration::ZERO, Duration::ZERO); MAX_SPELL_SLOTS]),
        }
    }
}

#[system(for_each)]
/// A system that resolves spell casting actions (most likely sent by the control component) by casting the spells.
pub fn spell_casting(
    caster: &mut SpellCaster,
    actions: &mut Actions,
    #[resource] messages: &mut MessageSet,
    #[resource] ix: &Interactions,
) {
    // reduce cooldowns
    for slot in caster.spell_slots.iter_mut() {
        if !slot.0.is_zero() {
            slot.0 = slot.0.saturating_sub(ix.delta);
        } else if !slot.1.is_zero() {
            slot.1 = Duration::ZERO;
        }
    }

    for i in 0..MAX_SPELL_SLOTS {
        if i < caster.spell_slots.len() && !caster.spell_slots[i].1.is_zero() {
            messages.insert(mooeye::UiMessage::Extern(
                crate::game_state::game_message::GameMessage::UpdateSpellSlots(
                    i,
                    (caster.spell_slots[i].0.as_secs_f32() / caster.spell_slots[i].1.as_secs_f32()
                        * 32.) as u8,
                ),
            ));
        }
    }

    // attempt casts

    let mut cast = None;

    if ix
        .commands
        .contains_key(&crate::game_state::controller::Command::Spell0)
    {
        cast = Some(0);
    }

    if ix
        .commands
        .contains_key(&crate::game_state::controller::Command::Spell1)
    {
        cast = Some(1);
    }

    if ix
        .commands
        .contains_key(&crate::game_state::controller::Command::Spell2)
    {
        cast = Some(2);
    }

    if ix
        .commands
        .contains_key(&crate::game_state::controller::Command::Spell3)
    {
        cast = Some(3);
    }
    if let Some(cast) = cast {
        if let Some(spell) = caster.spells.get(cast) {
            actions.add(spell.attempt_cast(&mut caster.spell_slots));
        }
    }
}

pub const MAX_SPELL_SLOTS: usize = 6;

pub struct Spell {
    spell_slots: TinyVec<[f32; MAX_SPELL_SLOTS]>,

    name: String,

    icon: Sprite,

    spell_: ActionContainer,
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
        available_slots: &mut TinyVec<[(Duration, Duration); MAX_SPELL_SLOTS]>,
    ) -> ActionContainer {
        let free_slots = available_slots
            .iter()
            .filter(|slot| slot.0.is_zero())
            .count();

        if free_slots >= self.spell_slots.len() {
            let mut ind = 0;
            for slot in available_slots {
                if slot.0.is_zero() && ind < self.spell_slots.len() {
                    slot.0 = Duration::from_secs_f32(self.spell_slots[ind]);
                    slot.1 = slot.0;
                    ind += 1;
                }
            }
            self.spell_.clone()
        } else {
            ActionContainer::ApplySingle(GameAction::None)
        }
    }
}
