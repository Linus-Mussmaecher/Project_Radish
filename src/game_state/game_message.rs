use mooeye::UiElement;

#[derive(Clone, Copy, Debug, PartialEq, Eq, std::hash::Hash)]
pub enum GameMessage {
    UpdateCityHealth(i32),
    UpdateGold(i32),
    UpdateSpellSlots(usize, u8),
    NextWave(i32),
}

pub type MessageSet = std::collections::HashSet<mooeye::UiMessage<GameMessage>>;

pub trait MessageReceiver{
    fn receive<T: Copy + Eq + std::hash::Hash + 'static>(&mut self, message: &mooeye::UiMessage<GameMessage>) -> Vec<(u32, UiElement<T>)>;
}
