use std::time::Duration;

use ggez::{Context, GameError};
use legion::{Resources, World};
use rand::random;

use mooeye::sprite::SpritePool;

use super::{
    components::{self, actions::*, graphics::Particle},
    game_message::MessageSet,
};

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
                (040, &spawn_basic_skeleton),
                (070, &spawn_fast_skeleton),
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
            self.credits += 25 + self.total.as_secs() / 5;
            // reset intervall
            self.intervall = Duration::ZERO;

            // randomly select an amount of available credits to spend
            let mut to_spend = (random::<f32>().powi(2) * self.credits as f32) as u64;
            if to_spend >= 40 {
                println!("Spending {} of {} credits.", to_spend, self.credits);
            }

            // while credits left to spend
            'outer: loop {
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
        components::Velocity::new(0., 10.),
        components::Graphics::from(sprites.init_sprite(
            "/sprites/enemies/skeleton_basic",
            Duration::from_secs_f32(0.25),
        )?),
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
        components::Velocity::new(35., 15.),
        components::BoundaryCollision::new(true, false, true),
        components::Graphics::from(sprites.init_sprite(
            "/sprites/enemies/skeleton_basic",
            Duration::from_secs_f32(0.20),
        )?),
        components::Aura::new(
            256.,
            |act| {
                match act {
                    // speed up nearby allies by 50%
                    GameAction::Move { delta } => GameAction::Move { delta: delta * 1.5 },
                    other => other,
                }
            },
            |_| true,
        ),
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
        components::Velocity::new(50., 0.),
        components::BoundaryCollision::new(true, false, true),
        components::Graphics::from(sprites.init_sprite(
            "/sprites/enemies/skeleton_sword",
            Duration::from_secs_f32(0.20),
        )?),
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
        components::Velocity::new(0., 10.),
        components::Graphics::from(sprites.init_sprite(
            "/sprites/enemies/skeleton_sword",
            Duration::from_secs_f32(0.25),
        )?),
        components::Aura::new(
            192.,
            |act| {
                match act {
                    // reduce dmg by 1, but if would be reduced to 0, onyl 20% chance to do so
                    GameAction::TakeDamage { dmg } => GameAction::TakeDamage {
                        dmg: if dmg == 1 {
                            if random::<f32>() < 0.8 {
                                1
                            } else {
                                0
                            }
                        } else {
                            0.max(dmg - 1)
                        },
                    },
                    other => other,
                }
            },
            |_| true,
        ),
        components::OnDeath::new(
            GameActionContainer::single(
                Distributor::new(gameaction_multiple![
                    GameAction::TakeHealing { heal: 2 },
                    GameAction::AddParticle(
                        Particle::new(
                            sprites.init_sprite("/sprites/heal", Duration::from_secs_f32(0.25))?,
                        )
                        .with_duration(Duration::from_secs(1))
                        .with_velocity(0., -15.)
                        .with_relative_position(0., -64.),
                    ),
                ])
                .with_enemies_only()
                .with_limit(5)
                .with_range(256.)
                .to_action(),
            ),
            MessageSet::new(),
        ),
        components::Enemy::new(2, 25),
        components::Health::new(3),
        components::Collision::new_basic(64., 64.),
    ));
    Ok(())
}
