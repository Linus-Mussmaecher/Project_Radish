use std::time::Duration;

use good_web_game::graphics::{self, TextFragment};
use legion::EntityStore;
use mooeye::{ui, ui::UiContainer, ui::UiContent};

use super::game_state;
use crate::{scenes::game_state::components::buildings, PALETTE};

const ID_WAVE_MENU: u32 = 200;
const ID_WAVE_SUBMENU_CONT: u32 = 210;
const ID_WAVE_SUBMENU: u32 = 220;

pub const ID_SPELLS: u32 = 201;
pub const ID_ENEMIES: u32 = 202;
pub const ID_HOUSE: u32 = 203;
pub const ID_NEXT_WAVE: u32 = 204;

const ID_REROLL: u32 = 221;

const ID_BUILDINGS_START: u32 = 222;

const ID_SPELL_EQUIP_START: u32 = 230;
const ID_SPELL_AVAIL_START: u32 = 240;

pub fn handle_wave_menu(
    messages: &game_state::MessageSet,
    gui: &mut ui::UiElement<game_state::GameMessage>,
    ctx: &mut good_web_game::Context,
    gfx_ctx: &mut good_web_game::event::GraphicsContext,
    world: &mut legion::World,
    resources: &mut legion::Resources,
) {
    // if neccessary: Spawn wave menu
    let mut player_sync_needed = false;
    for message in messages {
        if let ui::UiMessage::Extern(game_state::GameMessage::NextWave(wave)) = message {
            gui.add_element(0, construct_wave_menu(ctx, gfx_ctx, (wave - 1) as u32));
            player_sync_needed = true;
            break;
        }
    }

    if let (Some(mut director), Some(mut data), Some(player_ent), Some(mut spell_pool)) = (
        resources.get_mut::<game_state::director::Director>(),
        resources.get_mut::<game_state::game_data::GameData>(),
        resources.get::<game_state::Entity>(),
        resources.get_mut::<game_state::components::spell::SpellPool>(),
    ) {
        if let Ok(mut player) = world.entry_mut(*player_ent) {
            if let Ok(caster) = player.get_component_mut::<game_state::components::SpellCaster>() {
                // enemies submenu
                if messages.contains(&ui::UiMessage::Triggered(ID_ENEMIES)) {
                    gui.remove_elements(ID_WAVE_SUBMENU);
                    gui.add_element(
                        ID_WAVE_SUBMENU_CONT,
                        construct_enemies_menu(ctx, gfx_ctx, &director, &mut data.buildings),
                    );
                }

                // spells submenu
                if messages.contains(&ui::UiMessage::Triggered(ID_SPELLS)) {
                    gui.remove_elements(ID_WAVE_SUBMENU);
                    gui.add_element(
                        ID_WAVE_SUBMENU_CONT,
                        construct_spell_menu(ctx, caster, &spell_pool, &data.buildings),
                    );
                }

                // build submenu
                if messages.contains(&ui::UiMessage::Triggered(ID_HOUSE)) {
                    gui.remove_elements(ID_WAVE_SUBMENU);
                    gui.add_element(
                        ID_WAVE_SUBMENU_CONT,
                        construct_buildings_menu(ctx, gfx_ctx, &mut data.buildings),
                    );
                }

                // unlock and equip spells
                for message in messages {
                    // remember if anything has changed
                    let mut triggered = false;
                    match message {
                        // check for clicks if a spell in the shop index-range
                        ui::UiMessage::Triggered(id)
                            if *id >= ID_SPELL_AVAIL_START
                                && *id < ID_SPELL_AVAIL_START + spell_pool.1.len() as u32 =>
                        {
                            let mut purchased = false;
                            // calculate index
                            let index = (id - ID_SPELL_AVAIL_START) as usize;
                            // check if a spell is at that index
                            if let Some(template) = spell_pool.1.get_mut(index) {
                                // if spell was not yet purchased, attempt to purchase it
                                if template.level == 0
                                    && data.buildings.target
                                        [buildings::BuildingType::Mageguild as usize]
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
                        ui::UiMessage::Triggered(id)
                            if *id >= ID_SPELL_EQUIP_START && *id < ID_SPELL_EQUIP_START + 4 =>
                        {
                            // calculate index
                            let index = (id - ID_SPELL_EQUIP_START) as usize;
                            // check if a spell is stored and remove it
                            if let Some(stored) = spell_pool.0.take() {
                                // equip the spell
                                caster.equip_spell(index, stored);
                                triggered = true;
                                player_sync_needed = true;
                            }
                        }
                        _ => {}
                    }
                    // reload menu if neccessary
                    if triggered {
                        gui.remove_elements(ID_WAVE_SUBMENU);
                        gui.add_element(
                            ID_WAVE_SUBMENU_CONT,
                            construct_spell_menu(ctx, caster, &spell_pool, &data.buildings),
                        );
                    }
                }

                // reroll
                if messages.contains(&ui::UiMessage::Triggered(ID_REROLL))
                    && data.spend(director.get_reroll_cost())
                    && data.buildings.target[buildings::BuildingType::Watchtower as usize] > 0
                {
                    director.reroll_wave_enemies();
                    gui.remove_elements(ID_WAVE_SUBMENU);
                    gui.add_element(
                        ID_WAVE_SUBMENU_CONT,
                        construct_enemies_menu(ctx, gfx_ctx, &director, &mut data.buildings),
                    );
                }

                // buildings
                for i in 0..buildings::BUILDING_TYPES {
                    let index = data.buildings.target[i] as usize;
                    if messages.contains(&ui::UiMessage::Triggered(ID_BUILDINGS_START + i as u32))
                        && index < buildings::BUILDING_MAX_LEVEL
                        && data.spend(
                            super::game_state::components::buildings::get_building_info(i)
                                .level_costs[index] as i32,
                        )
                    {
                        data.buildings.target[i] += 1;
                        // if it is the mana well, sync spell slots
                        // if it is the watchtower, sync speed
                        if i == 2 && caster.can_add() || i == 0 {
                            player_sync_needed = true;
                        }
                        // rebuild menu
                        gui.remove_elements(ID_WAVE_SUBMENU);
                        gui.add_element(
                            ID_WAVE_SUBMENU_CONT,
                            construct_buildings_menu(ctx, gfx_ctx, &mut data.buildings),
                        );
                    }
                }

                // close wave menu and activate next wave
                if messages.contains(&ui::UiMessage::Triggered(ID_NEXT_WAVE)) {
                    gui.remove_elements(ID_WAVE_MENU);
                    // initialize next wave from director
                    director.next_wave();
                    // create wave announcer
                    gui.add_element(0, construct_wave_announcer(ctx, director.get_wave()));
                }
            }
        }
    }

    if player_sync_needed {
        sync_ui(ctx, gfx_ctx, gui, world, resources);
    }
}

/// Construct the last row all three submenus share
fn construct_wave_menu(
    ctx: &mut good_web_game::Context,
    gfx_ctx: &mut good_web_game::event::GraphicsContext,
    wave_survived: u32,
) -> ui::UiElement<game_state::GameMessage> {
    // play happy sound
    let wave_done = good_web_game::audio::Source::new(ctx, "./audio/sounds/ui/wave_done.wav")
        .expect("Could not load wave end sound.");
    good_web_game::audio::Source::play(&wave_done, ctx)
        .expect("[ERROR/Radish] Could not find wave_done.wav.");

    let enemies = graphics::Image::new(ctx, gfx_ctx, "./sprites/ui/enemies.png")
        .expect("[ERROR/Radish] Missing enemies menu sprite.")
        .to_element_builder(ID_ENEMIES, ctx)
        .as_shrink()
        .scaled(4., 4.)
        .with_trigger_key(good_web_game::input::keyboard::KeyCode::U)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .with_tooltip(
            graphics::Text::new(
                graphics::TextFragment::new("Look at approaching enemies.\n[U]")
                    .color(graphics::Color::from_rgb_u32(PALETTE[6]))
                    .scale(24.)
                    .font(crate::RETRO.with(|f| f.borrow().unwrap())),
            )
            .to_owned()
            .to_element_builder(0, ctx)
            .with_visuals(super::BUTTON_VIS)
            .build(),
        )
        .with_message_handler(|messages, _, transitions| {
            if messages.contains(&ui::UiMessage::Triggered(ID_ENEMIES)) {
                transitions.push_back(
                    ui::Transition::new(Duration::ZERO)
                        .with_new_visuals(ui::Visuals {
                            border_widths: [3., 3., 0., 3.],
                            corner_radii: [3., 0., 0., 3.],
                            ..super::BUTTON_VIS
                        })
                        .with_new_hover_visuals(None),
                )
            }
            if messages.contains(&ui::UiMessage::Triggered(ID_SPELLS))
                || messages.contains(&ui::UiMessage::Triggered(ID_HOUSE))
            {
                transitions.push_back(
                    ui::Transition::new(Duration::ZERO)
                        .with_new_visuals(super::BUTTON_VIS)
                        .with_new_hover_visuals(Some(super::BUTTON_HOVER_VIS)),
                )
            }
        })
        .build();

    let spellbook = graphics::Image::new(ctx, gfx_ctx, "./sprites/ui/book.png")
        .expect("[ERROR/Radish] Missing spellbook sprite.")
        .to_element_builder(ID_SPELLS, ctx)
        .as_shrink()
        .scaled(4., 4.)
        .with_trigger_key(good_web_game::input::keyboard::KeyCode::I)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .with_tooltip(
            graphics::Text::new(
                graphics::TextFragment::new("Look at your spellbook.\n[I]")
                    .color(graphics::Color::from_rgb_u32(PALETTE[6]))
                    .font(crate::RETRO.with(|f| f.borrow().unwrap()))
                    .scale(24.),
            )
            .to_owned()
            .to_element_builder(0, ctx)
            .with_visuals(super::BUTTON_VIS)
            .build(),
        )
        .with_message_handler(|messages, _, transitions| {
            if messages.contains(&ui::UiMessage::Triggered(ID_SPELLS)) {
                transitions.push_back(
                    ui::Transition::new(Duration::ZERO)
                        .with_new_visuals(ui::Visuals {
                            border_widths: [3., 3., 0., 3.],
                            corner_radii: [3., 0., 0., 3.],
                            ..super::BUTTON_VIS
                        })
                        .with_new_hover_visuals(None),
                )
            }
            if messages.contains(&ui::UiMessage::Triggered(ID_ENEMIES))
                || messages.contains(&ui::UiMessage::Triggered(ID_HOUSE))
            {
                transitions.push_back(
                    ui::Transition::new(Duration::ZERO)
                        .with_new_visuals(super::BUTTON_VIS)
                        .with_new_hover_visuals(Some(super::BUTTON_HOVER_VIS)),
                )
            }
        })
        .build();

    let house = graphics::Image::new(ctx, gfx_ctx, "./sprites/ui/house_add.png")
        .expect("[ERROR/Radish] Missing house sprite.")
        .to_element_builder(ID_HOUSE, ctx)
        .as_shrink()
        .scaled(4., 4.)
        .with_trigger_key(good_web_game::input::keyboard::KeyCode::O)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .with_tooltip(
            graphics::Text::new(
                graphics::TextFragment::new("Construct town buildings.\n[O]")
                    .color(graphics::Color::from_rgb_u32(PALETTE[6]))
                    .scale(24.)
                    .font(crate::RETRO.with(|f| f.borrow().unwrap())),
            )
            .to_owned()
            .to_element_builder(0, ctx)
            .with_visuals(super::BUTTON_VIS)
            .build(),
        )
        .with_message_handler(|messages, _, transitions| {
            if messages.contains(&ui::UiMessage::Triggered(ID_HOUSE)) {
                transitions.push_back(
                    ui::Transition::new(Duration::ZERO)
                        .with_new_visuals(ui::Visuals {
                            border_widths: [3., 3., 0., 3.],
                            corner_radii: [3., 0., 0., 3.],
                            ..super::BUTTON_VIS
                        })
                        .with_new_hover_visuals(None),
                )
            }
            if messages.contains(&ui::UiMessage::Triggered(ID_ENEMIES))
                || messages.contains(&ui::UiMessage::Triggered(ID_SPELLS))
            {
                transitions.push_back(
                    ui::Transition::new(Duration::ZERO)
                        .with_new_visuals(super::BUTTON_VIS)
                        .with_new_hover_visuals(Some(super::BUTTON_HOVER_VIS)),
                )
            }
        })
        .build();

    let next = graphics::Image::new(ctx, gfx_ctx, "./sprites/ui/next.png")
        .expect("[ERROR/Radish] Missing next sprite.")
        .to_element_builder(ID_NEXT_WAVE, ctx)
        .as_shrink()
        .scaled(4., 4.)
        .with_trigger_key(good_web_game::input::keyboard::KeyCode::P)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .with_tooltip(
            graphics::Text::new(
                graphics::TextFragment::new("Start the next wave!\n[P]")
                    .color(graphics::Color::from_rgb_u32(PALETTE[6]))
                    .scale(24.)
                    .font(crate::RETRO.with(|f| f.borrow().unwrap())),
            )
            .to_owned()
            .to_element_builder(0, ctx)
            .with_visuals(super::BUTTON_VIS)
            .build(),
        )
        .build();

    let nav_row = ui::containers::HorizontalBox::new_spaced(16.)
        .to_element_builder(0, ctx)
        .with_child(enemies)
        .with_child(spellbook)
        .with_child(house)
        .with_child(next)
        .with_alignment(ui::Alignment::Center, ui::Alignment::Min)
        .with_visuals(ui::Visuals {
            corner_radii: [3., 0., 0., 3.],
            border_widths: [0., 0., 3., 0.],
            ..super::BUTTON_VIS
        })
        .with_padding((8., 8., 0., 8.))
        .with_size(
            ui::Size::Fill(0., f32::INFINITY),
            ui::Size::Shrink(0., f32::INFINITY),
        )
        .build();

    let title = graphics::Text::new(
        graphics::TextFragment::new("Brief Respite")
            .color(graphics::Color::from_rgb_u32(PALETTE[8]))
            .scale(24.)
            .font(crate::RETRO.with(|f| f.borrow().unwrap())),
    )
    .to_owned()
    .to_element_builder(0, ctx)
    .build();

    let wave_info = graphics::Text::new(
        graphics::TextFragment::new(format!("You have survived wave {}.", wave_survived))
            .color(graphics::Color::from_rgb_u32(PALETTE[6]))
            .scale(32.)
            .font(crate::RETRO.with(|f| f.borrow().unwrap())),
    )
    .to_owned()
    .to_element_builder(0, ctx)
    .build();

    let submenu_cont = ui::containers::StackBox::new()
        .to_element_builder(ID_WAVE_SUBMENU_CONT, ctx)
        .with_child(
            ui::containers::VerticalBox::new()
                .to_element_builder(ID_WAVE_SUBMENU, ctx)
                .with_child(title)
                .with_child(wave_info)
                .build(),
        )
        .as_fill()
        .with_padding((25., 25., 25., 25.))
        .build();

    ui::containers::VerticalBox::new_spaced(8.)
        .to_element_builder(ID_WAVE_MENU, ctx)
        .with_child(nav_row)
        .with_child(submenu_cont)
        .with_padding((3., 3., 3., 3.))
        .with_visuals(super::BUTTON_VIS)
        .with_size(ui::Size::Shrink(580., f32::INFINITY), None)
        .with_alignment(ui::Alignment::Center, ui::Alignment::Min)
        .with_offset(None, 128.)
        .build()
}

fn construct_enemies_menu(
    ctx: &mut good_web_game::Context,
    gfx_ctx: &mut good_web_game::event::GraphicsContext,
    director: &super::game_state::director::Director,
    buildings: &mut game_state::components::buildings::Buildings,
) -> ui::UiElement<game_state::GameMessage> {
    // ---- Enemy display and reroll ----

    let title = graphics::Text::new(
        graphics::TextFragment::new("Approaching Enemies")
            .color(graphics::Color::from_rgb_u32(PALETTE[8]))
            .scale(40.)
            .font(crate::RETRO.with(|f| f.borrow().unwrap())),
    )
    .to_owned()
    .to_element_builder(0, ctx)
    .build();

    let mut enemy_box = ui::containers::HorizontalBox::new();

    for template in director.get_enemies() {
        enemy_box.add(
            template
                .icon
                .clone()
                .to_element_builder(0, ctx)
                .scaled(4., 4.)
                .with_tooltip(
                    graphics::Text::new(
                        graphics::TextFragment::new(template.name.as_str())
                            .color(graphics::Color::from_rgb_u32(PALETTE[8]))
                            .font(crate::RETRO.with(|f| f.borrow().unwrap()))
                            .scale(36.),
                    )
                    .add(
                        graphics::TextFragment::new("\n")
                            .font(crate::RETRO.with(|f| f.borrow().unwrap())),
                    )
                    .add(
                        graphics::TextFragment::new(template.description.as_str())
                            .color(graphics::Color::from_rgb_u32(PALETTE[6]))
                            .font(crate::RETRO.with(|f| f.borrow().unwrap()))
                            .scale(24.),
                    )
                    .set_bounds(
                        good_web_game::graphics::Point2::new(300., 200.),
                        graphics::Align::Left,
                    )
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
        .with_visuals(ui::Visuals {
            background: graphics::Color::from_rgb_u32(PALETTE[10]),
            ..super::BUTTON_VIS
        })
        .build();

    let reroll = if buildings.target[buildings::BuildingType::Watchtower as usize] == 0 {
        ().to_element(0, ctx)
    } else {
        graphics::Image::new(ctx, gfx_ctx, "./sprites/ui/reroll.png")
            .expect("[ERROR/Radish] Missing reroll sprite.")
            .to_element_builder(ID_REROLL, ctx)
            .as_shrink()
            .scaled(4., 4.)
            .with_padding((10., 10., 10., 10.))
            .with_trigger_key(good_web_game::input::keyboard::KeyCode::M)
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
                    .color(graphics::Color::from_rgb_u32(PALETTE[6]))
                    .font(crate::RETRO.with(|f| f.borrow().unwrap()))
                    .scale(24.),
                )
                .to_owned()
                .to_element_builder(0, ctx)
                .with_visuals(super::BUTTON_VIS)
                .build(),
            )
            .build()
    };

    let enemy_row = ui::containers::HorizontalBox::new()
        .to_element_builder(0, ctx)
        .with_child(enemy_box)
        .with_child(reroll)
        .with_alignment(ui::Alignment::Center, None)
        .build();

    // Container
    ui::containers::VerticalBox::new_spaced(16.)
        .to_element_builder(ID_WAVE_SUBMENU, ctx)
        .with_child(title)
        .with_child(enemy_row)
        .with_alignment(ui::Alignment::Center, ui::Alignment::Center)
        .as_fill()
        .build()
}

fn construct_spell_menu(
    ctx: &good_web_game::Context,
    caster: &mut game_state::components::SpellCaster,
    spell_pool: &game_state::components::spell::SpellPool,
    buildings: &game_state::components::buildings::Buildings,
) -> ui::UiElement<game_state::GameMessage> {
    // ---- Title ----

    let title = graphics::Text::new(
        graphics::TextFragment::new("Book of Spells")
            .color(graphics::Color::from_rgb_u32(PALETTE[8]))
            .font(crate::RETRO.with(|f| f.borrow().unwrap()))
            .scale(40.),
    )
    .to_owned()
    .to_element_builder(0, ctx)
    .build();

    // available spells

    let available_title = graphics::Text::new(
        graphics::TextFragment::new("Available:")
            .color(graphics::Color::from_rgb_u32(PALETTE[6]))
            .scale(24.)
            .font(crate::RETRO.with(|f| f.borrow().unwrap())),
    )
    .to_owned()
    .to_element_builder(0, ctx)
    .build();

    let available = spell_pool
        .1
        .iter()
        .enumerate()
        .fold(
            ui::containers::GridBox::new_spaced(6, 4, 8., 8.),
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
        graphics::TextFragment::new("Loadout:")
            .color(graphics::Color::from_rgb_u32(PALETTE[6]))
            .font(crate::RETRO.with(|f| f.borrow().unwrap()))
            .scale(24.),
    )
    .to_owned()
    .to_element_builder(0, ctx)
    .build();

    // equipped spells

    let loadout = caster
        .get_spells()
        .iter()
        .enumerate()
        .fold(
            ui::containers::HorizontalBox::new_spaced(16.).to_element_builder(0, ctx),
            |loadout, (index, spell)| {
                loadout
                    .with_child(spell.info_element_small(ID_SPELL_EQUIP_START + index as u32, ctx))
            },
        )
        .with_alignment(ui::Alignment::Center, None)
        .as_shrink()
        .build();

    ui::containers::VerticalBox::new_spaced(16.)
        .to_element_builder(ID_WAVE_SUBMENU, ctx)
        .with_child(title)
        .with_child(available_title)
        .with_child(available)
        .with_child(loadout_title)
        .with_child(loadout)
        .with_alignment(ui::Alignment::Center, ui::Alignment::Center)
        .as_fill()
        .build()
}

fn construct_buildings_menu(
    ctx: &mut good_web_game::Context,
    gfx_ctx: &mut good_web_game::event::GraphicsContext,
    buildings: &mut game_state::components::buildings::Buildings,
) -> ui::UiElement<game_state::GameMessage> {
    // ---- Enemy display and reroll ----

    let title = graphics::Text::new(
        graphics::TextFragment::new("Construct Buildings")
            .color(graphics::Color::from_rgb_u32(PALETTE[8]))
            .scale(40.)
            .font(crate::RETRO.with(|f| f.borrow().unwrap())),
    )
    .to_owned()
    .to_element_builder(0, ctx)
    .build();

    let mut construct_box =
        ui::containers::HorizontalBox::new_spaced(25.).to_element_builder(0, ctx);

    let icons = [
        "/sprites/ui/looking_glass.png",
        "/sprites/ui/potion.png",
        "/sprites/ui/mana_add.png",
    ];
    let keycodes = [
        ("B", good_web_game::input::keyboard::KeyCode::B),
        ("N", good_web_game::input::keyboard::KeyCode::N),
        ("M", good_web_game::input::keyboard::KeyCode::M),
    ];

    for i in 0..buildings::BUILDING_TYPES {
        let info = buildings::get_building_info(i);
        let build = graphics::Image::new(ctx, gfx_ctx, icons[i])
            .expect("[ERROR/Radish] Missing building sprite.")
            .to_element_builder(ID_BUILDINGS_START + i as u32, ctx)
            .as_shrink()
            .scaled(4., 4.)
            .with_padding((10., 10., 10., 10.))
            .with_visuals(super::BUTTON_VIS)
            .with_hover_visuals(super::BUTTON_HOVER_VIS)
            .with_trigger_key(keycodes[i].1)
            .with_tooltip({
                let mut text = graphics::Text::new(
                    graphics::TextFragment::new(
                        if buildings.target[i] < buildings::BUILDING_MAX_LEVEL as u8 {
                            format!(
                                "{} the {}.\n",
                                if buildings.target[i] == 0 {
                                    "Construct"
                                } else {
                                    "Upgrade"
                                },
                                info.name,
                            )
                        } else {
                            format!("{} is fully upgraded.\n", info.name,)
                        },
                    )
                    .font(crate::RETRO.with(|f| f.borrow().unwrap()))
                    .color(graphics::Color::from_rgb_u32(PALETTE[6]))
                    .scale(24.),
                );

                text.add(
                    TextFragment::new(info.description)
                        .font(crate::RETRO.with(|f| f.borrow().unwrap()))
                        .scale(20.)
                        .color(graphics::Color::from_rgb_u32(PALETTE[6])),
                );

                text.add(
                    TextFragment::new("\nCurrent level: ")
                        .scale(20.)
                        .font(crate::RETRO.with(|f| f.borrow().unwrap()))
                        .color(graphics::Color::from_rgb_u32(PALETTE[6])),
                );

                text.add(
                    TextFragment::new(format!("{}\n", buildings.target[i]))
                        .font(crate::RETRO.with(|f| f.borrow().unwrap()))
                        .scale(20.)
                        .color(graphics::Color::from_rgb_u32(PALETTE[7])),
                );

                if buildings.target[i] < buildings::BUILDING_MAX_LEVEL as u8 {
                    text.add(
                        TextFragment::new("Cost: ")
                            .scale(20.)
                            .color(graphics::Color::from_rgb_u32(PALETTE[6]))
                            .font(crate::RETRO.with(|f| f.borrow().unwrap())),
                    );

                    text.add(
                        TextFragment::new(format!(
                            "{}g\n",
                            info.level_costs[buildings.target[i] as usize]
                        ))
                        .scale(20.)
                        .color(graphics::Color::from_rgb_u32(PALETTE[7]))
                        .font(crate::RETRO.with(|f| f.borrow().unwrap())),
                    );

                    text.add(
                        TextFragment::new(format!("[{}]\n", keycodes[i].0))
                            .scale(20.)
                            .color(graphics::Color::from_rgb_u32(PALETTE[6]))
                            .font(crate::RETRO.with(|f| f.borrow().unwrap())),
                    );
                }

                text.to_element_builder(0, ctx)
                    .with_visuals(super::BUTTON_VIS)
                    .build()
            })
            .build();

        let level = graphics::Text::new(
            graphics::TextFragment::new(format!(" {} ", buildings.target[i]))
                .color(graphics::Color::from_rgb_u32(PALETTE[14]))
                .font(crate::RETRO.with(|f| f.borrow().unwrap()))
                .scale(24.),
        )
        .to_owned()
        .to_element_builder(0, ctx)
        .with_padding((2., 2., 2., 2.))
        .with_visuals(ui::Visuals {
            background: graphics::Color::from_rgb_u32(PALETTE[4]),
            border: graphics::Color::from_rgb_u32(PALETTE[4]),
            border_widths: [0.; 4],
            corner_radii: [14.; 4],
        })
        .as_shrink()
        .with_alignment(ui::Alignment::Max, ui::Alignment::Max)
        .build();

        construct_box = construct_box.with_child(
            ui::containers::StackBox::new()
                .to_element_builder(0, ctx)
                .with_child(level)
                .with_child(build)
                .build(),
        );
    }

    // Container
    ui::containers::VerticalBox::new_spaced(16.)
        .to_element_builder(ID_WAVE_SUBMENU, ctx)
        .with_child(title)
        .with_child(construct_box.build())
        .with_alignment(ui::Alignment::Center, ui::Alignment::Center)
        .as_fill()
        .build()
}

fn construct_wave_announcer(
    ctx: &good_web_game::Context,
    wave: u32,
) -> ui::UiElement<game_state::GameMessage> {
    let mut dur = ui::containers::DurationBox::new(
        Duration::from_secs(5),
        graphics::Text::new(
            graphics::TextFragment::new(format!("Wave {}", wave))
                .color(graphics::Color::from_rgb_u32(PALETTE[14]))
                .scale(48.)
                .font(crate::RETRO.with(|f| f.borrow().unwrap())),
        )
        .to_owned()
        .to_element_builder(0, ctx)
        .as_shrink()
        .build(),
    )
    .to_element_builder(0, ctx)
    .as_shrink()
    .with_alignment(ui::Alignment::Center, ui::Alignment::Center)
    .build();

    let ctr_layout = dur.get_layout();

    dur.add_transition(ui::Transition::new(Duration::from_secs(2)));

    dur.add_transition(
        ui::Transition::new(Duration::from_secs(3)).with_new_layout(ui::Layout {
            y_alignment: ui::Alignment::Min,
            ..ctr_layout
        }),
    );

    dur
}

pub fn sync_ui(
    ctx: &mut good_web_game::Context,
    gfx_ctx: &mut good_web_game::event::GraphicsContext,
    gui: &mut ui::UiElement<game_state::GameMessage>,
    world: &mut legion::World,
    resources: &mut legion::Resources,
) {
    if let (Some(data), Some(player_ent)) = (
        resources.get_mut::<game_state::game_data::GameData>(),
        resources.get::<game_state::Entity>(),
    ) {
        if let Ok(mut player) = world.entry_mut(*player_ent) {
            // sync spell

            if let Ok(caster) = player.get_component_mut::<game_state::components::SpellCaster>() {
                // game sync
                caster.set_extra_slots(
                    data.buildings.target[buildings::BuildingType::Manawell as usize] as usize,
                );
                // ui sync
                gui.remove_elements(super::game_ui::ID_MANA_SLOT);
                for i in 0..caster.get_slots() {
                    gui.add_element(
                        super::game_ui::ID_MANA_BAR,
                        super::game_ui::create_spellslot(ctx, gfx_ctx, i),
                    );
                }

                // ui_sync
                gui.remove_elements(super::game_ui::ID_SPELL_BAR_CHILDREN);
                gui.add_element(
                    super::game_ui::ID_SPELL_BAR,
                    caster
                        .get_spells()
                        .iter()
                        .fold(
                            ui::containers::HorizontalBox::new_spaced(16.)
                                .to_element_builder(super::game_ui::ID_SPELL_BAR_CHILDREN, ctx),
                            |loadout, spell| loadout.with_child(spell.info_element_small(0, ctx)),
                        )
                        .build(),
                );
            }

            // sync move speed

            if let Ok(control) = player.get_component_mut::<game_state::components::Control>() {
                // game sync
                control.move_speed = control.base_speed
                    * (1.
                        + 0.25
                            * (data.buildings.target[buildings::BuildingType::Watchtower as usize]
                                as f32
                                - 1.)
                                .max(0.));
            }
        }
    }
}
