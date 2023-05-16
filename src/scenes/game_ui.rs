use ggez::{graphics, GameError};

use mooeye::ui_element::UiContainer;
use mooeye::*;
use std::time::Duration;

use crate::PALETTE;

use crate::game_state;

/// Constructs the UiElement that forms the main UI of the game.
/// Consists of
///  - Menu button
///  - Gold & City health display
///  - Cooldown display for the players spell slots
///  - TODO: Indicator for currently equipped spells
///  - Vertical box to display messages (achievements etc.)
pub fn construct_game_ui(
    ctx: &ggez::Context,
) -> Result<UiElement<game_state::GameMessage>, GameError> {
    // main box
    let mut main_box = containers::StackBox::new();

    // options icon
    let cog_icon = graphics::Image::from_path(ctx, "/sprites/ui/cog.png")?
        .to_element_builder(1, ctx)
        .with_trigger_key(ggez::winit::event::VirtualKeyCode::F10)
        .with_visuals(super::BUTTON_VIS)
        .with_alignment(ui_element::Alignment::Max, ui_element::Alignment::Max)
        .scaled(2., 2.)
        .with_offset(-10., -10.)
        .as_shrink()
        .build();

    main_box.add(cog_icon);

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
    .with_message_handler(|message_set, _, transitions| {
        for message in message_set {
            if let ui_element::UiMessage::Extern(game_state::GameMessage::UpdateGold(new_gold)) =
                message
            {
                transitions.push_back(
                    ui_element::Transition::new(Duration::ZERO).with_new_content(
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

    let mut gold_box = containers::HorizontalBox::new();
    gold_box.add(gold_icon);
    gold_box.add(gold_text);
    let gold_box = gold_box
        .to_element_builder(0, ctx)
        .with_visuals(super::BUTTON_VIS)
        .with_alignment(ui_element::Alignment::Min, ui_element::Alignment::Min)
        .with_offset(10., 10.)
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
        .build();

    main_box.add(gold_box);

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
    .with_message_handler(|message_set, _, transitions| {
        for message in message_set {
            if let ui_element::UiMessage::Extern(game_state::GameMessage::UpdateCityHealth(
                new_health,
            )) = message
            {
                transitions.push_back(
                    ui_element::Transition::new(Duration::ZERO).with_new_content(
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

    let mut city_box = containers::HorizontalBox::new();
    city_box.add(city_display);
    city_box.add(city_text);
    let city_box = city_box
        .to_element_builder(0, ctx)
        .with_visuals(super::BUTTON_VIS)
        .with_alignment(ui_element::Alignment::Max, ui_element::Alignment::Min)
        .with_offset(-10., 10.)
        .with_tooltip(
            graphics::Text::new(
                graphics::TextFragment::new("The health your city currently has left.")
                    .color(graphics::Color::from_rgb_u32(PALETTE[6])),
            )
            .set_scale(24.)
            .set_font("Retro")
            .to_owned()
            .to_element_builder(0, ctx)
            .with_visuals(super::BUTTON_VIS)
            .build(),
        )
        .build();

    main_box.add(city_box);

    // Spells

    let mut spell_box = containers::HorizontalBox::new();

    for i in 0..4 {
        spell_box.add(create_spellslot(ctx, i));
    }

    let spell_box = spell_box
        .to_element_builder(50, ctx)
        .with_visuals(mooeye::ui_element::Visuals {
            border_widths: [0., 3., 3., 3.],
            corner_radii: [0., 3., 3., 0.],
            ..super::BUTTON_VIS
        })
        .with_alignment(ui_element::Alignment::Center, ui_element::Alignment::Min)
        .with_offset(-10., 0.)
        .build();

    main_box.add(spell_box);

    let message_box = containers::VerticalBox::new()
        .to_element_builder(100, ctx)
        .with_alignment(ui_element::Alignment::Center, ui_element::Alignment::Max)
        .with_offset(0., -25.)
        .with_size(
            ui_element::Size::Fill(0., f32::INFINITY),
            ui_element::Size::Shrink(0., f32::INFINITY),
        )
        .build();

    main_box.add(message_box);

    Ok(main_box
        .to_element_builder(0, ctx)
        .as_fill()
        .build())
}

pub fn create_spellslot(ctx: &ggez::Context, i: usize) -> UiElement<game_state::GameMessage> {
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
                if let ui_element::UiMessage::Extern(game_state::GameMessage::UpdateSpellSlots(
                    index,
                    value,
                )) = message
                {
                    if *index == i as usize {
                        transitions.push_back(
                            ui_element::Transition::new(Duration::ZERO)
                                .with_new_content(Covering::new(col, *value as f32 / 32.)),
                        );
                    }
                }
            }
        })
        .as_fill()
        .build();

    let mut stack = containers::StackBox::new();
    let mana_layout = mana.get_layout();
    stack.add(progress);
    stack.add(mana);

    stack
        .to_element_builder(0, ctx)
        .with_wrapper_layout(mana_layout)
        .build()
}

/// A ui-element that covers another element by a certain amount.
/// Change the covering amount by using messages and content-changing transitions.
struct Covering {
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

impl UiContent<game_state::GameMessage> for Covering {
    fn draw_content(
        &mut self,
        _ctx: &mut ggez::Context,
        canvas: &mut graphics::Canvas,
        param: ui_element::UiDrawParam,
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
