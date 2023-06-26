use ggez::graphics;
use mooeye::*;

use crate::PALETTE;

use super::{game_message, MessageReceiver};

pub const TUTORIAL_BOX: u32 = 600;
pub const TUTORIAL_INNER: u32 = 601;
pub const TUTORIAL_CLOSE: u32 = 602;

#[derive(Debug, Clone)]
pub struct TutorialMessage {
    title: String,
    message: String,
    shown: bool,
    condition: game_message::UiMessageFilter,
}

impl TutorialMessage {
    fn new(title: &str, message: &str, condition: game_message::UiMessageFilter) -> Self {
        Self {
            title: title.to_owned(),
            message: message.to_owned(),
            shown: false,
            condition,
        }
    }

    fn to_ui_element(&self, ctx: &ggez::Context) -> UiElement<game_message::GameMessage> {
        containers::VerticalBox::new_spaced(15.)
            .to_element_builder(TUTORIAL_INNER, ctx)
            .with_visuals(super::super::BUTTON_VIS)
            .with_child(
                graphics::Text::new("")
                    .add(
                        graphics::TextFragment::new(&self.title)
                            .font("Retro")
                            .scale(28.)
                            .color(graphics::Color::from_rgb_u32(PALETTE[7])),
                    )
                    .add(graphics::TextFragment::new("\n"))
                    .add(
                        graphics::TextFragment::new(&self.message)
                            .font("Retro")
                            .scale(20.)
                            .color(graphics::Color::from_rgb_u32(PALETTE[6])),
                    )
                    .to_owned()
                    .to_element(0, ctx),
            )
            .with_child(
                graphics::Text::new(
                    graphics::TextFragment::new("Close")
                        .color(graphics::Color::from_rgb_u32(PALETTE[6]))
                        .font("Retro")
                        .scale(28.),
                )
                .to_element_builder(TUTORIAL_CLOSE, ctx)
                .with_trigger_key(ggez::winit::event::VirtualKeyCode::C)
                .with_visuals(super::super::BUTTON_VIS)
                .with_hover_visuals(super::super::BUTTON_HOVER_VIS)
                .build(),
            )
            .with_alignment(ui_element::Alignment::Max, ui_element::Alignment::Center)
            .build()
    }
}
//programmiert by cutinus, dem programmschreibenden mit strammem max

pub struct TutorialManager {
    messages: Vec<TutorialMessage>,
}

impl TutorialManager {
    pub fn new() -> Self {
        Self {
            messages: vec![
                TutorialMessage::new(
                    "Welcome!",
                    "Kill skeletons pls.",
                    game_message::UiMessageFilter::Ui(UiMessage::Triggered(
                        super::ui::wave_menu::ID_NEXT_WAVE,
                    )),
                ),
                TutorialMessage::new(
                    "Still welcome!",
                    "This is the lookout menu.",
                    game_message::UiMessageFilter::Ui(UiMessage::Triggered(
                        super::ui::wave_menu::ID_ENEMIES,
                    )),
                ),
                TutorialMessage::new(
                    "Great Job!",
                    "You killed that skeleton.",
                    game_message::UiMessageFilter::Ext(
                        game_message::GameMessage::EnemyKilled(0),
                        game_message::GameMessageFilter::Type,
                    ),
                ),
            ],
        }
    }
}

impl MessageReceiver for TutorialManager {
    fn receive(
        &mut self,
        message: &mooeye::UiMessage<super::GameMessage>,
        gui: &mut UiElement<super::GameMessage>,
        ctx: &ggez::Context,
    ) {
        for tut_message in self.messages.iter_mut().filter(|tm| !tm.shown) {
            if tut_message.condition.check(message) {
                tut_message.shown = true;
                gui.add_element(TUTORIAL_BOX, tut_message.to_ui_element(ctx));
            }
        }
    }
}
