use super::{
    components::{actions::GameAction, buildings::Buildings},
    game_message::{GameMessage, MessageSet},
};
use legion::*;

/// A struct that holds some general data about the game play.
pub struct GameData {
    /// The current total score achieved.
    score: i32,
    /// The gold the player currently holds (= score - gold spent).
    gold: i32,
    /// The gold the player had on the last pass (may have changed from the outside)
    last_gold: i32,
    /// The health the city has left. Public to allow easy access.
    pub city_health: i32,
    /// The current state of buildings
    pub buildings: Buildings,
}

impl GameData {
    /// Creates a new GameData struct with the passed game play parameters.
    pub fn new(gold: i32, city_health: i32) -> Self {
        Self {
            score: 0,
            gold,
            last_gold: 0,
            city_health,
            buildings: Buildings::new(),
        }
    }

    /// Adds both gold and score to the player.
    pub fn add_gold(&mut self, amount: i32) {
        self.score += amount;
        self.gold += amount;
    }

    /// Attempts to spend a certain amount of gold.
    /// If the player has enough gold, the amount is subtracted and true is returned.
    /// Otherwise, no gold is subtracted and false is returned.
    pub fn spend(&mut self, amount: i32) -> bool {
        if amount <= self.gold {
            self.gold -= amount;
            true
        } else {
            false
        }
    }

    /// Returns the current score.
    pub fn get_score(&self) -> i32 {
        self.score
    }
}

/// A system that handles changes to game data, such as city damage or earning gold.
#[system(for_each)]
pub fn resolve_gama_data(
    actions: &super::components::Actions,
    #[resource] game_data: &mut GameData,
    #[resource] messages: &mut MessageSet,
) {
    let mut change_city = false;
    for action in actions.get_actions() {
        match action {
            GameAction::GainGold { amount } => {
                game_data.add_gold(*amount);
            }
            GameAction::TakeCityDamage { dmg } => {
                game_data.city_health -= *dmg;
                change_city = true;
            }
            _ => {}
        }
    }

    if game_data.last_gold != game_data.gold {
        messages.insert(mooeye::ui::UiMessage::Extern(GameMessage::UpdateGold(
            game_data.gold,
        )));
        game_data.last_gold = game_data.gold;
    }

    if change_city {
        messages.insert(mooeye::ui::UiMessage::Extern(
            GameMessage::UpdateCityHealth(game_data.city_health),
        ));
    }
}
