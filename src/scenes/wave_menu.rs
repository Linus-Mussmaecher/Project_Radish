use std::time::Duration;

use ggez::graphics;
use mooeye::{ui_element::UiContainer, *};

use crate::PALETTE;

pub fn construct_wave_menu(
    ctx: &ggez::Context,
    wave_survived: i32,
) -> UiElement<crate::game_state::GameMessage> {
    // title
    let wave_info = graphics::Text::new(
        graphics::TextFragment::new(format!("You have survived wave {}.", wave_survived))
            .color(graphics::Color::from_rgb_u32(PALETTE[6])),
    )
    .set_font("Retro")
    .set_scale(32.)
    .to_owned()
    .to_element_builder(0, ctx)
    .build();

    let mana = graphics::Image::from_path(ctx, "/sprites/ui/mana_add.png")
        .expect("[ERROR] Missing mana sprite.")
        .to_element_builder(202, ctx)
        .as_shrink()
        .scaled(2., 2.)
        .with_trigger_key(ggez::winit::event::VirtualKeyCode::U)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .with_tooltip(
            graphics::Text::new(
                graphics::TextFragment::new("Purchase an additional spell slot.\nCost: 250g")
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

        let house = graphics::Image::from_path(ctx, "/sprites/ui/house_add.png")
        .expect("[ERROR] Missing house sprite.")
        .to_element_builder(202, ctx)
        .as_shrink()
        .scaled(2., 2.)
        .with_trigger_key(ggez::winit::event::VirtualKeyCode::U)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .with_tooltip(
            graphics::Text::new(
                graphics::TextFragment::new("Purchase an additional town building.\nCost: 200g")
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

    let reroll = graphics::Image::from_path(ctx, "/sprites/ui/reroll.png")
        .expect("[ERROR] Missing reroll sprite.")
        .to_element_builder(203, ctx)
        .as_shrink()
        .scaled(2., 2.)
        .with_trigger_key(ggez::winit::event::VirtualKeyCode::I)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .with_tooltip(
            graphics::Text::new(
                graphics::TextFragment::new("Reroll the enemy selection.\nCost: 50g")
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

    let mut upgrade_box = containers::HorizontalBox::new();
    upgrade_box.add(mana);
    upgrade_box.add(house);
    upgrade_box.add(reroll);
    let upgrade_box = upgrade_box.to_element_builder(0, ctx).build();

    let next = graphics::Text::new(
        graphics::TextFragment::new("Next Wave").color(graphics::Color::from_rgb_u32(PALETTE[6])),
    )
    .set_font("Retro")
    .set_scale(32.)
    .to_owned()
    .to_element_builder(201, ctx)
    .with_trigger_key(ggez::winit::event::VirtualKeyCode::N)
    .with_visuals(super::BUTTON_VIS)
    .with_hover_visuals(super::BUTTON_HOVER_VIS)
    .build();

    // Container

    let mut menu_box = containers::VerticalBox::new();
    menu_box.add(wave_info);
    menu_box.add(upgrade_box);
    menu_box.add(next);
    menu_box.spacing = 25.;
    let menu_box = menu_box
        .to_element_builder(200, ctx)
        .with_visuals(super::BUTTON_VIS)
        .with_alignment(ui_element::Alignment::Center, ui_element::Alignment::Center)
        .with_padding((25., 25., 25., 25.))
        .build();

    menu_box
}

pub fn construct_wave_announcer(
    ctx: &ggez::Context,
    wave: u32,
) -> UiElement<crate::game_state::GameMessage> {
    let mut dur = containers::DurationBox::new(
        Duration::from_secs(5),
        graphics::Text::new(
            graphics::TextFragment::new(format!("Wave {}", wave))
                .color(graphics::Color::from_rgb_u32(PALETTE[14])),
        )
        .set_scale(48.)
        .set_font("Retro")
        .to_owned()
        .to_element_builder(0, ctx)
        .as_shrink()
        .build(),
    )
    .to_element_builder(0, ctx)
    .as_shrink()
    .with_alignment(ui_element::Alignment::Center, ui_element::Alignment::Center)
    .build();

    let ctr_layout = dur.get_layout();

    dur.add_transition(ui_element::Transition::new(Duration::from_secs(2)));

    dur.add_transition(
        ui_element::Transition::new(Duration::from_secs(3)).with_new_layout(ui_element::Layout {
            y_alignment: ui_element::Alignment::Min,
            ..ctr_layout
        }),
    );

    dur
}
