use super::{components::GameAction, game_action::ActionQueue, game_message::{MessageSet, GameMessage}};
use legion::*;

pub struct GameData {
    score: u32,
    gold: u32,
    pub city_health: i32,
}

impl GameData {
    pub fn add_gold(&mut self, amount: u32) {
        self.score += amount;
        self.gold += amount;
    }

    #[allow(dead_code)]
    pub fn spend(&mut self, amount: u32) -> bool {
        if amount <= self.gold {
            self.gold -= amount;
            true
        } else {
            false
        }
    }
}

impl Default for GameData {
    fn default() -> Self {
        Self {
            score: 0,
            gold: 0,
            city_health: 100,
        }
    }
}

#[system]
pub fn resolve_gama_data(
    #[resource] actions: &mut ActionQueue,
    #[resource] game_data: &mut GameData,
    #[resource] messages: &mut MessageSet,
) {
    let mut change_gold = false;
    let mut change_city = false;
    for action in actions {
        if let (_, GameAction::GainGold { amount }) = action {
            game_data.add_gold(*amount);
            change_gold = true;
        } else if let (_, GameAction::TakeCityDamage { dmg }) = action {
            game_data.city_health -= *dmg as i32;
            change_city = true;
        }
    }

    if change_gold {
        messages.insert(mooeye::UiMessage::Extern(GameMessage::UpdateGold(game_data.gold)));
    }

    if change_city{
        messages.insert(mooeye::UiMessage::Extern(GameMessage::UpdateCityHealth(game_data.city_health)));
    }
}
