use super::{
    components::actions::GameAction,
    game_message::{GameMessage, MessageSet},
};
use legion::*;

pub struct GameData {
    score: i32,
    gold: i32,
    pub city_health: i32,
}

impl GameData {
    pub fn add_gold(&mut self, amount: i32) {
        self.score += amount;
        self.gold += amount;
    }

    #[allow(dead_code)]
    pub fn spend(&mut self, amount: i32) -> bool {
        if amount <= self.gold {
            self.gold -= amount;
            true
        } else {
            false
        }
    }

    pub fn get_score(&self) -> i32{
        self.score
    }
}

impl Default for GameData {
    fn default() -> Self {
        Self {
            score: 0,
            gold: 0,
            city_health: 10,
        }
    }
}

#[system(for_each)]
pub fn resolve_gama_data(
    actions: &super::components::Actions,
    #[resource] game_data: &mut GameData,
    #[resource] messages: &mut MessageSet,
) {
    let mut change_gold = false;
    let mut change_city = false;
    for action in actions.get_actions() {
        match action {
            GameAction::GainGold { amount } => {
                game_data.add_gold(*amount);
                change_gold = true;
            }
            GameAction::TakeCityDamage { dmg } => {
                game_data.city_health -= *dmg as i32;
                change_city = true;
            }
            _ => {}
        }
    }

    if change_gold {
        messages.insert(mooeye::UiMessage::Extern(GameMessage::UpdateGold(
            game_data.gold,
        )));
    }

    if change_city {
        messages.insert(mooeye::UiMessage::Extern(GameMessage::UpdateCityHealth(
            game_data.city_health,
        )));
    }
}
