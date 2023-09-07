pub mod achievement_menu;
pub mod credits_menu;
pub mod highscore_menu;
pub mod options_menu;

use std::time::Duration;

use super::game_state;
use super::BUTTON_HOVER_VIS;
use super::BUTTON_VIS;

use crate::music;
use ggez::glam::Vec2;
use ggez::{graphics, GameError};
use mooeye::{scene_manager, sprite, ui, ui::UiContent};

use crate::PALETTE;

const CAMERA_SPEED: f32 = -60.;
const REL_TROOP_SPEED: f32 = 14.;

/// The main menu greeting the player on startup.
/// Contains navigation buttons to multiple submenus and allows starting games.
pub struct MainMenu {
    /// The gui containing the buttons to the submenus
    gui: ui::UiElement<()>,
    /// The music player for background music. Stops when starting a game
    music_player: music::MusicPlayer,

    /// sprites
    background_sprites: Vec<MainMenuSprite>,
    /// The current state
    state: MainMenuTransition,
}

/// A background sprite in the main menu
struct MainMenuSprite {
    /// The sprite
    sprite: mooeye::sprite::Sprite,
    /// Its position on the screen
    pos: Vec2,
    /// The velocity its moving with
    vel: Vec2,
}

///Different states of the main menu
type MainMenuTransition = Option<(Duration, game_state::GameConfig)>;

impl MainMenu {
    pub fn new(ctx: &ggez::Context) -> Result<Self, GameError> {
        // title
        let title = graphics::Image::from_path(ctx, "/sprites/ui/logo1.png")?
            .to_element_builder(0, ctx)
            .scaled(4., 4.)
            .build();

        // play
        let play = graphics::Text::new(
            graphics::TextFragment::new("Play").color(graphics::Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_builder(1, ctx)
        .with_trigger_key(ggez::winit::event::VirtualKeyCode::P)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .with_trigger_sound(ggez::audio::Source::new(ctx, "/audio/sounds/ui/blipSelect.wav").ok())
        .build();

        // advanced start
        let quick_advance = graphics::Text::new(
            graphics::TextFragment::new("Quick Advance")
                .color(graphics::Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_builder(2, ctx)
        .with_trigger_key(ggez::winit::event::VirtualKeyCode::U)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .with_trigger_sound(ggez::audio::Source::new(ctx, "/audio/sounds/ui/blipSelect.wav").ok())
        .with_tooltip(
            graphics::Text::new(
                graphics::TextFragment::new(
                    "Immediately start at a higher level, with some starting gold.",
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

        let debug = graphics::Text::new(
            graphics::TextFragment::new("Debug").color(graphics::Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_builder(3, ctx)
        .with_trigger_key(ggez::winit::event::VirtualKeyCode::D)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .with_trigger_sound(ggez::audio::Source::new(ctx, "/audio/sounds/ui/blipSelect.wav").ok())
        .build();

        // highscores

        let highscores = graphics::Text::new(
            graphics::TextFragment::new("Highscores")
                .color(graphics::Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_builder(4, ctx)
        .with_trigger_key(ggez::winit::event::VirtualKeyCode::H)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .with_trigger_sound(ggez::audio::Source::new(ctx, "/audio/sounds/ui/blipSelect.wav").ok())
        .build();

        // achievement

        let achievements = graphics::Text::new(
            graphics::TextFragment::new("Achievements")
                .color(graphics::Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_builder(5, ctx)
        .with_trigger_key(ggez::winit::event::VirtualKeyCode::A)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .with_trigger_sound(ggez::audio::Source::new(ctx, "/audio/sounds/ui/blipSelect.wav").ok())
        .build();

        let options = graphics::Text::new(
            graphics::TextFragment::new("Options").color(graphics::Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_builder(6, ctx)
        .with_trigger_key(ggez::winit::event::VirtualKeyCode::O)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .with_trigger_sound(ggez::audio::Source::new(ctx, "/audio/sounds/ui/blipSelect.wav").ok())
        .build();

        let credits = graphics::Text::new(
            graphics::TextFragment::new("Credits").color(graphics::Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_builder(7, ctx)
        .with_trigger_key(ggez::winit::event::VirtualKeyCode::C)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .with_trigger_sound(
            ggez::audio::Source::new(ctx, "/audio/sounds/ui/blipSelect.wav").unwrap(),
        )
        .build();

        let quit = graphics::Text::new(
            graphics::TextFragment::new("Quit").color(graphics::Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_builder(8, ctx)
        .with_trigger_key(ggez::winit::event::VirtualKeyCode::Q)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .with_trigger_sound(ggez::audio::Source::new(ctx, "/audio/sounds/ui/blipSelect.wav").ok())
        .build();

        // Container
        let menu_box = ui::containers::VerticalBox::new_spaced(25.)
            .to_element_builder(0, ctx)
            .with_child(play);

        let menu_box = if game_state::achievements::HIGHSCORES
            .with(|scores| scores.borrow().scores.is_empty())
        {
            menu_box
        } else {
            menu_box.with_child(quick_advance)
        };

        let menu_box = if cfg!(debug_assertions) {
            menu_box.with_child(debug)
        } else {
            menu_box
        }
        .with_child(highscores)
        .with_child(achievements)
        .with_child(options)
        .with_child(credits)
        .with_child(quit)
        .with_visuals(super::BUTTON_VIS)
        .with_alignment(ui::Alignment::Center, ui::Alignment::Min)
        .with_padding((25., 25., 25., 25.))
        .build();

        let big_box = ui::containers::VerticalBox::new()
            .to_element_builder(0, ctx)
            .with_child(title)
            .with_child(menu_box)
            .with_alignment(ui::Alignment::Min, ui::Alignment::Min)
            .with_padding((25., 25., 25., 25.))
            .build();

        let mut music_player = music::MusicPlayer::from_folder(ctx, "/audio/music/main_menu");
        music_player.poll_options();
        music_player.next_song(ctx);
        let sprite_pool = sprite::SpritePool::new().with_folder(ctx, "/sprites", true);

        // -----------------------------------
        // Create backgorund sprites
        // -----------------------------------

        let mut background_sprites = Vec::new();
        let b_boundaries = graphics::Rect {
            h: game_state::BOUNDARIES.h + 512.,
            ..game_state::BOUNDARIES
        };

        // step 1: cobbles
        for _i in 0..48 {
            background_sprites.push(MainMenuSprite {
                sprite: {
                    let mut cobble =
                        sprite_pool.init_sprite("/sprites/environment/cobble", Duration::ZERO)?;
                    cobble.set_variant(rand::random::<u32>());
                    cobble
                },
                pos: Vec2::new(
                    b_boundaries.w * rand::random::<f32>(),
                    b_boundaries.h * rand::random::<f32>(),
                ),
                vel: Vec2::new(0., CAMERA_SPEED),
            });
        }

        // step 2: brush
        for _i in 0..36 {
            let rand_x = (rand::random::<f32>() * 2. - 1.) * 32. * 4. * 4.;
            background_sprites.push(MainMenuSprite {
                sprite: {
                    let mut tree =
                        sprite_pool.init_sprite("/sprites/environment/tree", Duration::ZERO)?;
                    tree.set_variant(rand::random::<u32>());
                    tree
                },
                pos: Vec2::new(
                    (rand_x) + if rand_x > 0. { b_boundaries.w } else { 0. },
                    rand::random::<f32>() * b_boundaries.h,
                ),
                vel: Vec2::new(0., CAMERA_SPEED),
            });
        }

        // step 4: sorting
        background_sprites.sort_by(|p1, p2| {
            p1.pos
                .y
                .partial_cmp(&p2.pos.y)
                .expect("[ERROR/Radish] Ordering of y-coordinates in main menu sprites.")
        });

        // step 3: troops
        let troop_paths = vec![
            "armor",
            "legionnaire",
            "skeleton_basic",
            "skeleton_tank",
            "skeleton_wizard",
            "skeleton_sword",
            "skeleton_jump",
        ];

        for i in 0..12 {
            let count = rand::random::<u32>() % 4 + 3;
            let sprite_path = format!(
                "/sprites/enemies/{}",
                troop_paths[rand::random::<usize>() % troop_paths.len()]
            );
            for j in 0..count {
                background_sprites.push(MainMenuSprite {
                    sprite: sprite_pool.init_sprite(
                        &sprite_path,
                        Duration::from_secs_f32(rand::random::<f32>() * 0.2 + 0.3),
                    )?,
                    pos: Vec2::new(
                        b_boundaries.w / 2. - count as f32 * 32. + 64. * j as f32,
                        b_boundaries.h / 12. * i as f32,
                    ),
                    vel: Vec2::new(0., CAMERA_SPEED + REL_TROOP_SPEED),
                })
            }
        }

        Ok(Self {
            gui: big_box,
            music_player,
            background_sprites,
            state: None,
        })
    }
}

impl scene_manager::Scene for MainMenu {
    fn update(
        &mut self,
        ctx: &mut ggez::Context,
    ) -> Result<mooeye::scene_manager::SceneSwitch, ggez::GameError> {
        let messages = self.gui.manage_messages(ctx, None);

        let mut res = scene_manager::SceneSwitch::None;

        match self.state.take() {
            None => {
                if messages.contains(&ui::UiMessage::Triggered(1)) {
                    for sprite in &mut self.background_sprites {
                        sprite.vel.y -= 128.;
                    }
                    self.state = Some((Duration::from_secs(4), game_state::GameConfig::default()));
                }

                if messages.contains(&ui::UiMessage::Triggered(2)) {
                    for sprite in &mut self.background_sprites {
                        sprite.vel.y -= 128.;
                    }
                    self.state = Some((
                        Duration::from_secs(4),
                        game_state::GameConfig {
                            starting_gold: game_state::achievements::HIGHSCORES.with(|scores| {
                                scores
                                    .borrow()
                                    .scores
                                    .first()
                                    .map(|(score, _)| *score as i32)
                                    .unwrap_or_default()
                            }) / 3,
                            starting_wave: game_state::achievements::HIGHSCORES.with(|scores| {
                                scores
                                    .borrow()
                                    .scores
                                    .first()
                                    .map(|(_, wave)| *wave)
                                    .unwrap_or_default()
                            }) / 2,
                            ..Default::default()
                        },
                    ));
                }

                if messages.contains(&ui::UiMessage::Triggered(3)) {
                    self.state = Some((Duration::ZERO, game_state::GameConfig::debug()));
                }

                if messages.contains(&ui::UiMessage::Triggered(4)) {
                    res =
                        scene_manager::SceneSwitch::push(highscore_menu::HighscoreMenu::new(ctx)?);
                }

                if messages.contains(&ui::UiMessage::Triggered(5)) {
                    res = scene_manager::SceneSwitch::push(achievement_menu::AchievementMenu::new(
                        ctx,
                    )?);
                }

                if messages.contains(&ui::UiMessage::Triggered(6)) {
                    res = scene_manager::SceneSwitch::push(options_menu::OptionsMenu::new(ctx)?);
                }

                if messages.contains(&ui::UiMessage::Triggered(7)) {
                    res = scene_manager::SceneSwitch::push(credits_menu::CreditsMenu::new(ctx)?);
                }

                if messages.contains(&ui::UiMessage::Triggered(8)) {
                    self.music_player.stop(ctx);
                    res = mooeye::scene_manager::SceneSwitch::Pop(1);
                }
            }
            Some((dur, config)) => {
                // continuously speed up camera
                for sprite in &mut self.background_sprites {
                    sprite.vel.y -= ctx.time.delta().as_secs_f32() * 96.;
                }
                // if in a transitioning state: reduce duration and, if its has lapsed, switch the scene
                if dur.is_zero() {
                    self.music_player.stop(ctx);
                    res = mooeye::scene_manager::SceneSwitch::replace(
                        game_state::GameState::new(ctx, config)?,
                        1,
                    );
                } else {
                    self.state = Some((dur.saturating_sub(ctx.time.delta()), config));
                }
            }
        }

        Ok(res)
    }

    fn draw(&mut self, ctx: &mut ggez::Context, mouse_listen: bool) -> Result<(), ggez::GameError> {
        // music
        self.music_player.check_song(ctx);
        self.music_player.poll_options();

        // graphics
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from_rgb_u32(PALETTE[11]));
        canvas.set_sampler(graphics::Sampler::nearest_clamp());
        let (screen_w, screen_h) = ctx.gfx.drawable_size();

        // move sprites
        for sprite in self.background_sprites.iter_mut() {
            sprite.pos += sprite.vel * ctx.time.delta().as_secs_f32();
            //if self.state.is_none() {
            // reset sprites that have left the screen
            if sprite.pos.y < game_state::BOUNDARIES.y - 256. {
                sprite.pos.y = game_state::BOUNDARIES.y + game_state::BOUNDARIES.h + 256.;
            }
            //}
        }

        // draw environment & background sprites
        game_state::GameState::draw_background(
            &ggez::graphics::Rect::new(0., 0., 600., 900.),
            ctx,
            &mut canvas,
        );

        for b_sprite in self.background_sprites.iter_mut() {
            b_sprite.sprite.draw_sprite(
                ctx,
                &mut canvas,
                ggez::graphics::DrawParam::new()
                    .dest(Vec2::new(
                        (b_sprite.pos.x + (screen_w - game_state::BOUNDARIES.w) / 2.).floor(),
                        (b_sprite.pos.y + (screen_h - game_state::BOUNDARIES.h) / 2.).floor(),
                    ))
                    .scale(Vec2::new(4., 4.)),
            );
        }

        self.gui.draw_to_screen(ctx, &mut canvas, mouse_listen);

        // draw occlusion
        if let Some((dur, _)) = self.state {
            let ratio = 1. - dur.as_secs_f32() / 4.;
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .color(graphics::Color::new(0., 0., 0., ratio))
                    .scale(Vec2::new(screen_w, screen_h)),
            );
            canvas.draw(
                &graphics::Text::new(
                    graphics::TextFragment::new("Loading...")
                        .color(graphics::Color::from_rgb_u32(PALETTE[8]))
                        .font("Retro")
                        .scale(28.),
                ),
                graphics::DrawParam::new().dest(Vec2::new(16., screen_h - 16. - 28.)),
            );
        }

        canvas.finish(ctx)?;

        Ok(())
    }
}

impl Drop for MainMenu {
    fn drop(&mut self) {
        // save options to file on game exit (when the main menu is dropped)
        crate::options::OPTIONS.with(|opt| {
            if opt.borrow().save_to_file("./data/options.toml").is_err() {
                println!("[ERROR/Radish] Could not save options.")
            };
        });

        crate::scenes::game_state::achievements::ACHIEVEMENTS.with(|ach| {
            if std::fs::write(
                "./data/achievements.toml",
                toml::to_string(ach).unwrap_or_default(),
            )
            .is_err()
            {
                println!("[ERROR/Radish] Could not save achievements.");
            };
        });

        // Save highscores
        crate::scenes::game_state::achievements::HIGHSCORES.with(|scores| {
            if std::fs::write(
                "./data/highscores.toml",
                toml::to_string(scores).unwrap_or_default(),
            )
            .is_err()
            {
                println!("[ERROR/Radish] Could not save highscores.")
            }
        });
    }
}
