use mooeye::UiElement;

#[derive(Clone, Copy, Debug, PartialEq, Eq, std::hash::Hash)]
pub enum GameMessage {
    UpdateCityHealth(i32),
    UpdateGold(i32),
    // needs to use u8 instead of f32 to allow ordering
    UpdateSpellSlots(usize, u8),
    NextWave(i32),
    EnemyKilled(u8),
    BuildingUp(usize, u8),
    BuildingDown(usize, u8),
}

impl PartialOrd for GameMessage {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::UpdateCityHealth(a), Self::UpdateCityHealth(b)) => Some(a.cmp(b)),
            (Self::UpdateGold(a), Self::UpdateGold(b)) => Some(a.cmp(b)),
            (Self::UpdateSpellSlots(_, a), Self::UpdateSpellSlots(_, b)) => Some(a.cmp(b)),
            (Self::NextWave(a), Self::NextWave(b)) => Some(a.cmp(b)),
            (Self::EnemyKilled(a), Self::EnemyKilled(b)) => Some(a.cmp(b)),
            (Self::BuildingUp(a, lvl_a), Self::BuildingUp(b, lvl_b)) => if *a == *b {
                Some(lvl_a.cmp(lvl_b))
            } else {
                None
            },
            (Self::BuildingDown(a, _), Self::BuildingDown(b, _)) => Some(a.cmp(b)),
            (_, _) => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GameMessageFilter {
    Equality,
    Max,
    Min,
    Type,
}

impl GameMessageFilter {
    pub fn check(&self, model: &GameMessage, to_check: &GameMessage) -> bool {
        match self {
            GameMessageFilter::Equality => model == to_check,
            GameMessageFilter::Max => model >= to_check,
            GameMessageFilter::Min => model <= to_check,
            GameMessageFilter::Type => model <= to_check || model >= to_check,
        }
    }
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
