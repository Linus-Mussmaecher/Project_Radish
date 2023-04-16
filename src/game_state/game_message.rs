
#[derive(Clone, Copy, Debug, PartialEq, Eq, std::hash::Hash)]
pub enum GameMessage{
    UpdateCityHealth(i32),
    UpdateGold(i32),
}


pub type MessageSet = std::collections::HashSet<mooeye::UiMessage<GameMessage>>;