use ggez::{glam::Vec2, graphics::Rect};
use legion::{systems::CommandBuffer, world::SubWorld, *};

use crate::game_state::{components::Actions, game_message::MessageSet};

type ActionQueue = Vec<(legion::Entity, GameAction)>;

use super::{actions::GameAction, Position};
pub struct Collision {
    w: f32,
    h: f32,

    collision_handler: Box<dyn Fn(Entity, Entity) -> (ActionQueue, MessageSet) + Send + Sync>,

    executive_collision_handler:
        Option<Box<dyn Fn(&mut CommandBuffer, Entity, Entity) + Send + Sync>>,

    immunity: Vec<Entity>,
}

impl Collision {
    pub fn new(
        w: f32,
        h: f32,
        collision_handler: impl Fn(Entity, Entity) -> (ActionQueue, MessageSet) + Send + Sync + 'static,
    ) -> Self {
        Self {
            w,
            h,
            collision_handler: Box::new(collision_handler),
            immunity: Vec::new(),
            executive_collision_handler: None,
        }
    }

    pub fn new_executive(
        w: f32,
        h: f32,
        collision_handler: impl Fn(Entity, Entity) -> (ActionQueue, MessageSet) + Send + Sync + 'static,
        executive_collision_handler: impl Fn(&mut CommandBuffer, Entity, Entity) + Send + Sync + 'static,
    ) -> Self {
        Self {
            w,
            h,
            collision_handler: Box::new(collision_handler),
            immunity: Vec::new(),
            executive_collision_handler: Some(Box::new(executive_collision_handler)),
        }
    }

    pub fn new_basic(w: f32, h: f32) -> Self {
        Self::new(w, h, |_, _| (ActionQueue::new(), MessageSet::new()))
    }

    fn get_collider(&self, pos: Vec2) -> Rect {
        Rect::new(pos.x - self.w / 2., pos.y - self.h / 2., self.w, self.h)
    }
}

pub struct BoundaryCollision {
    x_boundaries: bool,
    y_boundaries: bool,
    bounce: bool,
}

impl BoundaryCollision {
    pub fn new(x_boundaries: bool, y_boundaries: bool, bounce: bool) -> Self {
        Self {
            x_boundaries,
            y_boundaries,
            bounce,
        }
    }
}

#[system(for_each)]
pub fn boundary_collision(
    pos: &mut Position,
    bcol: &BoundaryCollision,
    col: Option<&Collision>,
    sprite: Option<&super::Sprite>,
    vel: Option<&mut super::Velocity>,
    #[resource] boundaries: &Rect,
) {
    //try to get a reasonable height
    let (w, h) = if let Some(col) = col {
        (col.w, col.h)
    } else if let Some(sprite) = sprite {
        sprite.get_dimensions()
    } else {
        (0., 0.)
    };

    // get valid boundaries for center of object
    let mod_boundaries = Rect::new(
        boundaries.x + w / 2.,
        boundaries.y + h / 2.,
        boundaries.w - w,
        boundaries.h - h,
    );

    // flip velocity if neccessary
    if bcol.bounce {
        if let Some(vel) = vel {
            *vel = super::Velocity::new(
                if bcol.x_boundaries
                    && (pos.x < mod_boundaries.x || pos.x > mod_boundaries.x + mod_boundaries.w)
                {
                    -vel.get_dx()
                } else {
                    vel.get_dx()
                },
                if bcol.y_boundaries
                    && (pos.y < mod_boundaries.y || pos.y > mod_boundaries.y + mod_boundaries.h)
                {
                    -vel.get_dy()
                } else {
                    vel.get_dy()
                },
            );
        }
    }

    // clamp x-position
    if bcol.x_boundaries {
        pos.x = pos
            .x
            .clamp(boundaries.x + w / 2., boundaries.x + boundaries.w - w / 2.);
    }

    // clamp y-position
    if bcol.y_boundaries {
        pos.y = pos
            .y
            .clamp(boundaries.y + h / 2., boundaries.y + boundaries.h - w / 2.);
    }
}

#[system]
#[read_component(Position)]
#[read_component(Collision)]
#[write_component(Actions)]
pub fn collision(
    world: &mut SubWorld,
    #[resource] messages: &mut MessageSet,
    cmd: &mut CommandBuffer,
) {
    // collision with other entities

    let mut total_actions: Vec<(Entity, GameAction)> = Vec::new();

    for (ent1, pos1, col1) in <(Entity, &Position, &Collision)>::query().iter(world) {
        for (ent2, pos2, col2) in <(Entity, &Position, &Collision)>::query().iter(world) {
            if col1.get_collider(*pos1).overlaps(&col2.get_collider(*pos2))
                && *ent1 != *ent2
                && !col1.immunity.contains(ent2)
            {
                //println!("Collisions: {:?}, {:?}", *ent1, *ent2);
                let (n_actions, n_messages) = (col1.collision_handler)(*ent1, *ent2);
                messages.extend(n_messages.iter());
                total_actions.extend(n_actions.iter());
                if let Some(exec) = &col1.executive_collision_handler {
                    exec(cmd, *ent1, *ent2);
                }
            }
        }
    }

    for (ent, action) in total_actions{
        if let Ok(mut entry) = world.entry_mut(ent){
            if let Ok(actions) = entry.get_component_mut::<Actions>(){
                actions.push(action);
            }
        }
    }
}

#[system(for_each)]
pub fn resolve_immunities(collision: &mut Collision, actions: &Actions) {
    for action in actions.get_actions() {
        if let GameAction::AddImmunity { other } = action {
            collision.immunity.push(*other);
        }
    }
}
