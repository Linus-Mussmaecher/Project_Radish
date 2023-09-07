use ggez::graphics;
use legion::system;
use mooeye::{ui, ui::UiContent};
use std::time::Duration;
use tinyvec::TinyVec;

use mooeye::sprite::Sprite;

use crate::PALETTE;

use super::super::{achievements, controller, game_message};

use super::{
    actions::{ActionContainer, GameAction},
    Actions,
};

/// Module containing constructor functions for all fire spells.
mod fire;
/// Module containing constructor functions for all ice and lightning spells.
mod iceligthning;
/// Module containing constructor functions for all air, void and arcane spells.
mod misc;

/// The maximum amount of Spell Slots an entity can (by default) have.
pub const MAX_SPELL_SLOTS: usize = 8;

pub fn init_spell_pool(
    sprite_pool: &mooeye::sprite::SpritePool,
    achievements: &achievements::AchievementSet,
) -> SpellPool {
    (
        None,
        vec![
            SpellTemplate::new(fire::construct_fireball(sprite_pool), 50).purchased(),
            SpellTemplate::new(fire::construct_scorch(sprite_pool), 90),
            SpellTemplate::new(fire::construct_mortar(sprite_pool), 145).guild_condition(1),
            SpellTemplate::new(fire::construct_flameorb(sprite_pool), 50).guild_condition(2),
            SpellTemplate::new(fire::construct_conflagrate(sprite_pool), 150)
                .guild_condition(3)
                .achievement_condition(achievements.list.get(8), sprite_pool),
            SpellTemplate::new(fire::construct_phoenix(sprite_pool), 200)
                .guild_condition(4)
                .achievement_condition(achievements.list.get(5), sprite_pool),
            SpellTemplate::new(iceligthning::construct_ice_bomb(sprite_pool), 75).purchased(),
            SpellTemplate::new(iceligthning::construct_shard(sprite_pool), 60).guild_condition(1),
            SpellTemplate::new(iceligthning::construct_ice_lance(sprite_pool), 80)
                .guild_condition(2)
                .achievement_condition(achievements.list.get(9), sprite_pool),
            SpellTemplate::new(iceligthning::construct_lightning_orb(sprite_pool), 90),
            SpellTemplate::new(iceligthning::construct_overload(sprite_pool), 120)
                .guild_condition(1),
            SpellTemplate::new(iceligthning::construct_lightning_ball(sprite_pool), 145)
                .guild_condition(2)
                .achievement_condition(achievements.list.get(1), sprite_pool),
            SpellTemplate::new(misc::construct_gale_force(sprite_pool), 120).guild_condition(3),
            SpellTemplate::new(misc::construct_airburst(sprite_pool), 170)
                .guild_condition(4)
                .achievement_condition(achievements.list.get(10), sprite_pool),
            SpellTemplate::new(misc::construct_mind_wipe(sprite_pool), 200).guild_condition(3),
            SpellTemplate::new(misc::construct_blackhole(sprite_pool), 200)
                .guild_condition(4)
                .achievement_condition(achievements.list.get(11), sprite_pool),
            SpellTemplate::new(misc::construct_arcane_blast(sprite_pool), 140).guild_condition(3),
            SpellTemplate::new(misc::construct_arcane_missiles(sprite_pool), 150)
                .guild_condition(4)
                .achievement_condition(achievements.list.get(14), sprite_pool),
        ],
    )
}

pub fn init_base_spells(
    spell_pool: &SpellPool,
    sprite_pool: &mooeye::sprite::SpritePool,
    spells: &[usize],
) -> Vec<Spell> {
    spells
        .iter()
        .map(|&index| {
            if index == 0 || index > spell_pool.1.len() {
                Spell::not_available(sprite_pool, "Purchase & equip more spells between waves!")
            } else {
                spell_pool.1[index - 1].spell.clone()
            }
        })
        .collect()
}

/// A component managing spell casting and spell slots.
pub struct SpellCaster {
    /// The spell slots available to this caster.
    /// A spell slots is a pair of two durations.
    /// The first duration marks how long this spell slot will remain blocked, the second number the total duration of the blockage (for percentage outputs).
    spell_slots: TinyVec<[(Duration, Duration); MAX_SPELL_SLOTS]>,
    /// The list of spells this caster can cast.
    spells: Vec<Spell>,
    /// the amount of slots this element started with
    base_slots: usize,
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
            base_slots: init_slots,
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
    pub fn set_extra_slots(&mut self, slots: usize) {
        // cutoff if smaller
        self.spell_slots.truncate(slots + self.base_slots);
        // add if bigger
        for _ in 0..(slots + self.base_slots) - self.spell_slots.len() {
            self.spell_slots.push(Default::default());
        }
    }

    /// Returns the amount of current spell slots on this entity.
    pub fn get_slots(&self) -> usize {
        self.spell_slots.len()
    }

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
                return if let Some(sound) = &spell.sound {
                    match spell.spell_.clone() {
                        ActionContainer::ApplySingle(a) => ActionContainer::ApplyMultiple(vec![
                            a,
                            GameAction::PlaySound(sound.clone()),
                        ]),
                        ActionContainer::ApplyMultiple(mut vec) => {
                            vec.push(GameAction::PlaySound(sound.clone()));
                            ActionContainer::ApplyMultiple(vec)
                        }
                    }
                } else {
                    spell.spell_.clone()
                };
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
            messages.insert(ui::UiMessage::Extern(
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
    /// Describes the required level on the mage's guild.
    pub guild_condition: u8,
}

impl SpellTemplate {
    /// Creates a new spell template from a spell and a  cost.
    pub fn new(spell: Spell, cost: i32) -> Self {
        Self {
            level: 0,
            cost,
            spell,
            guild_condition: 0,
        }
    }

    // modifies the template to have the spell purchased from the start
    pub fn purchased(mut self) -> Self {
        self.level = 1;
        self
    }

    /// Modifies the template based on an achievement:
    /// If the achievement exists and is not unlocked, the spell will be replaced by a non-available spell with a fitting message.
    /// If the achievement doesn't exist or is unlocked, the spell will display as normal.
    pub fn achievement_condition(
        mut self,
        ach: Option<&achievements::Achievement>,
        sprite_pool: &mooeye::sprite::SpritePool,
    ) -> Self {
        if let Some(ach) = ach {
            if !ach.is_achieved() {
                self.spell = Spell::not_available(
                    sprite_pool,
                    &format!(
                        "Complete the achievement '{}' to unlock this spell for future games.",
                        ach.get_name()
                    ),
                );
                self.level = 0;
                self.cost = 0;
            }
        }
        self
    }

    /// Modifies the template based on a required level of the mages guild:
    pub fn guild_condition(mut self, level: u8) -> Self {
        self.guild_condition = level;
        self
    }

    /// Returns a small UiElement representing this spell template, consisting of the icon and a tooltip.
    pub fn info_element_small<T: Copy + Eq + std::hash::Hash + 'static>(
        &self,
        id: u32,
        ctx: &ggez::Context,
        buildings: &super::buildings::Buildings,
    ) -> ui::UiElement<T> {
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
        .with_visuals(ui::Visuals {
            background: graphics::Color::from_rgb_u32(PALETTE[8]),
            border: graphics::Color::from_rgb_u32(PALETTE[8]),
            border_widths: [0.; 4],
            corner_radii: [10.; 4],
        })
        .as_shrink()
        .with_alignment(ui::Alignment::Max, ui::Alignment::Max)
        .build();

        let guild = graphics::Text::new(
            graphics::TextFragment::new(format!(" {} ", self.guild_condition))
                .color(graphics::Color::from_rgb_u32(PALETTE[14])),
        )
        .set_scale(16.)
        .set_font("Retro")
        .to_owned()
        .to_element_builder(0, ctx)
        .with_padding((2., 2., 2., 2.))
        .with_visuals(ui::Visuals {
            background: graphics::Color::from_rgb_u32(PALETTE[4]),
            border: graphics::Color::from_rgb_u32(PALETTE[4]),
            border_widths: [0.; 4],
            corner_radii: [10.; 4],
        })
        .as_shrink()
        .with_alignment(ui::Alignment::Max, ui::Alignment::Min)
        .build();

        ui::containers::StackBox::new()
            .to_element_builder(id, ctx)
            .with_wrapper_layout(icon.get_layout())
            .with_child(if self.level == 0 && self.cost != 0 {
                cost
            } else {
                ().to_element(0, ctx)
            })
            .with_child(if buildings.target[1] >= self.guild_condition {
                ().to_element(0, ctx)
            } else {
                guild
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
    /// The sound played on casting the spell
    sound: Option<String>,

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
        sound: impl Into<Option<&'static str>>,
        spell_: impl Into<ActionContainer>,
        spell_slots: TinyVec<[f32; MAX_SPELL_SLOTS]>,
    ) -> Self {
        Self {
            name: name.to_owned(),
            description: description.to_owned(),
            icon,
            spell_: spell_.into(),
            spell_slots,
            sound: sound.into().map(|s| s.to_owned()),
        }
    }

    fn not_available(sprite_pool: &mooeye::sprite::SpritePool, reason: &str) -> Self {
        Self {
            name: "Spell not available".to_owned(),
            description: reason.to_owned(),
            icon: sprite_pool
                .init_sprite("/sprites/ui/lock", Duration::ZERO)
                .unwrap_or_default(),
            spell_: GameAction::None.into(),
            spell_slots: TinyVec::new(),
            sound: None,
        }
    }

    /// Returns a small UiElement representing this spell, consisting of the icon and a tooltip.
    pub fn info_element_small<T: Copy + Eq + std::hash::Hash + 'static>(
        &self,
        id: u32,
        ctx: &ggez::Context,
    ) -> ui::UiElement<T> {
        self.icon
            .clone()
            .to_element_builder(id, ctx)
            .with_visuals(crate::scenes::BUTTON_VIS)
            .with_size(ui::Size::Fixed(48.), ui::Size::Fixed(48.))
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
                    graphics::TextFragment::new(if !self.spell_slots.is_empty() {
                        "\nSpell slots:"
                    } else {
                        ""
                    })
                    .color(graphics::Color::from_rgb_u32(PALETTE[3]))
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
