use crate::options;
use ggez::{graphics, GameError};
use mooeye::{scene_manager, ui, ui::UiContent};

use crate::PALETTE;

const VOLUME_IDS: u32 = 11;
const VOLUME_MUSIC_IDS: u32 = 21;
const VOLUME_CONTAINER_ID: u32 = 10;
const VOLUME_MUSIC_CONTAINER_ID: u32 = 20;

pub struct OptionsMenu {
    gui: ui::UiElement<()>,
    controller: super::game_state::Controller,
    options: options::OptionsConfig,
}

impl OptionsMenu {
    pub fn new(ctx: &ggez::Context) -> Result<Self, GameError> {
        // title

        let title = graphics::Text::new(
            graphics::TextFragment::new("Options").color(graphics::Color::from_rgb_u32(PALETTE[8])),
        )
        .set_font("Retro")
        .set_scale(48.)
        .to_owned()
        .to_element(0, ctx);

        let reset_bindings = graphics::Text::new(
            graphics::TextFragment::new("Reset Keybindings")
                .color(graphics::Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(28.)
        .to_owned()
        .to_element_builder(1, ctx)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .with_trigger_sound(ggez::audio::Source::new(ctx, "/audio/sounds/ui/blipSelect.wav").ok())
        .with_trigger_key(ggez::winit::event::VirtualKeyCode::R)
        .build();

        let sound = graphics::Text::new(
            graphics::TextFragment::new("Sound Volume")
                .color(graphics::Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(28.)
        .to_owned()
        .to_element_builder(0, ctx)
        .build();

        let music = graphics::Text::new(
            graphics::TextFragment::new("Music Volume")
                .color(graphics::Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(28.)
        .to_owned()
        .to_element_builder(0, ctx)
        .build();

        let back = graphics::Text::new(
            graphics::TextFragment::new("Close").color(graphics::Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_builder(3, ctx)
        .with_trigger_key(ggez::winit::event::VirtualKeyCode::C)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .with_trigger_sound(ggez::audio::Source::new(ctx, "/audio/sounds/ui/blipSelect.wav").ok())
        .build();

        let options = options::OPTIONS.with(|opt| *opt.borrow());

        let tutorial = graphics::Text::new(
            graphics::TextFragment::new("Re-enable Tutorial Hints")
                .color(graphics::Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(28.)
        .to_owned()
        .to_element_builder(2, ctx)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .with_trigger_sound(ggez::audio::Source::new(ctx, "/audio/sounds/ui/blipSelect.wav").ok())
        .with_trigger_key(ggez::winit::event::VirtualKeyCode::R)
        .build();

        // Container

        let options_box = ui::containers::VerticalBox::new()
            .to_element_builder(0, ctx)
            .with_child(title)
            .with_child(sound)
            .with_child(
                ui::containers::StackBox::new()
                    .to_element_builder(VOLUME_CONTAINER_ID, ctx)
                    .with_child(create_sound_adjuster(ctx, VOLUME_IDS, options.volume))
                    .with_wrapper_layout(ui::Layout::default())
                    .build(),
            )
            .with_child(
                ().to_element_builder(0, ctx)
                    .with_size(None, ui::Size::Fixed(20.))
                    .build(),
            )
            .with_child(music)
            .with_child(
                ui::containers::StackBox::new()
                    .to_element_builder(VOLUME_MUSIC_CONTAINER_ID, ctx)
                    .with_child(create_sound_adjuster(
                        ctx,
                        VOLUME_MUSIC_IDS,
                        options.music_volume,
                    ))
                    .with_wrapper_layout(ui::Layout::default())
                    .build(),
            )
            .with_child(
                ().to_element_builder(0, ctx)
                    .with_size(None, ui::Size::Fixed(20.))
                    .build(),
            )
            .with_child(reset_bindings)
            .with_child(
                ().to_element_builder(0, ctx)
                    .with_size(None, ui::Size::Fixed(20.))
                    .build(),
            )
            .with_child(tutorial)
            .with_child(
                ().to_element_builder(0, ctx)
                    .with_size(None, ui::Size::Fixed(20.))
                    .build(),
            )
            .with_child(back)
            .with_visuals(super::BUTTON_VIS)
            .with_alignment(ui::Alignment::Max, ui::Alignment::Center)
            .with_offset(-25., 0.)
            .with_padding((25., 25., 25., 25.))
            .build();

        Ok(Self {
            gui: options_box,
            controller: super::game_state::Controller::from_path("./data/keymap.toml")
                .unwrap_or_default(),
            options,
        })
    }
}

impl scene_manager::Scene for OptionsMenu {
    fn update(
        &mut self,
        ctx: &mut ggez::Context,
    ) -> Result<mooeye::scene_manager::SceneSwitch, GameError> {
        let messages = self.gui.manage_messages(ctx, None);

        // Adjust sound volume.

        let mut rebuild_meter = false;

        if messages.contains(&ui::UiMessage::Triggered(VOLUME_IDS + 1)) {
            // adjust sound
            self.options.volume = self.options.volume.saturating_sub(10);
            // set marker
            rebuild_meter = true;
        }

        if messages.contains(&ui::UiMessage::Triggered(VOLUME_IDS + 2)) {
            self.options.volume = self.options.volume.saturating_sub(1);
            rebuild_meter = true;
        }

        if messages.contains(&ui::UiMessage::Triggered(VOLUME_IDS + 3)) {
            self.options.volume = self.options.volume.saturating_add(1);
            rebuild_meter = true;
        }

        if messages.contains(&ui::UiMessage::Triggered(VOLUME_IDS + 4)) {
            self.options.volume = self.options.volume.saturating_add(10);
            rebuild_meter = true;
        }

        if rebuild_meter {
            // remove old element
            self.gui.remove_elements(VOLUME_IDS);
            // add element with new value
            self.gui.add_element(
                VOLUME_CONTAINER_ID,
                create_sound_adjuster(ctx, VOLUME_IDS, self.options.volume),
            );
            // reset marker
            rebuild_meter = false;
        }

        // Adjust music volume

        if messages.contains(&ui::UiMessage::Triggered(VOLUME_MUSIC_IDS + 1)) {
            self.options.music_volume = self.options.music_volume.saturating_sub(10);
            rebuild_meter = true;
        }

        if messages.contains(&ui::UiMessage::Triggered(VOLUME_MUSIC_IDS + 2)) {
            self.options.music_volume = self.options.music_volume.saturating_sub(1);
            rebuild_meter = true;
        }

        if messages.contains(&ui::UiMessage::Triggered(VOLUME_MUSIC_IDS + 3)) {
            self.options.music_volume = self.options.music_volume.saturating_add(1);
            rebuild_meter = true;
        }

        if messages.contains(&ui::UiMessage::Triggered(VOLUME_MUSIC_IDS + 4)) {
            self.options.music_volume = self.options.music_volume.saturating_add(10);
            rebuild_meter = true;
        }

        if rebuild_meter {
            self.gui.remove_elements(VOLUME_MUSIC_IDS);
            self.gui.add_element(
                VOLUME_MUSIC_CONTAINER_ID,
                create_sound_adjuster(ctx, VOLUME_MUSIC_IDS, self.options.music_volume),
            );
        }

        // Reset keybinginds

        if messages.contains(&ui::UiMessage::Triggered(1)) {
            self.controller = super::game_state::Controller::default();
        }
        if messages.contains(&ui::UiMessage::Triggered(2)) {
            self.options.tutorial = true;
        }

        // Exit options

        if messages.contains(&ui::UiMessage::Triggered(3)) {
            if self.controller.save_to_file("./data/keymap.toml").is_err() {
                println!("[WARNING] Could not save keybindings.")
            }
            // save internally
            options::OPTIONS.with(|opt| *opt.borrow_mut() = self.options);

            Ok(mooeye::scene_manager::SceneSwitch::Pop(1))
        } else {
            Ok(mooeye::scene_manager::SceneSwitch::None)
        }
    }

    fn draw(&mut self, ctx: &mut ggez::Context, mouse_listen: bool) -> Result<(), GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, None);
        canvas.set_sampler(graphics::Sampler::nearest_clamp());

        self.gui.draw_to_screen(ctx, &mut canvas, mouse_listen);

        canvas.finish(ctx)?;
        Ok(())
    }
}

fn create_sound_adjuster(ctx: &ggez::Context, id_start: u32, value: u8) -> ui::UiElement<()> {
    ui::containers::HorizontalBox::new_spaced(0.)
        .to_element_builder(id_start, ctx)
        .with_child(
            graphics::Text::new(
                graphics::TextFragment::new("<<").color(graphics::Color::from_rgb_u32(PALETTE[6])),
            )
            .set_font("Retro")
            .set_scale(28.)
            .to_owned()
            .to_element_builder(id_start + 1, ctx)
            .with_visuals(ui::Visuals {
                corner_radii: [0., 0., 3., 3.],
                border_widths: [3., 1.5, 3., 3.],
                ..super::BUTTON_VIS
            })
            .with_hover_visuals(ui::Visuals {
                corner_radii: [0., 0., 3., 3.],
                border_widths: [3., 1.5, 3., 3.],
                ..super::BUTTON_HOVER_VIS
            })
            .with_trigger_sound(
                ggez::audio::Source::new(ctx, "/audio/sounds/ui/blipSelect.wav").ok(),
            )
            .as_shrink()
            .build(),
        )
        .with_child(
            graphics::Text::new(
                graphics::TextFragment::new("< ").color(graphics::Color::from_rgb_u32(PALETTE[6])),
            )
            .set_font("Retro")
            .set_scale(28.)
            .to_owned()
            .to_element_builder(id_start + 2, ctx)
            .with_visuals(ui::Visuals {
                corner_radii: [0.; 4],
                border_widths: [3., 1.5, 3., 1.5],
                ..super::BUTTON_VIS
            })
            .with_hover_visuals(ui::Visuals {
                corner_radii: [0.; 4],
                border_widths: [3., 1.5, 3., 1.5],
                ..super::BUTTON_HOVER_VIS
            })
            .with_trigger_sound(
                ggez::audio::Source::new(ctx, "/audio/sounds/ui/blipSelect.wav").ok(),
            )
            .as_shrink()
            .build(),
        )
        .with_child(
            graphics::Text::new(
                graphics::TextFragment::new(format!("{}", value))
                    .color(graphics::Color::from_rgb_u32(PALETTE[6])),
            )
            .set_font("Retro")
            .set_scale(24.)
            .to_owned()
            .to_element_builder(0, ctx)
            .with_visuals(ui::Visuals {
                corner_radii: [0.; 4],
                border_widths: [3., 1.5, 3., 1.5],
                ..super::BUTTON_VIS
            })
            .with_trigger_sound(
                ggez::audio::Source::new(ctx, "/audio/sounds/ui/blipSelect.wav").ok(),
            )
            .as_fill()
            .build(),
        )
        .with_child(
            graphics::Text::new(
                graphics::TextFragment::new(" >").color(graphics::Color::from_rgb_u32(PALETTE[6])),
            )
            .set_font("Retro")
            .set_scale(28.)
            .to_owned()
            .to_element_builder(id_start + 3, ctx)
            .with_visuals(ui::Visuals {
                corner_radii: [0.; 4],
                border_widths: [3., 1.5, 3., 1.5],
                ..super::BUTTON_VIS
            })
            .with_hover_visuals(ui::Visuals {
                corner_radii: [0.; 4],
                border_widths: [3., 1.5, 3., 1.5],
                ..super::BUTTON_HOVER_VIS
            })
            .with_trigger_sound(
                ggez::audio::Source::new(ctx, "/audio/sounds/ui/blipSelect.wav").ok(),
            )
            .as_shrink()
            .build(),
        )
        .with_child(
            graphics::Text::new(
                graphics::TextFragment::new(">>").color(graphics::Color::from_rgb_u32(PALETTE[6])),
            )
            .set_font("Retro")
            .set_scale(28.)
            .to_owned()
            .to_element_builder(id_start + 4, ctx)
            .with_visuals(ui::Visuals {
                corner_radii: [3., 3., 0., 0.],
                border_widths: [3., 3., 3., 1.5],
                ..super::BUTTON_VIS
            })
            .with_hover_visuals(ui::Visuals {
                corner_radii: [3., 3., 0., 0.],
                border_widths: [3., 3., 3., 1.5],
                ..super::BUTTON_HOVER_VIS
            })
            .as_shrink()
            .build(),
        )
        .as_fill()
        .build()
}
