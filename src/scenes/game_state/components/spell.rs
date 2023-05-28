use ggez::graphics;
use legion::system;
use mooeye::{containers, ui_element, UiContent};
use std::time::Duration;
use tinyvec::TinyVec;

use mooeye::sprite::Sprite;

use crate::PALETTE;

use super::super::{controller, game_message};

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

    /// Returns a reference to a slice of the spells this entity can cast.
    pub fn get_spells(&self) -> &[Spell] {
        &self.spells
    }

    /// Equips a spell in the given slot.
    pub fn equip_spell(&mut self, index: usize, spell: Spell) {
        if self.spells.len() > index {
            self.spells[index] = spell;
        } else {
            self.spells.push(spell);
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
    #[resource] messages: &mut game_message::MessageSet,
    #[resource] ix: &controller::Interactions,
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
                game_message::GameMessage::UpdateSpellSlots(
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
            .contains_key(&controller::Command::spell_from_int(i))
        {
            actions.push_container(caster.attempt_cast(i))
        }
    }
}

/// A set of spells that the user can unlock. The Option may contain a spell about to be equipped.
pub type SpellPool = (Option<Spell>, Vec<SpellTemplate>);

/// A struct that represents the possibility of a spell that can be equipped. It has a cost and a level.
/// Level 0 implies the spell is not unlocked yet and has to be unlocked for the cost.
#[derive(Clone, Debug)]
pub struct SpellTemplate {
    /// The spells level. Level 0 implies the spell is not unlocked yet and has to be unlocked for the cost.
    pub level: u32,
    /// The cost to unlock the spell.
    pub cost: i32,
    /// The spell itself.
    pub spell: Spell,
}

impl SpellTemplate {
    /// Creates a new spell template from a spell and a  cost.
    pub fn new(spell: Spell, cost: i32, level: u32) -> Self {
        Self { level, cost, spell }
    }

    /// Returns a small UiElement representing this spell template, consisting of the icon and a tooltip.
    pub fn info_element_small<T: Copy + Eq + std::hash::Hash + 'static>(
        &self,
        id: u32,
        ctx: &ggez::Context,
    ) -> mooeye::UiElement<T> {
        let icon = self.spell.info_element_small(0, ctx);

        let cost = graphics::Text::new(
            graphics::TextFragment::new(format!("{}", self.cost))
                .color(graphics::Color::from_rgb_u32(PALETTE[14])),
        )
        .set_scale(16.)
        .set_font("Retro")
        .to_owned()
        .to_element_builder(0, ctx)
        .with_padding((2., 2., 2., 2.))
        .with_visuals(ui_element::Visuals {
            background: graphics::Color::from_rgb_u32(PALETTE[8]),
            border: graphics::Color::from_rgb_u32(PALETTE[8]),
            border_widths: [0.; 4],
            corner_radii: [10.; 4],
        })
        .as_shrink()
        .with_alignment(ui_element::Alignment::Max, ui_element::Alignment::Max)
        .build();

        containers::StackBox::new()
            .to_element_builder(id, ctx)
            .with_wrapper_layout(icon.get_layout())
            .with_child(if self.level == 0 {
                cost
            } else {
                ().to_element(0, ctx)
            })
            .with_child(
                crate::scenes::game_state::ui::game_ui::Covering::new(
                    graphics::Color {
                        a: 0.7,
                        ..graphics::Color::from_rgb_u32(PALETTE[0])
                    },
                    if self.level == 0 { 1. } else { 0. },
                )
                .to_element(0, ctx),
            )
            .with_child(icon)
            .build()
    }
}

pub fn init_spell_pool(sprite_pool: &mooeye::sprite::SpritePool) -> SpellPool {
    (
        None,
        vec![
            SpellTemplate::new(spell_list::construct_fireball(sprite_pool), 50, 1),
            SpellTemplate::new(spell_list::construct_ice_bomb(sprite_pool), 75, 1),
            SpellTemplate::new(spell_list::construct_lightning_orb(sprite_pool), 90, 0),
            SpellTemplate::new(spell_list::construct_conflagrate(sprite_pool), 110, 0),
            SpellTemplate::new(spell_list::construct_shard(sprite_pool), 60, 0),
            SpellTemplate::new(spell_list::construct_ice_lance(sprite_pool), 80, 0),
            SpellTemplate::new(spell_list::construct_scorch(sprite_pool), 90, 0),
            SpellTemplate::new(spell_list::construct_overload(sprite_pool), 120, 0),
            SpellTemplate::new(spell_list::construct_arcane_missiles(sprite_pool), 150, 0),
            SpellTemplate::new(spell_list::construct_arcane_blast(sprite_pool), 140, 0),
            SpellTemplate::new(spell_list::construct_blackhole(sprite_pool), 200, 0),
        ],
    )
}

#[derive(Clone, Debug)]
/// A spell struct.
pub struct Spell {
    // -------- COSMETIC --------
    /// The name of the spell.
    name: String,
    /// A short description of the spell to be displayed in the spell book.
    description: String,
    /// The visual representation of the spell in the spell bar or spell book.
    icon: Sprite,

    // -------- FUNCTIONAL --------
    /// The actual action that is activated when casting the spell. This action is added to the caster!
    spell_: ActionContainer,
    /// The amount of spell slots this spell has to block to be cast.
    spell_slots: TinyVec<[f32; MAX_SPELL_SLOTS]>,
}

impl Spell {
    /// Constructs a new spell.
    fn new(
        name: &str,
        description: &str,
        icon: Sprite,
        spell_: impl Into<ActionContainer>,
        spell_slots: TinyVec<[f32; MAX_SPELL_SLOTS]>,
    ) -> Self {
        Self {
            name: name.to_owned(),
            description: description.to_owned(),
            icon,
            spell_: spell_.into(),
            spell_slots,
        }
    }

    /// Returns a small UiElement representing this spell, consisting of the icon and a tooltip.
    pub fn info_element_small<T: Copy + Eq + std::hash::Hash + 'static>(
        &self,
        id: u32,
        ctx: &ggez::Context,
    ) -> mooeye::UiElement<T> {
        self.icon
            .clone()
            .to_element_builder(id, ctx)
            .with_visuals(crate::scenes::BUTTON_VIS)
            .with_size(
                mooeye::ui_element::Size::Fixed(48.),
                mooeye::ui_element::Size::Fixed(48.),
            )
            .with_tooltip(
                graphics::Text::new(
                    graphics::TextFragment::new(&self.name)
                        .color(graphics::Color::from_rgb_u32(PALETTE[7]))
                        .scale(28.),
                )
                .add("\n")
                .add(
                    graphics::TextFragment::new(&self.description)
                        .color(graphics::Color::from_rgb_u32(PALETTE[6]))
                        .scale(20.),
                )
                .add(
                    graphics::TextFragment::new("\nCasting slots:")
                        .color(graphics::Color::from_rgb_u32(PALETTE[6]))
                        .scale(20.),
                )
                .add(
                    graphics::TextFragment::new(self.spell_slots.iter().fold(
                        String::new(),
                        |mut old, &slot| {
                            old.push_str(&format!("  {:.1}", slot));
                            old
                        },
                    ))
                    .color(graphics::Color::from_rgb_u32(PALETTE[4]))
                    .scale(20.),
                )
                .set_font("Retro")
                .set_wrap(true)
                .set_bounds(ggez::glam::Vec2::new(400., 200.))
                .to_owned()
                .to_element_builder(0, ctx)
                .with_visuals(crate::scenes::BUTTON_VIS)
                .build(),
            )
            .build()
    }
}
