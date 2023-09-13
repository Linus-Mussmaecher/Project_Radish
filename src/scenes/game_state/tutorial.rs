use good_web_game::graphics;
use mooeye::{ui, ui::UiContent};

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

    fn to_ui_element(
        &self,
        ctx: &mut good_web_game::Context,
    ) -> ui::UiElement<game_message::GameMessage> {
        ui::containers::VerticalBox::new_spaced(15.)
            .to_element_builder(TUTORIAL_INNER, ctx)
            .with_visuals(super::super::BUTTON_VIS)
            .with_padding((10., 10., 10., 10.))
            .with_alignment(ui::Alignment::Max, ui::Alignment::Center)
            .with_size(ui::Size::Fixed(380.), None)
            .with_child({
                graphics::Text::new("")
                    .add(
                        graphics::TextFragment::new(self.title.as_str())
                            .font(crate::RETRO.with(|f| f.borrow().unwrap()))
                            .scale(28.)
                            .color(graphics::Color::from_rgb_u32(PALETTE[7])),
                    )
                    .add(graphics::TextFragment::new("\n"))
                    .add(
                        graphics::TextFragment::new(self.message.as_str())
                            .font(crate::RETRO.with(|f| f.borrow().unwrap()))
                            .scale(20.)
                            .color(graphics::Color::from_rgb_u32(PALETTE[6])),
                    )
                    .set_bounds(graphics::Point2::new(350., 1000.), graphics::Align::Left)
                    .to_owned()
                    .to_element_builder(0, ctx)
                    .build()
            })
            .with_child(
                graphics::Text::new(
                    graphics::TextFragment::new("Continue")
                        .color(graphics::Color::from_rgb_u32(PALETTE[6]))
                        .font(crate::RETRO.with(|f| f.borrow().unwrap()))
                        .scale(28.),
                )
                .to_element_builder(TUTORIAL_CLOSE, ctx)
                .with_trigger_key(good_web_game::input::keyboard::KeyCode::C)
                .with_visuals(super::super::BUTTON_VIS)
                .with_hover_visuals(super::super::BUTTON_HOVER_VIS)
                .build(),
            )
            .build()
    }
}
//programmiert by cutinus, dem programmschreibenden mit strammem max

pub struct TutorialManager {
    messages: Vec<TutorialMessage>,
    active: bool,
}

impl TutorialManager {
    pub fn new_empty() -> Self {
        Self {
            messages: vec![],
            active: false,
        }
    }

    pub fn new() -> Self {
        Self {
            active: false,
            messages: vec![
                TutorialMessage::new(
                    "Defend the town!",
                    "Kill skeletons before they reach the town!\n\n\
                    Move with A/D and cast spells with J/K/L/;. You can hover your spells to see what they do.",
                    game_message::UiMessageFilter::Ext(game_message::GameMessage::UpdateCityHealth(0), game_message::GameMessageFilter::Type),
                ),
                TutorialMessage::new(
                    "You just cast a spell",
                    "Casting a spell blocks some of your spells slots for some time.\n\n\
                    More powerful spells block more spell slots, and for longer.\n\n\
                    You can view the status of your spell slots in the top left.\n\n\
                    Hover over your spells to see their spell slot requirements.",
                    game_message::UiMessageFilter::Ext(game_message::GameMessage::UpdateSpellSlots(0, 16), game_message::GameMessageFilter::Max),
                ),
                TutorialMessage::new(
                    "You killed an enemy",
                    "Killing enemies grants you gold and increases your total score.\n\n\
                    Gold is used to purchase upgrades between waves.",
                    game_message::UiMessageFilter::Ext(
                        game_message::GameMessage::EnemyKilled(0),
                        game_message::GameMessageFilter::Type,
                    ),
                ),
                TutorialMessage::new(
                    "You killed an elite enemy",
                    "Elite enemies are tougher than normal enemies, have more abilities and deal more damage to your city.\n\n\
                    But they are also worth a lot more gold!",
                    game_message::UiMessageFilter::Ext(
                        game_message::GameMessage::EnemyKilled(10),
                        game_message::GameMessageFilter::Min,
                    ),
                ),
                TutorialMessage::new(
                    "Brief Respite",
                    "You survived the first wave! Take some time to look around town.\n\n\
                    Click the three left icons to look at your available options.\n\n\
                    When you are done, use the right arrow icon to start the next wave.",
                    game_message::UiMessageFilter::Ext(game_message::GameMessage::NextWave(2),game_message::GameMessageFilter::Equality),
                ),
                TutorialMessage::new(
                    "Lookout",
                    "Here you can view what enemies you will face in the next wave.\n\n\
                    After upgrading your lookout in the construction menu, you can reroll this selection.\n\n\
                    Rerolling gets more expensive every time.",
                    game_message::UiMessageFilter::Ui(ui::UiMessage::Triggered(
                        super::ui::wave_menu::ID_ENEMIES,
                    )),
                ),
                TutorialMessage::new(
                    "Spell Book",
                    "Here you can view your equipped and available spells.\n\n\
                    Purchase spells for gold by clicking them. \
                    Purchasing some spells requires you to upgrade your mage guild in the buildings menu first. \
                    Purchasing a spell also makes further spells more expensive.\n\n\
                    To equip a spell for the next wave, click it and then click the slot you want it in.",
                    game_message::UiMessageFilter::Ui(ui::UiMessage::Triggered(
                        super::ui::wave_menu::ID_SPELLS,
                    )),
                ),
                TutorialMessage::new(
                    "Construction",
                    "Here you can construct and upgrade your town's buildings for gold.\n\n\
                    Construct the watchtower to reroll approaching enemies and later increase your movement speed.\n\n\
                    Construct and upgrade the Mage's Guild to allow the purchase of more powerful spells.\n\n\
                    Construct and upgrade the mana well to increase the amount of spell slots available in combat.",
                    game_message::UiMessageFilter::Ui(ui::UiMessage::Triggered(
                        super::ui::wave_menu::ID_HOUSE,
                    )),
                ),
                TutorialMessage::new(
                    "You built a building",
                    "Constructing a building gives you a permanent upgrade.\n\n\
                    Additionally, buildings can protect your town. Enemies getting past you will first destroy your buildings before damaging your town.",
                    game_message::UiMessageFilter::Ext(
                        game_message::GameMessage::BuildingUp(0, 0),
                        game_message::GameMessageFilter::Type,
                    ),
                ),
                TutorialMessage::new(
                    "You just lost a building",
                    "Enemies getting past you will first destroy your buildings before damaging your town.\n\n\
                    You can always rebuild the destroyed building.",
                    game_message::UiMessageFilter::Ext(
                        game_message::GameMessage::BuildingDown(0, 0),
                        game_message::GameMessageFilter::Type,
                    ),
                ),
            ],
        }
    }

    pub fn is_active(&self) -> bool {
        self.active
    }
}

impl MessageReceiver for TutorialManager {
    fn receive(
        &mut self,
        message: &ui::UiMessage<super::GameMessage>,
        gui: &mut ui::UiElement<super::GameMessage>,
        ctx: &mut good_web_game::Context,
        gfx_ctx: &mut good_web_game::event::GraphicsContext,
    ) {
        for tut_message in self.messages.iter_mut().filter(|tm| !tm.shown) {
            if tut_message.condition.check(message) {
                tut_message.shown = true;
                if self.active {
                    gui.remove_elements(TUTORIAL_INNER);
                }
                gui.add_element(TUTORIAL_BOX, tut_message.to_ui_element(ctx));
                self.active = true;
            }
        }

        if let ui::UiMessage::Triggered(TUTORIAL_CLOSE) = message {
            self.active = false;
        }
    }
}
