use legion::system;
use std::time::Duration;
use tinyvec::TinyVec;

use crate::game_state::{controller::Interactions, game_message::MessageSet};
use mooeye::sprite::Sprite;

use super::{
    actions::{ActionContainer, GameAction},
    Actions,
};

/// A mod containing construction functions for all spells.
pub mod spell_list;

/// The maximum amount of Spell Slots an entity can (by default) have.
pub const MAX_SPELL_SLOTS: usize = 8;

/// A component managing spell casting and spell slots.
pub struct SpellCaster {
    /// The spell slots available to this caster.
    /// A spell slots is a pair of two durations.
    /// The first duration marks how long this spell slot will remain blocked, the second number the total duration of the blockage (for percentage outputs).
    spell_slots: TinyVec<[(Duration, Duration); MAX_SPELL_SLOTS]>,
    /// The list of spells this caster can cast.
    spells: Vec<Spell>,
}

impl SpellCaster {
    /// Returns a new spell casting component with the specified number of slots.
    pub fn new(spells: Vec<Spell>, init_slots: usize) -> Self {
        Self {
            spells,
            spell_slots: {
                let mut vec = TinyVec::new();
                for _ in 0..init_slots {
                    vec.push(Default::default());
                }
                vec
            },
        }
    }

    /// Adds a new spell slot to this entity.
    pub fn add_slot(&mut self) {
        if self.spell_slots.len() < MAX_SPELL_SLOTS {
            self.spell_slots.push(Default::default());
        }
    }

    /// Returns the amount of current spell slots on this entity.
    pub fn get_slots(&self) -> usize {
        self.spell_slots.len()
    }

    #[allow(dead_code)]
    /// Returns the amount of currently free and unblocked spell slots.
    pub fn get_free_slots(&self) -> usize {
        self.spell_slots
            .iter()
            .filter(|slot| slot.0.is_zero())
            .count()
    }

    /// Returns wether this entity has less spell slots than the maximum.
    pub fn can_add(&self) -> bool {
        self.spell_slots.len() < MAX_SPELL_SLOTS
    }

    /// Attempts to cast a spell by checking wether the required slots are available and then blocking them.
    /// Returns a set of actions to be added to the caster (in the [spell_casting] system).
    fn attempt_cast(&mut self, index: usize) -> ActionContainer {
        if let Some(spell) = self.spells.get(index) {
            if self.get_free_slots() >= spell.spell_slots.len() {
                let mut ind = 0;
                for slot in self.spell_slots.iter_mut() {
                    if slot.0.is_zero() && ind < spell.spell_slots.len() {
                        slot.0 = Duration::from_secs_f32(spell.spell_slots[ind]);
                        slot.1 = slot.0;
                        ind += 1;
                    }
                }
                return spell.spell_.clone();
            }
        }
        ActionContainer::ApplySingle(GameAction::None)
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

    for (i, slot) in caster.spell_slots.iter().enumerate() {
        if !slot.1.is_zero() {
            messages.insert(mooeye::UiMessage::Extern(
                crate::game_state::game_message::GameMessage::UpdateSpellSlots(
                    i,
                    (slot.0.as_secs_f32() / slot.1.as_secs_f32() * 32.) as u8,
                ),
            ));
        }
    }

    // attempt casts

    for i in 0..4 {
        if ix
            .commands
            .contains_key(&crate::game_state::controller::Command::spell_from_int(i))
        {
            actions.add(caster.attempt_cast(i))
        }
    }
}

#[allow(dead_code)]
/// A spell struct.
pub struct Spell {
    /// The amount of spell slots this spell has to block to be cast.
    spell_slots: TinyVec<[f32; MAX_SPELL_SLOTS]>,
    /// The name of the spell.
    name: String,
    /// The visual representation of the spell in the spell bar or spell book.
    icon: Sprite,
    /// The actual action that is activated when casting the spell. This action is added to the caster!
    spell_: ActionContainer,
}

#[allow(dead_code)]
impl Spell {
    /// Returns the name of the spell.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Returns the icon of the spell.
    pub fn get_icon(&self) -> &Sprite {
        &self.icon
    }

    /// Returns a slice of floats representing the duration (in seconds) this spell will require to block slots for.
    pub fn get_spell_slots(&self) -> &[f32] {
        &self.spell_slots
    }
}
