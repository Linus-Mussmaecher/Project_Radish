use mooeye::UiElement;

#[derive(Clone, Copy, Debug, PartialEq, Eq, std::hash::Hash)]
pub enum GameMessage {
    UpdateCityHealth(i32),
    UpdateGold(i32),
    UpdateSpellSlots(usize, u8),
    NextWave(i32),
    EnemyKilled(i32),
}

pub type MessageSet = std::collections::HashSet<mooeye::UiMessage<GameMessage>>;

pub trait MessageReceiver {
    fn receive(
        &mut self,
        message: &mooeye::UiMessage<GameMessage>,
        gui: &mut UiElement<GameMessage>,
        ctx: &ggez::Context,
    );
}
