use std::time::Duration;

use ggez::graphics;
use mooeye::{ui_element::UiContainer, *};

use super::game_state;
use crate::PALETTE;

pub fn handle_wave_menu(
    messages: &game_state::MessageSet,
    gui: &mut mooeye::UiElement<game_state::GameMessage>,
    ctx: &ggez::Context,
    director: &mut game_state::director::Director,
    data: &mut game_state::game_data::GameData,
    caster: &mut game_state::components::SpellCaster,
) {
    // if neccessary: Spawn wave menu
    if messages.iter().any(|message| {
        matches!(
            message,
            &UiMessage::Extern(game_state::GameMessage::NextWave(_))
        )
    }) {
        gui.add_element(
            0,
            construct_wave_menu(ctx, director.get_wave() as i32, &director.get_enemies()),
        );
    }

    // close wave menu and activate next wave
    if messages.contains(&UiMessage::Triggered(201)) {
        gui.remove_elements(200);
        // initialize next wave from director
        director.next_wave();
        // create wave announcer
        gui.add_element(0, construct_wave_announcer(ctx, director.get_wave()));
    }

    // Add spell slot
    if messages.contains(&UiMessage::Triggered(202)) && caster.can_add() && data.spend(250) {
        caster.add_slot();
        gui.add_element(
            50,
            super::game_ui::create_spellslot(ctx, caster.get_slots() - 1),
        );
    }

    // spellbook update
    if messages.contains(&UiMessage::Triggered(203)) {
        gui.remove_elements(61);
        let mut wrapbox = mooeye::containers::HorizontalBox::new();
        for spell in caster.get_spells() {
            wrapbox.add(spell.info_element_small(ctx));
        }
        gui.add_element(60, wrapbox
            .to_element(0, ctx));
    }

    // reroll enemies
    if messages.contains(&UiMessage::Triggered(204)) && data.spend(50) {
        // reroll enemies
        director.reroll_wave_enemies();
        // recreate UI
        gui.remove_elements(200);
        gui.add_element(
            0,
            construct_wave_menu(ctx, director.get_wave() as i32, &director.get_enemies()),
        );
    }
}

fn construct_wave_menu(
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
        .scaled(4., 4.)
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
        .scaled(4., 4.)
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
        .to_element_builder(203, ctx)
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
        .to_element_builder(204, ctx)
        .as_shrink()
        .scaled(4., 4.)
        .with_trigger_key(ggez::winit::event::VirtualKeyCode::O)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .with_tooltip(
            graphics::Text::new(
                graphics::TextFragment::new(
                    "Purchase an additional town building.\n[O/TODO]\nCost: 200g",
                )
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
        .scaled(4., 4.)
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

fn construct_wave_announcer(ctx: &ggez::Context, wave: u32) -> UiElement<game_state::GameMessage> {
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
