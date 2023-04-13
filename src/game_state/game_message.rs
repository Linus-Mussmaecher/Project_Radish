
#[derive(Clone, Copy, Debug, PartialEq, Eq, std::hash::Hash)]
pub enum GameMessage{
    SkeletonSpawned,
}


pub type MessageSet = std::collections::HashSet<mooeye::UiMessage<GameMessage>>;