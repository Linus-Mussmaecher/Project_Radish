use ggez::{glam::Vec2, graphics::Rect};
use legion::{world::SubWorld, *};
use mooeye::UiMessage;

use crate::game_state::{
    game_action::ActionQueue,
    game_message::{GameMessage, MessageSet},
};

use super::{GameAction, Position};
pub struct Collision {
    w: f32,
    h: f32,

    self_collision_actions: Vec<GameAction>,
    other_collision_actions: Vec<GameAction>,
    collision_messages: Vec<UiMessage<GameMessage>>,

    immunity: Vec<Entity>,
}

impl Collision {
    pub fn new(
        w: f32,
        h: f32,
        self_collision_actions: impl Into<Option<Vec<GameAction>>>,
        other_collision_actions: impl Into<Option<Vec<GameAction>>>,
        collision_messages: impl Into<Option<Vec<UiMessage<GameMessage>>>>,
    ) -> Self {
        Self {
            w,
            h,
            self_collision_actions: self_collision_actions.into().unwrap_or_else(|| Vec::new()),
            other_collision_actions: other_collision_actions.into().unwrap_or_else(|| Vec::new()),
            collision_messages: collision_messages.into().unwrap_or_else(|| Vec::new()),
            immunity: Vec::new(),
        }
    }

    pub fn new_basic(w: f32, h: f32) -> Self {
        Self::new(w, h, None, None, None)
    }

    fn get_collider(&self, pos: Vec2) -> Rect {
        Rect::new(pos.x, pos.y, self.w, self.h)
    }
}

#[system]
#[read_component(Position)]
#[read_component(Collision)]
pub fn collision(
    world: &mut SubWorld,
    #[resource] actions: &mut ActionQueue,
    #[resource] messages: &mut MessageSet,
) {
    for (ent1, pos1, col1) in <(Entity, &Position, &Collision)>::query().iter(world) {
        for (ent2, pos2, col2) in <(Entity, &Position, &Collision)>::query().iter(world) {
            if col1.get_collider(*pos1).overlaps(&col2.get_collider(*pos2))
                && *ent1 != *ent2
                && !col1.immunity.contains(ent2)
            {
                messages.extend(col1.collision_messages.iter());
                actions.extend(
                    col1.self_collision_actions
                        .iter()
                        .map(|action| (*ent1, *action)),
                );
                actions.extend(
                    col1.other_collision_actions
                        .iter()
                        .map(|action| (*ent2, *action)),
                );
            }
        }
    }
}

#[system(for_each)]
pub fn resolve_immunities(this: &Entity, collision: &mut Collision, #[resource] actions: &ActionQueue){
    for (ent, action) in actions{
        if *this == *ent{
            if let GameAction::AddImmunity { other } = action{
                collision.immunity.push(*other);
            }
        }
    }
}