use std::time::Duration;

use ggez::{Context, GameError};
use legion::{Resources, World};
use mooeye::sprite;
use rand::random;

use super::components;

#[derive(Clone)]
pub struct Director {
    intervall: Duration,
    total: Duration,
    credits: u64,
    enemies: Vec<(
        u64,
        &'static dyn Fn(&mut World, &mut Resources, &Context) -> Result<(), GameError>,
    )>,
}

impl Director {
    pub fn new() -> Self {
        Self {
            intervall: Duration::ZERO,
            total: Duration::ZERO,
            credits: 0,
            enemies: vec![
                (40, &spawn_basic_skeleton),
                (70, &spawn_fast_skeleton),
                (120, &spawn_tank_skeleton),
            ],
        }
    }

    pub fn progress(
        &mut self,
        ctx: &Context,
        world: &mut World,
        resources: &mut Resources,
    ) -> Result<(), GameError> {
        // add time since last frame to counters

        self.intervall += ctx.time.delta();
        self.total += ctx.time.delta();

        // if a 1-second intervall has passed, attempt a spawn

        if self.intervall >= Duration::from_secs(1) {
            // grant credits
            self.credits += 10 + self.total.as_secs() / 3;
            // reset intervall
            self.intervall = Duration::ZERO;

            // randomly select an amount of available credits to spend
            let mut to_spend = (random::<f32>() * self.credits as f32) as u64;
            println!("Spending {} of {} credits.", to_spend, self.credits);

            // while credits left to spend
            'outer: loop {
                // randomly select a spawn among the optionis
                let mut enemy_ind = random::<usize>() % self.enemies.len();
                let mut enemy = self.enemies.get(enemy_ind);

                // downgrade spawn until affordable
                while match enemy {
                    Some((cost, _)) => *cost > to_spend,
                    None => true,
                } {
                    // if no downgrade possible, end this spending round
                    if enemy_ind == 0 {
                        break 'outer;
                    }
                    // otherwise, downgrade and try next enemy
                    enemy_ind -= 1;
                    enemy = self.enemies.get(enemy_ind);
                }

                // unpack enemy
                if let Some((cost, spawner)) = enemy {
                    // reduce available credits
                    to_spend -= cost;
                    self.credits -= cost;

                    // spawn
                    spawner(world, resources, ctx)?;
                }
            }
        }

        Ok(())
    }
}

pub fn spawn_basic_skeleton(world: &mut World, resources: &mut Resources, ctx: &Context) -> Result<(), GameError> {
    // get boundaries for random x
    let boundaries = *resources.get::<ggez::graphics::Rect>().ok_or_else(|| {
        ggez::GameError::CustomError("Could not unpack boundaries.".to_owned())
    })?;
    world.push((
        components::Position::new(random::<f32>() * boundaries.w + boundaries.x, -20.),
        components::Velocity::new(0., 40.),
        sprite::Sprite::from_path_fmt(
            "/sprites/skeleton_basic_16_16.png",
            ctx,
            Duration::from_secs_f32(0.25),
        )?,
        components::Enemy::new(1, 10),
        components::Health::new(4),
        components::Collision::new_basic(64., 64.),
    ));
    Ok(())
}

pub fn spawn_fast_skeleton(world: &mut World, resources: &mut Resources, ctx: &Context) -> Result<(), GameError> {
    // get boundaries for random x
    let boundaries = *resources.get::<ggez::graphics::Rect>().ok_or_else(|| {
        ggez::GameError::CustomError("Could not unpack boundaries.".to_owned())
    })?;
    world.push((
        components::Position::new(random::<f32>() * boundaries.w + boundaries.x, -20.),
        components::Velocity::new(45., 65.),
        components::BoundaryCollision::new(true, false, true),
        sprite::Sprite::from_path_fmt(
            "/sprites/skeleton_basic_16_16.png",
            ctx,
            Duration::from_secs_f32(0.20),
        )?,
        components::Enemy::new(1, 15),
        components::Health::new(3),
        components::Collision::new_basic(64., 64.),
    ));
    Ok(())
}

pub fn spawn_tank_skeleton(world: &mut World, resources: &mut Resources, ctx: &Context) -> Result<(), GameError> {
    // get boundaries for random x
    let boundaries = *resources.get::<ggez::graphics::Rect>().ok_or_else(|| {
        ggez::GameError::CustomError("Could not unpack boundaries.".to_owned())
    })?;
    world.push((
        components::Position::new(random::<f32>() * boundaries.w + boundaries.x, -20.),
        components::Velocity::new(0., 15.),
        sprite::Sprite::from_path_fmt(
            "/sprites/skeleton_sword_16_16.png",
            ctx,
            Duration::from_secs_f32(0.25),
        )?,
        components::Enemy::new(2, 25),
        components::Health::new(6),
        components::Collision::new_basic(64., 64.),
    ));
    Ok(())
}
