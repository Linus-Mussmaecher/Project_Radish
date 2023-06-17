use std::time::Duration;

use ggez::graphics;
use mooeye::{ui_element::UiContainer, *};

use super::game_state;
use crate::{scenes::game_state::components::buildings, PALETTE};

const ID_WAVE_MENU: u32 = 200;
const ID_WAVE_SUBMENU_CONT: u32 = 210;
const ID_WAVE_SUBMENU: u32 = 220;

const ID_SPELLS: u32 = 201;
const ID_ENEMIES: u32 = 202;
const ID_HOUSE: u32 = 203;
pub const ID_NEXT_WAVE: u32 = 204;

const ID_REROLL: u32 = 221;

const ID_BUILDINGS_START: u32 = 222;

const ID_SPELL_EQUIP_START: u32 = 230;
const ID_SPELL_AVAIL_START: u32 = 240;

pub fn handle_wave_menu(
    messages: &game_state::MessageSet,
    gui: &mut mooeye::UiElement<game_state::GameMessage>,
    ctx: &ggez::Context,
    director: &mut game_state::director::Director,
    data: &mut game_state::game_data::GameData,
    caster: &mut game_state::components::SpellCaster,
    spell_pool: &mut game_state::components::spell::SpellPool,
    buildings: &mut game_state::components::buildings::Buildings,
) {
    // if neccessary: Spawn wave menu
    if messages.iter().any(|message| {
        matches!(
            message,
            &UiMessage::Extern(game_state::GameMessage::NextWave(_))
        )
    }) {
        gui.add_element(0, construct_wave_menu(ctx, director.get_wave()));
        // sync spellslots in case a building got destroyed
        sync_spellslots(ctx, gui, caster, buildings);
    }

    // enemies submenu
    if messages.contains(&UiMessage::Triggered(ID_ENEMIES)) {
        gui.remove_elements(ID_WAVE_SUBMENU);
        gui.add_element(
            ID_WAVE_SUBMENU_CONT,
            construct_enemies_menu(ctx, &director, buildings),
        );
    }

    // spells submenu
    if messages.contains(&UiMessage::Triggered(ID_SPELLS)) {
        gui.remove_elements(ID_WAVE_SUBMENU);
        gui.add_element(
            ID_WAVE_SUBMENU_CONT,
            construct_spell_menu(ctx, caster, spell_pool, buildings),
        );
    }

    // build submenu
    if messages.contains(&UiMessage::Triggered(ID_HOUSE)) {
        gui.remove_elements(ID_WAVE_SUBMENU);
        gui.add_element(
            ID_WAVE_SUBMENU_CONT,
            construct_buildings_menu(ctx, buildings),
        );
    }

    // unlock and equip spells
    for message in messages {
        // remember if anything has changed
        let mut triggered = false;
        match message {
            // check for clicks if a spell in the shop index-range
            &UiMessage::Triggered(id)
                if id >= ID_SPELL_AVAIL_START
                    && id < ID_SPELL_AVAIL_START + spell_pool.1.len() as u32 =>
            {
                let mut purchased = false;
                // calculate index
                let index = (id - ID_SPELL_AVAIL_START) as usize;
                // check if a spell is at that index
                if let Some(template) = spell_pool.1.get_mut(index) {
                    // if spell was not yet purchased, attempt to purchase it
                    if template.level == 0
                        && buildings.target[buildings::BuildingType::MAGEGUILD as usize]
                            >= template.guild_condition
                        && data.spend(template.cost)
                    {
                        template.level = 1;
                        purchased = true;
                    }
                    // if spell is (now) unlocked, store a copy
                    if template.level > 0 {
                        spell_pool.0 = Some(template.spell.clone());
                        triggered = true;
                    }
                }
                // if anything was purchased, increase cost of remaining spells
                if purchased {
                    for spell in spell_pool.1.iter_mut() {
                        if spell.cost > 0 {
                            spell.cost += 20;
                        }
                    }
                }
            }

            // check for clicks if a spell in the equipped spell index-range
            &UiMessage::Triggered(id)
                if id >= ID_SPELL_EQUIP_START && id < ID_SPELL_EQUIP_START + 4 =>
            {
                // calculate index
                let index = (id - ID_SPELL_EQUIP_START) as usize;
                // check if a spell is stored and remove it
                if let Some(stored) = spell_pool.0.take() {
                    // equip the spell
                    caster.equip_spell(index, stored);
                    triggered = true;
                }
            }
            _ => {}
        }
        // reload menu if neccessary
        if triggered {
            gui.remove_elements(ID_WAVE_SUBMENU);
            gui.add_element(
                ID_WAVE_SUBMENU_CONT,
                construct_spell_menu(ctx, caster, spell_pool, buildings),
            );
        }
    }

    // reroll
    if messages.contains(&UiMessage::Triggered(ID_REROLL))
        && data.spend(director.get_reroll_cost())
        && buildings.target[buildings::BuildingType::WATCHTOWER as usize] > 0
    {
        director.reroll_wave_enemies();
        gui.remove_elements(ID_WAVE_SUBMENU);
        gui.add_element(
            ID_WAVE_SUBMENU_CONT,
            construct_enemies_menu(ctx, &director, buildings),
        );
    }

    // buildings
    for i in 0..buildings::BUILDING_TYPES {
        let index = buildings.target[i] as usize;
        if messages.contains(&UiMessage::Triggered(ID_BUILDINGS_START + i as u32))
            && index < buildings::BUILDING_MAX_LEVEL
            && data.spend(
                super::game_state::components::buildings::get_building_info(i).level_costs[index]
                    as i32,
            )
        {
            buildings.target[i] += 1;
            // if it is the mana well, sync spell slots
            if i == 2 && caster.can_add() {
                sync_spellslots(ctx, gui, caster, buildings);
            }
            // rebuild menu
            gui.remove_elements(ID_WAVE_SUBMENU);
            gui.add_element(
                ID_WAVE_SUBMENU_CONT,
                construct_buildings_menu(ctx, buildings),
            );
        }
    }

    // close wave menu and activate next wave
    if messages.contains(&UiMessage::Triggered(ID_NEXT_WAVE)) {
        gui.remove_elements(ID_WAVE_MENU);
        // initialize next wave from director
        director.next_wave();
        // create wave announcer
        gui.add_element(0, construct_wave_announcer(ctx, director.get_wave()));
        // make sure spells are correct
        gui.remove_elements(super::game_ui::ID_SPELL_BAR_CHILDREN);
        gui.add_element(
            super::game_ui::ID_SPELL_BAR,
            caster
                .get_spells()
                .iter()
                .fold(
                    containers::HorizontalBox::new_spaced(16.)
                        .to_element_builder(super::game_ui::ID_SPELL_BAR_CHILDREN, ctx),
                    |loadout, spell| loadout.with_child(spell.info_element_small(0, ctx)),
                )
                .build(),
        );
    }
}

/// Construct the last row all three submenus share
fn construct_wave_menu(
    ctx: &ggez::Context,
    wave_survived: u32,
) -> UiElement<game_state::GameMessage> {
    let enemies = graphics::Image::from_path(ctx, "/sprites/ui/enemies.png")
        .expect("[ERROR] Missing enemies menu sprite.")
        .to_element_builder(ID_ENEMIES, ctx)
        .as_shrink()
        .scaled(4., 4.)
        .with_trigger_key(ggez::winit::event::VirtualKeyCode::U)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .with_tooltip(
            graphics::Text::new(
                graphics::TextFragment::new("Look at approaching enemies.\n[U]")
                    .color(graphics::Color::from_rgb_u32(PALETTE[6])),
            )
            .set_scale(24.)
            .set_font("Retro")
            .to_owned()
            .to_element_builder(0, ctx)
            .with_visuals(super::BUTTON_VIS)
            .build(),
        )
        .with_message_handler(|messages, _, transitions| {
            if messages.contains(&UiMessage::Triggered(ID_ENEMIES)) {
                transitions.push_back(
                    ui_element::Transition::new(Duration::ZERO)
                        .with_new_visuals(ui_element::Visuals {
                            border_widths: [3., 3., 0., 3.],
                            corner_radii: [3., 0., 0., 3.],
                            ..super::BUTTON_VIS
                        })
                        .with_new_hover_visuals(None),
                )
            }
            if messages.contains(&UiMessage::Triggered(ID_SPELLS))
                || messages.contains(&UiMessage::Triggered(ID_HOUSE))
            {
                transitions.push_back(
                    ui_element::Transition::new(Duration::ZERO)
                        .with_new_visuals(super::BUTTON_VIS)
                        .with_new_hover_visuals(Some(super::BUTTON_HOVER_VIS)),
                )
            }
        })
        .build();

    let spellbook = graphics::Image::from_path(ctx, "/sprites/ui/book.png")
        .expect("[ERROR] Missing spellbook sprite.")
        .to_element_builder(ID_SPELLS, ctx)
        .as_shrink()
        .scaled(4., 4.)
        .with_trigger_key(ggez::winit::event::VirtualKeyCode::I)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .with_tooltip(
            graphics::Text::new(
                graphics::TextFragment::new("Look at your spellbook.\n[I]")
                    .color(graphics::Color::from_rgb_u32(PALETTE[6])),
            )
            .set_scale(24.)
            .set_font("Retro")
            .to_owned()
            .to_element_builder(0, ctx)
            .with_visuals(super::BUTTON_VIS)
            .build(),
        )
        .with_message_handler(|messages, _, transitions| {
            if messages.contains(&UiMessage::Triggered(ID_SPELLS)) {
                transitions.push_back(
                    ui_element::Transition::new(Duration::ZERO)
                        .with_new_visuals(ui_element::Visuals {
                            border_widths: [3., 3., 0., 3.],
                            corner_radii: [3., 0., 0., 3.],
                            ..super::BUTTON_VIS
                        })
                        .with_new_hover_visuals(None),
                )
            }
            if messages.contains(&UiMessage::Triggered(ID_ENEMIES))
                || messages.contains(&UiMessage::Triggered(ID_HOUSE))
            {
                transitions.push_back(
                    ui_element::Transition::new(Duration::ZERO)
                        .with_new_visuals(super::BUTTON_VIS)
                        .with_new_hover_visuals(Some(super::BUTTON_HOVER_VIS)),
                )
            }
        })
        .build();

    let house = graphics::Image::from_path(ctx, "/sprites/ui/house_add.png")
        .expect("[ERROR] Missing house sprite.")
        .to_element_builder(ID_HOUSE, ctx)
        .as_shrink()
        .scaled(4., 4.)
        .with_trigger_key(ggez::winit::event::VirtualKeyCode::O)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .with_tooltip(
            graphics::Text::new(
                graphics::TextFragment::new("Purchase an additional town building.\n[O]")
                    .color(graphics::Color::from_rgb_u32(PALETTE[6])),
            )
            .set_scale(24.)
            .set_font("Retro")
            .to_owned()
            .to_element_builder(0, ctx)
            .with_visuals(super::BUTTON_VIS)
            .build(),
        )
        .with_message_handler(|messages, _, transitions| {
            if messages.contains(&UiMessage::Triggered(ID_HOUSE)) {
                transitions.push_back(
                    ui_element::Transition::new(Duration::ZERO)
                        .with_new_visuals(ui_element::Visuals {
                            border_widths: [3., 3., 0., 3.],
                            corner_radii: [3., 0., 0., 3.],
                            ..super::BUTTON_VIS
                        })
                        .with_new_hover_visuals(None),
                )
            }
            if messages.contains(&UiMessage::Triggered(ID_ENEMIES))
                || messages.contains(&UiMessage::Triggered(ID_SPELLS))
            {
                transitions.push_back(
                    ui_element::Transition::new(Duration::ZERO)
                        .with_new_visuals(super::BUTTON_VIS)
                        .with_new_hover_visuals(Some(super::BUTTON_HOVER_VIS)),
                )
            }
        })
        .build();

    let next = graphics::Image::from_path(ctx, "/sprites/ui/next.png")
        .expect("[ERROR] Missing next sprite.")
        .to_element_builder(ID_NEXT_WAVE, ctx)
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

    let nav_row = containers::HorizontalBox::new_spaced(16.)
        .to_element_builder(0, ctx)
        .with_child(enemies)
        .with_child(spellbook)
        .with_child(house)
        .with_child(next)
        .with_alignment(ui_element::Alignment::Center, ui_element::Alignment::Min)
        .with_visuals(ui_element::Visuals {
            corner_radii: [3., 0., 0., 3.],
            border_widths: [0., 0., 3., 0.],
            ..super::BUTTON_VIS
        })
        .with_padding((8., 8., 0., 8.))
        .with_size(
            ui_element::Size::Fill(0., f32::INFINITY),
            ui_element::Size::Shrink(0., f32::INFINITY),
        )
        .build();

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

    let submenu_cont = containers::StackBox::new()
        .to_element_builder(ID_WAVE_SUBMENU_CONT, ctx)
        .with_child(
            containers::VerticalBox::new()
                .to_element_builder(ID_WAVE_SUBMENU, ctx)
                .with_child(title)
                .with_child(wave_info)
                .build(),
        )
        .as_fill()
        .with_padding((25., 25., 25., 25.))
        .build();

    containers::VerticalBox::new_spaced(8.)
        .to_element_builder(ID_WAVE_MENU, ctx)
        .with_child(nav_row)
        .with_child(submenu_cont)
        .with_padding((3., 3., 3., 3.))
        .with_visuals(super::BUTTON_VIS)
        .with_size(ui_element::Size::Shrink(580., f32::INFINITY), None)
        .with_alignment(ui_element::Alignment::Center, ui_element::Alignment::Min)
        .with_offset(None, 128.)
        .build()
}

fn construct_enemies_menu(
    ctx: &ggez::Context,
    director: &super::game_state::director::Director,
    buildings: &mut game_state::components::buildings::Buildings,
) -> UiElement<game_state::GameMessage> {
    // ---- Enemy display and reroll ----

    let title = graphics::Text::new(
        graphics::TextFragment::new("Approaching Enemies")
            .color(graphics::Color::from_rgb_u32(PALETTE[8])),
    )
    .set_font("Retro")
    .set_scale(40.)
    .to_owned()
    .to_element_builder(0, ctx)
    .build();

    let mut enemy_box = containers::HorizontalBox::new();

    for template in director.get_enemies() {
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

    let reroll = if buildings.target[buildings::BuildingType::WATCHTOWER as usize] == 0 {
        ().to_element(0, ctx)
    } else {
        graphics::Image::from_path(ctx, "/sprites/ui/reroll.png")
            .expect("[ERROR] Missing reroll sprite.")
            .to_element_builder(ID_REROLL, ctx)
            .as_shrink()
            .scaled(4., 4.)
            .with_padding((10., 10., 10., 10.))
            .with_trigger_key(ggez::winit::event::VirtualKeyCode::M)
            .with_visuals(super::BUTTON_VIS)
            .with_hover_visuals(super::BUTTON_HOVER_VIS)
            .with_tooltip(
                graphics::Text::new(
                    graphics::TextFragment::new(if buildings.target[0] == 0 {
                        "Purchase the watchtower to reroll enemy waves.".to_owned()
                    } else {
                        format!(
                            "Reroll the enemy selection.\n[M]\nCost: {}g",
                            director.get_reroll_cost()
                        )
                    })
                    .color(graphics::Color::from_rgb_u32(PALETTE[6])),
                )
                .set_scale(24.)
                .set_font("Retro")
                .to_owned()
                .to_element_builder(0, ctx)
                .with_visuals(super::BUTTON_VIS)
                .build(),
            )
            .build()
    };

    let enemy_row = containers::HorizontalBox::new()
        .to_element_builder(0, ctx)
        .with_child(enemy_box)
        .with_child(reroll)
        .with_alignment(ui_element::Alignment::Center, None)
        .build();

    // Container
    containers::VerticalBox::new_spaced(16.)
        .to_element_builder(ID_WAVE_SUBMENU, ctx)
        .with_child(title)
        .with_child(enemy_row)
        .with_alignment(ui_element::Alignment::Center, ui_element::Alignment::Center)
        .as_fill()
        .build()
}

fn construct_spell_menu(
    ctx: &ggez::Context,
    caster: &mut game_state::components::SpellCaster,
    spell_pool: &game_state::components::spell::SpellPool,
    buildings: &game_state::components::buildings::Buildings,
) -> UiElement<game_state::GameMessage> {
    // ---- Title ----

    let title = graphics::Text::new(
        graphics::TextFragment::new("Book of Spells")
            .color(graphics::Color::from_rgb_u32(PALETTE[8])),
    )
    .set_font("Retro")
    .set_scale(40.)
    .to_owned()
    .to_element_builder(0, ctx)
    .build();

    // available spells

    let available_title = graphics::Text::new(
        graphics::TextFragment::new("Available:").color(graphics::Color::from_rgb_u32(PALETTE[6])),
    )
    .set_font("Retro")
    .set_scale(24.)
    .to_owned()
    .to_element_builder(0, ctx)
    .build();

    let available = spell_pool
        .1
        .iter()
        .enumerate()
        .fold(
            containers::GridBox::new_spaced(6, 4, 8., 8.),
            |mut gbox, (ind, template)| {
                gbox.add(
                    template.info_element_small(ID_SPELL_AVAIL_START + ind as u32, ctx, buildings),
                    ind % 6,
                    ind / 6,
                )
                .expect("Unexpected Index out of bounds when adding spell pool to grid.");
                gbox
            },
        )
        .to_element_builder(0, ctx)
        .build();

    let loadout_title = graphics::Text::new(
        graphics::TextFragment::new("Loadout:").color(graphics::Color::from_rgb_u32(PALETTE[6])),
    )
    .set_font("Retro")
    .set_scale(24.)
    .to_owned()
    .to_element_builder(0, ctx)
    .build();

    // equipped spells

    let loadout = caster
        .get_spells()
        .iter()
        .enumerate()
        .fold(
            containers::HorizontalBox::new_spaced(16.).to_element_builder(0, ctx),
            |loadout, (index, spell)| {
                loadout
                    .with_child(spell.info_element_small(ID_SPELL_EQUIP_START + index as u32, ctx))
            },
        )
        .with_alignment(ui_element::Alignment::Center, None)
        .as_shrink()
        .build();

    containers::VerticalBox::new_spaced(16.)
        .to_element_builder(ID_WAVE_SUBMENU, ctx)
        .with_child(title)
        .with_child(available_title)
        .with_child(available)
        .with_child(loadout_title)
        .with_child(loadout)
        .with_alignment(ui_element::Alignment::Center, ui_element::Alignment::Center)
        .as_fill()
        .build()
}

fn construct_buildings_menu(
    ctx: &ggez::Context,
    buildings: &mut game_state::components::buildings::Buildings,
) -> UiElement<game_state::GameMessage> {
    // ---- Enemy display and reroll ----

    let title = graphics::Text::new(
        graphics::TextFragment::new("Construct Buildings")
            .color(graphics::Color::from_rgb_u32(PALETTE[8])),
    )
    .set_font("Retro")
    .set_scale(40.)
    .to_owned()
    .to_element_builder(0, ctx)
    .build();

    let mut construct_box = containers::HorizontalBox::new_spaced(25.).to_element_builder(0, ctx);

    let icons = [
        "/sprites/ui/looking_glass.png",
        "/sprites/ui/potion.png",
        "/sprites/ui/mana_add.png",
    ];

    for i in 0..buildings::BUILDING_TYPES {
        let info = buildings::get_building_info(i);
        let build = graphics::Image::from_path(ctx, icons[i])
            .expect("[ERROR] Missing building sprite.")
            .to_element_builder(ID_BUILDINGS_START + i as u32, ctx)
            .as_shrink()
            .scaled(4., 4.)
            .with_padding((10., 10., 10., 10.))
            .with_visuals(super::BUTTON_VIS)
            .with_hover_visuals(super::BUTTON_HOVER_VIS)
            .with_tooltip(
                graphics::Text::new(
                    graphics::TextFragment::new(
                        if buildings.target[i] < buildings::BUILDING_MAX_LEVEL as u8 {
                            format!(
                                "{} the {}.\nCurrent level: {}\n{}\nCost: {}g",
                                if buildings.target[i] == 0 {
                                    "Construct"
                                } else {
                                    "Upgrade"
                                },
                                info.name,
                                buildings.target[i],
                                info.description,
                                info.level_costs[buildings.target[i] as usize],
                            )
                        } else {
                            format!("{} is fully upgraded.", info.name,)
                        },
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

        let level = graphics::Text::new(
            graphics::TextFragment::new(format!(" {} ", buildings.target[i]))
                .color(graphics::Color::from_rgb_u32(PALETTE[14])),
        )
        .set_scale(24.)
        .set_font("Retro")
        .to_owned()
        .to_element_builder(0, ctx)
        .with_padding((2., 2., 2., 2.))
        .with_visuals(ui_element::Visuals {
            background: graphics::Color::from_rgb_u32(PALETTE[4]),
            border: graphics::Color::from_rgb_u32(PALETTE[4]),
            border_widths: [0.; 4],
            corner_radii: [14.; 4],
        })
        .as_shrink()
        .with_alignment(ui_element::Alignment::Max, ui_element::Alignment::Max)
        .build();

        construct_box = construct_box.with_child(
            containers::StackBox::new()
                .to_element_builder(0, ctx)
                .with_child(level)
                .with_child(build)
                .build(),
        );
    }

    // Container
    containers::VerticalBox::new_spaced(16.)
        .to_element_builder(ID_WAVE_SUBMENU, ctx)
        .with_child(title)
        .with_child(construct_box.build())
        .with_alignment(ui_element::Alignment::Center, ui_element::Alignment::Center)
        .as_fill()
        .build()
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

fn sync_spellslots(
    ctx: &ggez::Context,
    gui: &mut mooeye::UiElement<game_state::GameMessage>,
    caster: &mut game_state::components::SpellCaster,
    buildings: &mut game_state::components::buildings::Buildings,
) {
    // game sync
    caster.set_extra_slots(buildings.target[buildings::BuildingType::MANAWELL as usize] as usize);
    // ui sync
    gui.remove_elements(super::game_ui::ID_MANA_SLOT);
    for i in 0..caster.get_slots() {
        gui.add_element(
            super::game_ui::ID_MANA_BAR,
            super::game_ui::create_spellslot(ctx, i),
        );
    }
}
