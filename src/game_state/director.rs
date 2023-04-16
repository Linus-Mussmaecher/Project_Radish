use std::time::Duration;

use ggez::Context;
use legion::World;
use mooeye::sprite;

use super::components;

pub struct Director {
    last_spawn: Duration,
}

impl Director {
    pub fn new() -> Self {
        Self {
            last_spawn: Duration::ZERO,
        }
    }

    pub fn progress(&mut self, ctx: &Context, world: &mut World) {
        self.last_spawn += ctx.time.delta();
        if self.last_spawn >= Duration::from_secs(5) {
            self.last_spawn = Duration::ZERO;

            world.push((
                components::Position::new(208., -20.),
                components::Velocity::new(0., 2.),
                sprite::Sprite::from_path_fmt(
                    "/sprites/skeleton_basic_16_16.png",
                    ctx,
                    Duration::from_secs_f32(0.25),
                )
                .expect("Could not load sprite."),
                components::Enemy::new(1, 10),
                components::Health::new(4),
                components::Collision::new_basic(64., 64.),
            ));
        }
    }
}
