use std::time::Duration;

use ggez::graphics;
use mooeye::{ui_element::UiContainer, *};

use super::game_state;
use crate::PALETTE;

pub fn construct_wave_menu(
    ctx: &ggez::Context,
    wave_survived: i32,
    enemies: &[&game_state::EnemyTemplate],
) -> UiElement<game_state::GameMessage> {
    // ---- Title ----

    let title = graphics::Text::new(
        graphics::TextFragment::new("Brief Respite")
            .color(graphics::Color::from_rgb_u32(PALETTE[8])),
    )
    .set_font("Retro")
    .set_scale(48.)
    .to_owned()
    .to_element_builder(0, ctx)
    .build();

    let wave_info = graphics::Text::new(
        graphics::TextFragment::new(format!("You have survived wave {}.", wave_survived))
            .color(graphics::Color::from_rgb_u32(PALETTE[6])),
    )
    .set_font("Retro")
    .set_scale(32.)
    .to_owned()
    .to_element_builder(0, ctx)
    .build();

    // ---- Enemy display and reroll ----

    let enemy_info = graphics::Text::new(
        graphics::TextFragment::new("Approaching enemies:")
            .color(graphics::Color::from_rgb_u32(PALETTE[6])),
    )
    .set_font("Retro")
    .set_scale(32.)
    .to_owned()
    .to_element_builder(0, ctx)
    .build();

    let mut enemy_box = containers::HorizontalBox::new();

    for template in enemies {
        enemy_box.add(
            template
                .icon
                .clone()
                .to_element_builder(0, ctx)
                .scaled(4., 4.)
                .with_tooltip(
                    graphics::Text::new(
                        graphics::TextFragment::new(&template.name)
                            .color(graphics::Color::from_rgb_u32(PALETTE[8]))
                            .scale(36.),
                    )
                    .add(graphics::TextFragment::new("\n"))
                    .add(
                        graphics::TextFragment::new(&template.description)
                            .color(graphics::Color::from_rgb_u32(PALETTE[6]))
                            .scale(24.),
                    )
                    .set_bounds(ggez::glam::Vec2::new(300., 200.))
                    .set_wrap(true)
                    .set_font("Retro")
                    .to_owned()
                    .to_element_builder(0, ctx)
                    .with_visuals(super::BUTTON_VIS)
                    .build(),
                )
                .build(),
        );
    }

    let enemy_box = enemy_box
        .to_element_builder(0, ctx)
        .with_visuals(super::BUTTON_VIS)
        .build();

    let reroll = graphics::Image::from_path(ctx, "/sprites/ui/reroll.png")
        .expect("[ERROR] Missing reroll sprite.")
        .to_element_builder(204, ctx)
        .as_shrink()
        .scaled(2., 2.)
        .with_padding((10., 10., 10., 10.))
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

    let mut row1 = containers::HorizontalBox::new();
    row1.add(enemy_box);
    row1.add(reroll);

    let row1 = row1
        .to_element_builder(0, ctx)
        .with_alignment(ui_element::Alignment::Center, None)
        .build();

    // ---- Other town actions ----

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
                graphics::TextFragment::new("Purchase an additional spell slot.\n[U]\nCost: 250g")
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

        let spellbook = graphics::Image::from_path(ctx, "/sprites/ui/book.png")
        .expect("[ERROR] Missing spellbook sprite.")
        .to_element_builder(202, ctx)
        .as_shrink()
        .scaled(4., 4.)
        .with_trigger_key(ggez::winit::event::VirtualKeyCode::I)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .with_tooltip(
            graphics::Text::new(
                graphics::TextFragment::new("Look at your spellbook.\n[I/TODO]")
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
        .to_element_builder(203, ctx)
        .as_shrink()
        .scaled(2., 2.)
        .with_trigger_key(ggez::winit::event::VirtualKeyCode::O)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .with_tooltip(
            graphics::Text::new(
                graphics::TextFragment::new("Purchase an additional town building.\n[O/TODO]\nCost: 200g")
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

        let next = graphics::Image::from_path(ctx, "/sprites/ui/next.png")
            .expect("[ERROR] Missing reroll sprite.")
            .to_element_builder(201, ctx)
            .as_shrink()
            .scaled(2., 2.)
            .with_trigger_key(ggez::winit::event::VirtualKeyCode::P)
            .with_visuals(super::BUTTON_VIS)
            .with_hover_visuals(super::BUTTON_HOVER_VIS)
            .with_tooltip(
                graphics::Text::new(
                    graphics::TextFragment::new("Start the next wave!\n[P]")
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

    let mut row2 = containers::HorizontalBox::new();
    row2.spacing = 16.;
    row2.add(mana);
    row2.add(spellbook);
    row2.add(house);
    row2.add(next);

    let row2 = row2
        .to_element_builder(0, ctx)
        .with_alignment(ui_element::Alignment::Center, None)
        .build();

    let spacing1 =
        ().to_element_builder(0, ctx)
            .with_size(None, ui_element::Size::Shrink(8., 8.))
            .build();
    let spacing2 =
        ().to_element_builder(0, ctx)
            .with_size(None, ui_element::Size::Shrink(8., 8.))
            .build();

    // Container
    let mut menu_box = containers::VerticalBox::new();
    menu_box.add(title);
    menu_box.add(wave_info);
    menu_box.add(spacing1);
    menu_box.add(enemy_info);
    menu_box.add(row1);
    menu_box.add(spacing2);
    menu_box.add(row2);
    menu_box.spacing = 16.;
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
) -> UiElement<game_state::GameMessage> {
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
