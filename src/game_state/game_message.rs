
#[derive(Clone, Copy, Debug, PartialEq, Eq, std::hash::Hash)]
pub enum GameMessage{
    
}


pub type MessageSet = std::collections::HashSet<mooeye::UiMessage<GameMessage>>;