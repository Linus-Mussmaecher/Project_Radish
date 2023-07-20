use ggez::{graphics, GameError};

use mooeye::*;
use mooeye::{ui, ui::UiContainer, ui::UiContent};
use std::hash::Hash;
use std::time::Duration;

use super::game_state;

use crate::PALETTE;

pub const ID_SPELL_BAR: u32 = 60;
pub const ID_SPELL_BAR_CHILDREN: u32 = 61;
pub const ID_MANA_BAR: u32 = 50;
pub const ID_MANA_SLOT: u32 = 51;

/// Constructs the UiElement that forms the main UI of the game.
/// Consists of
///  - Menu button
///  - Gold & City health display
///  - Cooldown display for the players spell slots
///  - Indicator for currently equipped spells
///  - Vertical box to display messages (achievements etc.)
pub fn construct_game_ui(
    ctx: &ggez::Context,
    config: super::game_state::GameConfig,
) -> Result<ui::UiElement<super::super::GameMessage>, GameError> {
    // options icon
    let cog_icon = graphics::Image::from_path(ctx, "/sprites/ui/cog.png")?
        .to_element_builder(1, ctx)
        .with_trigger_key(ggez::winit::event::VirtualKeyCode::F10)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .with_alignment(ui::Alignment::Max, ui::Alignment::Max)
        .scaled(2., 2.)
        .with_offset(-10., -10.)
        .as_shrink()
        .build();

    // gold display
    let gold_icon = sprite::Sprite::from_path_fmt(
        "/sprites/ui/coin_16_16.png",
        ctx,
        Duration::from_secs_f32(0.25),
    )?
    .to_element_builder(0, ctx)
    .scaled(2., 2.)
    .build();

    let gold_text = graphics::Text::new(
        graphics::TextFragment::new("0000").color(graphics::Color::from_rgb_u32(PALETTE[6])),
    )
    .set_scale(32.)
    .set_font("Retro")
    .to_owned()
    .to_element_builder(0, ctx)
    .with_tooltip(
        graphics::Text::new(
            graphics::TextFragment::new("Your current amount of gold.")
                .color(graphics::Color::from_rgb_u32(PALETTE[6])),
        )
        .set_scale(24.)
        .set_font("Retro")
        .to_owned()
        .to_element_builder(0, ctx)
        .with_visuals(super::BUTTON_VIS)
        .build(),
    )
    .with_message_handler(|message_set, _, transitions| {
        for message in message_set {
            if let ui::UiMessage::Extern(game_state::GameMessage::UpdateGold(new_gold)) = message {
                transitions.push_back(
                    ui::Transition::new(Duration::ZERO).with_new_content(
                        graphics::Text::new(
                            graphics::TextFragment::new(format!("{:04}", *new_gold))
                                .color(graphics::Color::from_rgb_u32(PALETTE[6])),
                        )
                        .set_scale(32.)
                        .set_font("Retro")
                        .to_owned(),
                    ),
                );
            }
        }
    })
    .build();

    // city health display

    let city_display = sprite::Sprite::from_path_fmt(
        "/sprites/ui/city_16_16.png",
        ctx,
        Duration::from_secs_f32(0.25),
    )?
    .to_element_builder(0, ctx)
    .scaled(2., 2.)
    .build();

    let city_text = graphics::Text::new(
        graphics::TextFragment::new("100").color(graphics::Color::from_rgb_u32(PALETTE[6])),
    )
    .set_scale(32.)
    .set_font("Retro")
    .to_owned()
    .to_element_builder(0, ctx)
    .with_tooltip(
        graphics::Text::new(
            graphics::TextFragment::new("The health your town currently has left.")
                .color(graphics::Color::from_rgb_u32(PALETTE[6])),
        )
        .set_scale(24.)
        .set_font("Retro")
        .to_owned()
        .to_element_builder(0, ctx)
        .with_visuals(super::BUTTON_VIS)
        .build(),
    )
    .with_message_handler(|message_set, _, transitions| {
        for message in message_set {
            if let ui::UiMessage::Extern(game_state::GameMessage::UpdateCityHealth(new_health)) =
                message
            {
                transitions.push_back(
                    ui::Transition::new(Duration::ZERO).with_new_content(
                        graphics::Text::new(
                            graphics::TextFragment::new(format!("{:03}", *new_health))
                                .color(graphics::Color::from_rgb_u32(PALETTE[6])),
                        )
                        .set_scale(32.)
                        .set_font("Retro")
                        .to_owned(),
                    ),
                );
            }
        }
    })
    .build();

    let mut data_box = ui::containers::GridBox::new(2, 2);
    data_box.add(gold_icon, 0, 0)?;
    data_box.add(gold_text, 1, 0)?;
    data_box.add(city_display, 0, 1)?;
    data_box.add(city_text, 1, 1)?;

    let data_box = data_box
        .to_element_builder(0, ctx)
        .with_visuals(super::BUTTON_VIS)
        .with_alignment(ui::Alignment::Max, ui::Alignment::Min)
        .with_offset(-8., 8.)
        .build();

    // Spells

    let mut slot_box = ui::containers::HorizontalBox::new();

    for i in 0..config.base_slots {
        slot_box.add(create_spellslot(ctx, i));
    }

    let slot_box = slot_box
        .to_element_builder(ID_MANA_BAR, ctx)
        .with_visuals(mooeye::ui::Visuals {
            border_widths: [0., 3., 3., 3.],
            corner_radii: [0., 3., 3., 0.],
            ..super::BUTTON_VIS
        })
        .with_alignment(ui::Alignment::Min, ui::Alignment::Min)
        .with_offset(32., None)
        .build();

    let spell_box = ui::containers::StackBox::new()
        .to_element_builder(ID_SPELL_BAR, ctx)
        .with_visuals(mooeye::ui::Visuals {
            border_widths: [3., 3., 0., 3.],
            corner_radii: [3., 0., 0., 3.],
            ..super::BUTTON_VIS
        })
        .with_padding((5., 5., 0., 5.))
        .with_alignment(ui::Alignment::Min, ui::Alignment::Max)
        .with_offset(32., None)
        .build();

    let achievement_box = ui::containers::VerticalBox::new()
        .to_element_builder(super::super::achievements::ACHIEVEMENT_BOX, ctx)
        .with_alignment(ui::Alignment::Center, ui::Alignment::Max)
        .with_offset(0., -25.)
        .with_size(
            ui::Size::Fill(0., f32::INFINITY),
            ui::Size::Shrink(0., f32::INFINITY),
        )
        .build();

    let tutorial_box = ui::containers::VerticalBox::new()
        .to_element_builder(super::super::tutorial::TUTORIAL_BOX, ctx)
        .with_alignment(ui::Alignment::Max, ui::Alignment::Center)
        .with_offset(-8., None)
        .with_size(
            ui::Size::Shrink(0., f32::INFINITY),
            ui::Size::Shrink(0., f32::INFINITY),
        )
        .build();

    Ok(ui::containers::StackBox::new()
        .to_element_builder(0, ctx)
        .with_child(achievement_box)
        .with_child(tutorial_box)
        .with_child(data_box)
        .with_child(slot_box)
        .with_child(spell_box)
        .with_child(cog_icon)
        .as_fill()
        .build())
}

pub fn create_spellslot(ctx: &ggez::Context, i: usize) -> ui::UiElement<game_state::GameMessage> {
    let mana = graphics::Image::from_path(ctx, "/sprites/spells/mana.png")
        .expect("[ERROR] Could not unpack mana symbol. Aborting.")
        .to_element_builder(0, ctx)
        .scaled(2., 2.)
        .build();

    let mut col = graphics::Color::from_rgb_u32(PALETTE[0]);
    col.a = 0.96;

    let progress = Covering::new(col, 0.)
        .to_element_builder(0, ctx)
        .with_message_handler(move |message_set, _layout, transitions| {
            for message in message_set {
                if let ui::UiMessage::Extern(game_state::GameMessage::UpdateSpellSlots(
                    index,
                    value,
                )) = message
                {
                    if *index == i {
                        transitions.push_back(
                            ui::Transition::new(Duration::ZERO)
                                .with_new_content(Covering::new(col, *value as f32 / 32.)),
                        );
                    }
                }
            }
        })
        .as_fill()
        .build();

    let mut stack = ui::containers::StackBox::new();
    let mana_layout = mana.get_layout();
    stack.add(progress);
    stack.add(mana);

    stack
        .to_element_builder(ID_MANA_SLOT, ctx)
        .with_wrapper_layout(mana_layout)
        .build()
}

/// A ui-element that covers another element by a certain amount.
/// Change the covering amount by using messages and content-changing transitions.
pub struct Covering {
    /// The covering percentage, between 0 and 1.
    covering: f32,
    /// The color of the covering.
    color: graphics::Color,
}

impl Covering {
    /// Creates a new covering.
    pub fn new(color: graphics::Color, covering: f32) -> Self {
        Self { covering, color }
    }
}

impl<T: Copy + Eq + Hash> ui::UiContent<T> for Covering {
    fn draw_content(
        &mut self,
        _ctx: &mut ggez::Context,
        canvas: &mut graphics::Canvas,
        param: ui::UiDrawParam,
    ) {
        let mut target_mod = param.target;
        target_mod.y += (1. - self.covering) * target_mod.h;
        target_mod.h *= self.covering;
        canvas.draw(
            &graphics::Quad,
            param.param.dest_rect(target_mod).color(self.color),
        );
    }
}
