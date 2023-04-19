use std::time::Duration;

use ggez::{Context, GameError};
use legion::{Resources, World};
use rand::random;

use crate::sprite_pool::SpritePool;

use super::components;

#[derive(Clone)]
pub struct Director {
    intervall: Duration,
    total: Duration,
    credits: u64,
    enemies: Vec<(
        u64,
        &'static dyn Fn(&mut World, &mut Resources) -> Result<(), GameError>,
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
                (100, &spawn_loot_skeleton),
                (150, &spawn_tank_skeleton),
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
            let mut to_spend = (random::<f32>().powi(2) * self.credits as f32) as u64;
            if to_spend >= 40 {
                println!("Spending {} of {} credits.", to_spend, self.credits);
            }

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
                    spawner(world, resources)?;
                }
            }
        }

        Ok(())
    }
}

pub fn spawn_basic_skeleton(world: &mut World, resources: &mut Resources) -> Result<(), GameError> {
    // get boundaries for random x
    let boundaries = *resources
        .get::<ggez::graphics::Rect>()
        .ok_or_else(|| ggez::GameError::CustomError("Could not unpack boundaries.".to_owned()))?;
    let sprites = resources
        .get::<SpritePool>()
        .ok_or_else(|| ggez::GameError::CustomError("Could not unpack sprite pool.".to_owned()))?;
    world.push((
        components::Position::new(random::<f32>() * boundaries.w + boundaries.x, -20.),
        components::Velocity::new(0., 40.),
        sprites.init_sprite(
            "/sprites/enemies/skeleton_basic_16_16.png",
            Duration::from_secs_f32(0.25),
        )?,
        components::Enemy::new(1, 10),
        components::Health::new(4),
        components::Collision::new_basic(64., 64.),
    ));
    Ok(())
}

pub fn spawn_fast_skeleton(world: &mut World, resources: &mut Resources) -> Result<(), GameError> {
    // get boundaries for random x
    let boundaries = *resources
        .get::<ggez::graphics::Rect>()
        .ok_or_else(|| ggez::GameError::CustomError("Could not unpack boundaries.".to_owned()))?;
    let sprites = resources
        .get::<SpritePool>()
        .ok_or_else(|| ggez::GameError::CustomError("Could not unpack sprite pool.".to_owned()))?;
    world.push((
        components::Position::new(random::<f32>() * boundaries.w + boundaries.x, -20.),
        components::Velocity::new(45., 25.),
        components::BoundaryCollision::new(true, false, true),
        sprites.init_sprite(
            "/sprites/enemies/skeleton_basic_16_16.png",
            Duration::from_secs_f32(0.20),
        )?,
        components::Aura::new(256., |act| {
            if let components::GameAction::Move { delta } = act {
                // speed up nearby allies by 50%
                components::GameAction::Move { delta: delta * 1.5 }
            } else {
                act
            }
        }),
        components::Enemy::new(1, 15),
        components::Health::new(3),
        components::Collision::new_basic(64., 64.),
    ));
    Ok(())
}

pub fn spawn_loot_skeleton(world: &mut World, resources: &mut Resources) -> Result<(), GameError> {
    // get boundaries for random x
    let boundaries = *resources
        .get::<ggez::graphics::Rect>()
        .ok_or_else(|| ggez::GameError::CustomError("Could not unpack boundaries.".to_owned()))?;
    let sprites = resources
        .get::<SpritePool>()
        .ok_or_else(|| ggez::GameError::CustomError("Could not unpack sprite pool.".to_owned()))?;
    world.push((
        components::Position::new(random::<f32>() * boundaries.w + boundaries.x, 30.),
        components::Velocity::new(75., 0.),
        components::BoundaryCollision::new(true, false, true),
        sprites.init_sprite(
            "/sprites/enemies/skeleton_sword_16_16.png",
            Duration::from_secs_f32(0.20),
        )?,
        components::Enemy::new(0, 100),
        components::Health::new(8),
        components::LifeDuration::new(Duration::from_secs(15)),
        components::Collision::new_basic(64., 64.),
    ));
    Ok(())
}

pub fn spawn_tank_skeleton(world: &mut World, resources: &mut Resources) -> Result<(), GameError> {
    // get boundaries for random x
    let boundaries = *resources
        .get::<ggez::graphics::Rect>()
        .ok_or_else(|| ggez::GameError::CustomError("Could not unpack boundaries.".to_owned()))?;
    let sprites = resources
        .get::<SpritePool>()
        .ok_or_else(|| ggez::GameError::CustomError("Could not unpack sprite pool.".to_owned()))?;
    world.push((
        components::Position::new(random::<f32>() * boundaries.w + boundaries.x, -20.),
        components::Velocity::new(0., 15.),
        sprites.init_sprite(
            "/sprites/enemies/skeleton_sword_16_16.png",
            Duration::from_secs_f32(0.25),
        )?,
        components::Aura::new(192., |act| {
            if let components::GameAction::TakeDamage { dmg } = act {
                // reduce dmg by 1, but not below 1, unless it was already at 0
                components::GameAction::TakeDamage {
                    dmg: (1.min(dmg)).max(dmg - 1),
                }
            } else {
                act
            }
        }),
        components::Enemy::new(2, 25),
        components::Health::new(3),
        components::Collision::new_basic(64., 64.),
    ));
    Ok(())
}
