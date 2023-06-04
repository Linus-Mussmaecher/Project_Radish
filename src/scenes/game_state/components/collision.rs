use ggez::{glam::Vec2, graphics::Rect};
use legion::{system, Entity, EntityStore, IntoQuery};

use super::actions::Actions;

/// A custom type to remember a set of delayed actions.
type ActionQueue = Vec<(legion::Entity, GameAction)>;

use super::{actions::GameAction, Position};

/// A component that manages an entities collision box and collision handling.
pub struct Collision {
    /// Width of this elements collider.
    w: f32,
    /// Height of this elements collider.
    h: f32,

    /// A function that provides a number of actions and messages to execute on collision, based on the entities colliding.
    collision_handler: Box<dyn Fn(Entity, Entity) -> ActionQueue + Send + Sync>,

    /// A list of all entities that cannot collide with this one.
    immunity: Vec<Entity>,
}

impl Collision {
    /// Creates a new collision component.
    pub fn new(
        w: f32,
        h: f32,
        collision_handler: impl Fn(Entity, Entity) -> ActionQueue + Send + Sync + 'static,
    ) -> Self {
        Self {
            w,
            h,
            collision_handler: Box::new(collision_handler),
            immunity: Vec::new(),
        }
    }

    /// Creates a new collision component that does itself not apply actions or send messages (but can trigger collisions with other collision components).
    pub fn new_basic(w: f32, h: f32) -> Self {
        Self::new(w, h, |_, _| ActionQueue::new())
    }

    /// Returns the collision bounds (x,y,w,h) of this component.
    fn get_collider(&self, pos: Vec2) -> Rect {
        Rect::new(pos.x - self.w / 2., pos.y - self.h / 2., self.w, self.h)
    }

    /// Builder function that makes an entity immune to collisions from this entity.
    pub fn with_immunity(mut self, entity: Entity) -> Self {
        self.immunity.push(entity);
        self
    }
}

/// A component that manages wether this element respects the game boundaries.
pub struct BoundaryCollision {
    /// Wether the element respects the left and right boundaries.
    x_boundaries: bool,
    /// Wether the element respects the top and bottom boundaries.
    y_boundaries: bool,
    /// Wether the element will simply continue running against the boundary or bounce off it.
    bounce: bool,
}

impl BoundaryCollision {
    /// Creates a new boundary component.
    pub fn new(x_boundaries: bool, y_boundaries: bool, bounce: bool) -> Self {
        Self {
            x_boundaries,
            y_boundaries,
            bounce,
        }
    }
}

#[system(for_each)]
/// A system that manages collisions of entities with the boundary of the game world.
pub fn boundary_collision(
    pos: &mut Position,
    bcol: &BoundaryCollision,
    col: Option<&Collision>,
    gfx: Option<&super::Graphics>,
    vel: Option<&mut super::Velocity>,
    #[resource] boundaries: &Rect,
) {
    //try to get a reasonable height
    let (w, h) = if let Some(col) = col {
        (col.w, col.h)
    } else if let Some(gfx) = gfx {
        gfx.get_size()
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
                    -1.
                } else {
                    1.
                } * vel.get_dx(),
                if bcol.y_boundaries
                    && (pos.y < mod_boundaries.y || pos.y > mod_boundaries.y + mod_boundaries.h)
                {
                    -1.
                } else {
                    1.
                } * vel.get_dy(),
            );
        }
    }

    // clamp x-position
    if bcol.x_boundaries {
        pos.x = pos
            .x
            .clamp(mod_boundaries.x, mod_boundaries.x + mod_boundaries.w);
    }

    // clamp y-position
    if bcol.y_boundaries {
        pos.y = pos
            .y
            .clamp(mod_boundaries.y, mod_boundaries.y + mod_boundaries.h);
    }
}

#[system]
#[read_component(Position)]
#[read_component(Collision)]
#[write_component(Actions)]
/// A system that manages collisions of entities with each other.
pub fn collision(world: &mut legion::world::SubWorld) {
    // Create a list of all actions triggered by collisions.
    let mut total_actions: Vec<(Entity, GameAction)> = Vec::new();

    // Iterate over all pairs of possible colliders.
    for (ent1, pos1, col1) in <(Entity, &Position, &Collision)>::query().iter(world) {
        for (ent2, pos2, col2) in <(Entity, &Position, &Collision)>::query().iter(world) {
            // check for collision
            if col1.get_collider(*pos1).overlaps(&col2.get_collider(*pos2))
                && *ent1 != *ent2
                && !col1.immunity.contains(ent2)
            {
                let n_actions = (col1.collision_handler)(*ent1, *ent2);
                total_actions.extend(n_actions);
            }
        }
    }

    // Apply all remembered actions.
    for (ent, action) in total_actions {
        if let Ok(mut entry) = world.entry_mut(ent) {
            if let Ok(actions) = entry.get_component_mut::<Actions>() {
                actions.push(action);
            }
        }
    }
}

#[system(for_each)]
/// A system that resolvess all [GameAction::AddImmunity] actions by adding the entities to the immunity lists.
pub fn resolve_immunities(collision: &mut Collision, actions: &Actions) {
    for action in actions.get_actions() {
        if let GameAction::AddImmunity { other } = action {
            collision.immunity.push(*other);
        }
    }
}
