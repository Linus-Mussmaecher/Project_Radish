use crate::scenes::options;
use ggez::{graphics, GameError};
use mooeye::{*};

use crate::PALETTE;

const VOLUME_IDS: u32 = 11;
const VOLUME_MUSIC_IDS: u32 = 21;
const VOLUME_CONTAINER_ID: u32 = 10;
const VOLUME_MUSIC_CONTAINER_ID: u32 = 20;

pub struct OptionsMenu {
    gui: UiElement<()>,
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
        .set_scale(24.)
        .to_owned()
        .to_element_builder(1, ctx)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .with_trigger_key(ggez::winit::event::VirtualKeyCode::R)
        .build();

        let sound = graphics::Text::new(
            graphics::TextFragment::new("Sound Volume")
                .color(graphics::Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(24.)
        .to_owned()
        .to_element_builder(0, ctx)
        .build();

        let music = graphics::Text::new(
            graphics::TextFragment::new("Music Volume")
                .color(graphics::Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(24.)
        .to_owned()
        .to_element_builder(0, ctx)
        .build();

        let back = graphics::Text::new(
            graphics::TextFragment::new("Close").color(graphics::Color::from_rgb_u32(PALETTE[6])),
        )
        .set_font("Retro")
        .set_scale(32.)
        .to_owned()
        .to_element_builder(2, ctx)
        .with_trigger_key(ggez::winit::event::VirtualKeyCode::C)
        .with_visuals(super::BUTTON_VIS)
        .with_hover_visuals(super::BUTTON_HOVER_VIS)
        .build();

    let options = options::OptionsConfig::from_path(".data/options.toml").unwrap_or_default();

        // Container

        let options_box = mooeye::containers::VerticalBox::new_spaced(25.)
            .to_element_builder(0, ctx)
            .with_child(title)
            .with_child(sound)
            .with_child(
                containers::StackBox::new()
                .to_element_builder(VOLUME_CONTAINER_ID, ctx)
                .with_child(create_sould_adjuster(ctx, VOLUME_IDS, options.volume))
                .with_wrapper_layout(mooeye::ui_element::Layout::default())
                .build()
            )
            .with_child(music)
            .with_child(
                containers::StackBox::new()
                .to_element_builder(VOLUME_MUSIC_CONTAINER_ID, ctx)
                .with_child(create_sould_adjuster(ctx, VOLUME_MUSIC_IDS, options.music_volume))
                .with_wrapper_layout(mooeye::ui_element::Layout::default())
                .build()
            )
            .with_child(reset_bindings)
            .with_child(back)
            .with_visuals(super::BUTTON_VIS)
            .with_alignment(ui_element::Alignment::Min, ui_element::Alignment::Min)
            .with_offset(25., 25.)
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

        if messages.contains(&mooeye::UiMessage::Triggered(VOLUME_IDS + 1)) {
            // adjust sound
            self.options.volume -= 0.1;
            // set marker 
            rebuild_meter = true;
        }

        if messages.contains(&mooeye::UiMessage::Triggered(VOLUME_IDS + 2)) {
            self.options.volume -= 0.01;
            rebuild_meter = true;
        }

        if messages.contains(&mooeye::UiMessage::Triggered(VOLUME_IDS + 3)) {
            self.options.volume += 0.01;
            rebuild_meter = true;
        }

        if messages.contains(&mooeye::UiMessage::Triggered(VOLUME_IDS + 4)) {
            self.options.volume += 0.1;
            rebuild_meter = true;
        }

        if rebuild_meter {
            // remove old element
            self.gui.remove_elements(VOLUME_IDS);
            // add element with new value
            self.gui.add_element(VOLUME_CONTAINER_ID, create_sould_adjuster(ctx, VOLUME_IDS, self.options.volume));
            // reset marker
            rebuild_meter = false;
        }

        // Adjust music volume

        if messages.contains(&mooeye::UiMessage::Triggered(VOLUME_MUSIC_IDS + 1)) {
            self.options.music_volume -= 0.1;
            rebuild_meter = true;
        }

        if messages.contains(&mooeye::UiMessage::Triggered(VOLUME_MUSIC_IDS + 2)) {
            self.options.music_volume -= 0.01;
            rebuild_meter = true;
        }

        if messages.contains(&mooeye::UiMessage::Triggered(VOLUME_MUSIC_IDS + 3)) {
            self.options.music_volume += 0.01;
            rebuild_meter = true;
        }

        if messages.contains(&mooeye::UiMessage::Triggered(VOLUME_MUSIC_IDS + 4)) {
            self.options.music_volume += 0.1;
            rebuild_meter = true;
        }

        if rebuild_meter{
            self.gui.remove_elements(VOLUME_MUSIC_IDS);
            self.gui.add_element(VOLUME_MUSIC_CONTAINER_ID, create_sould_adjuster(ctx, VOLUME_MUSIC_IDS, self.options.music_volume));
        }

        // Reset keybinginds


        if messages.contains(&mooeye::UiMessage::Triggered(1)) {
            self.controller = super::game_state::Controller::default();
        }

        // Exit options

        if messages.contains(&mooeye::UiMessage::Triggered(2)) {
            if self.controller.save_to_file("./data/keymap.toml").is_err() {
                println!("[WARNING] Could not save keybindings.")
            }
            if self.options.save_to_file("./data/options.toml").is_err() {
                println!("[WARNING] Could not save options.")
            }
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

fn create_sould_adjuster(ctx: &ggez::Context, id_start: u32, value: f32) -> mooeye::UiElement<()> {
    containers::HorizontalBox::new_spaced(0.)
        .to_element_builder(id_start, ctx)
        .with_child(
            graphics::Text::new(
                graphics::TextFragment::new("<<").color(graphics::Color::from_rgb_u32(PALETTE[6])),
            )
            .set_font("Retro")
            .set_scale(28.)
            .to_owned()
            .to_element_builder(id_start + 1, ctx).with_visuals(ui_element::Visuals{
                corner_radii: [0., 0., 3., 3.],
                border_widths: [3., 1.5, 3., 3.], 
                ..super::BUTTON_VIS
            })
            .with_hover_visuals(ui_element::Visuals{
                corner_radii: [0., 0., 3., 3.],
                border_widths: [3., 1.5, 3., 3.], 
                ..super::BUTTON_HOVER_VIS
            })
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
            .to_element_builder(id_start + 2, ctx).with_visuals(ui_element::Visuals{
                corner_radii: [0.; 4],
                border_widths: [3., 1.5, 3., 1.5], 
                ..super::BUTTON_VIS
            })
            .with_hover_visuals(ui_element::Visuals{
                corner_radii: [0.; 4],
                border_widths: [3., 1.5, 3., 1.5], 
                ..super::BUTTON_HOVER_VIS
            })
            .as_shrink()
            .build(),
        )
        .with_child(
            graphics::Text::new(
                graphics::TextFragment::new(format!("{}", (value * 100.) as u8))
                    .color(graphics::Color::from_rgb_u32(PALETTE[6])),
            )
            .set_font("Retro")
            .set_scale(24.)
            .to_owned()
            .to_element_builder(0, ctx).with_visuals(ui_element::Visuals{
                corner_radii: [0.; 4],
                border_widths: [3., 1.5, 3., 1.5], 
                ..super::BUTTON_VIS
            })
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
            .with_visuals(ui_element::Visuals{
                corner_radii: [0.; 4],
                border_widths: [3., 1.5, 3., 1.5], 
                ..super::BUTTON_VIS
            })
            .with_hover_visuals(ui_element::Visuals{
                corner_radii: [0.; 4],
                border_widths: [3., 1.5, 3., 1.5], 
                ..super::BUTTON_HOVER_VIS
            })
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
            .with_visuals(ui_element::Visuals{
                corner_radii: [3., 3., 0., 0.],
                border_widths: [3., 3., 3., 1.5], 
                ..super::BUTTON_VIS
            })
            .with_hover_visuals(ui_element::Visuals{
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