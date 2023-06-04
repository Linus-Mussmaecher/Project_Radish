use std::{fmt::Debug, time::Duration};

use super::*;
use ggez::{graphics, GameError};
use legion::{system, systems::CommandBuffer};
use mooeye::sprite;
use rand::random;

/// The maximum amount of different enemy templates per wave
pub(super) const WAVE_SIZE: usize = 4;

mod spawners;
mod templates;
/// The director struct is responsible for spawning waves of enemies.
/// A director regularly earns credit points and spends them on units from a customizable enemy set until a wave threshhold is reached.
/// Then, the director rerolls the enemy pool and starts a new wave.
#[derive(Clone)]
pub struct Director {
    // --- ONGOING VALUES ---
    /// The current wave number.
    wave: u32,
    /// The current state of the director.
    state: DirectorState,

    /// The time since the director last spawned units.
    intervall: Duration,
    /// The current amount of credits this director can spend.
    credits: u32,

    /// The enemies
    wave_enemies: [usize; WAVE_SIZE],
    /// The enemy posse the director can select spawns from, containing their costs and a spawning function pointer.
    enemies: Vec<EnemyTemplate>,

    // --- CONFIGURATION ---
    /// The base amounts of credits per second
    base_credits: f32,
    /// The amount of additional credits granted each second per round passed.
    wave_credits: f32,
}

impl Director {
    /// Spawns a new director with default parameters.
    pub fn new(sprite_pool: &sprite::SpritePool, config: &super::GameConfig) -> Self {
        Self {
            wave: 1,
            state: DirectorState::Spawning(450),

            intervall: Duration::ZERO,
            credits: 0,

            wave_enemies: config.wave_enemies,
            enemies: templates::generate_templates(sprite_pool).unwrap_or_default(),

            base_credits: config.base_credits,
            wave_credits: config.wave_credits,
        }
    }

    /// Returns the current wave.
    pub fn get_wave(&self) -> u32 {
        self.wave
    }

    /// Returns a reference with the current wave's enemies
    pub fn get_enemies(&self) -> [&EnemyTemplate; WAVE_SIZE] {
        self.wave_enemies.map(|i| &self.enemies[i])
    }

    /// If currently in the last [DirectorState] of a wave cycle, reset to the first one, increase the wave number
    /// and grant a wave_pool for that next wave.
    pub fn next_wave(&mut self) {
        if self.state == DirectorState::WaitingForMenu {
            self.wave += 1;
            self.state = DirectorState::Spawning(
                30 * (self.base_credits + 2. * self.wave_credits * self.wave as f32) as u32,
            );
        }
    }

    pub fn reroll_wave_enemies(&mut self) {
        // get 4 random indices of enemies
        for i in 0..WAVE_SIZE {
            self.wave_enemies[i] = rand::random::<usize>() % self.enemies.len();
        }
        // sort the wave_enemies array
        self.wave_enemies.sort();
    }
}

/// A system that handles the directors interaction with the game world.
/// This increases the director credits and spends them, handles unit spawning and sends messages to initialize new waves.
#[system]
pub fn direct(
    subworld: &mut legion::world::SubWorld,
    enemy_query: &mut legion::Query<&components::Enemy>,
    cmd: &mut CommandBuffer,
    #[resource] boundaries: &graphics::Rect,
    #[resource] director: &mut Director,
    #[resource] ix: &controller::Interactions,
    #[resource] messages: &mut MessageSet,
) {
    // add time since last frame to counters

    director.intervall += ix.delta;

    match director.state {
        DirectorState::Spawning(wave_pool) => {
            // only spawn in 1-second intervalls
            if director.intervall >= Duration::from_secs(1) {
                // grant credits
                director.credits +=
                    (director.base_credits + director.wave_credits * director.wave as f32) as u32;
                // reset intervall
                director.intervall = Duration::ZERO;

                // randomly select an amount of available credits to spend
                let mut to_spend = (random::<f32>().powi(2) * director.credits as f32) as u32;

                // while credits left to spend
                'outer: loop {
                    // select a random enemy type
                    let mut enemy_ind = random::<usize>() % director.wave_enemies.len();
                    let mut enemy = director.enemies.get(director.wave_enemies[enemy_ind]);

                    // downgrade spawn until affordable
                    while match enemy {
                        Some(enemy_template) => enemy_template.cost > to_spend,
                        None => true,
                    } {
                        // if no downgrade possible, end this spending round
                        if enemy_ind == 0 {
                            break 'outer;
                        }
                        // otherwise, downgrade and try next enemy
                        enemy_ind -= 1;
                        enemy = director.enemies.get(director.wave_enemies[enemy_ind]);
                    }

                    // unpack enemy
                    if let Some(enemy_template) = enemy {
                        // spawn
                        if (enemy_template.spawner._spawner)(
                            cmd,
                            ggez::glam::Vec2::new(rand::random::<f32>() * boundaries.w, -20.),
                        )
                        .is_ok()
                        {
                            // if spawning threw no error, reduce available credits
                            to_spend -= enemy_template.cost;
                            director.credits -= enemy_template.cost;

                            // possible switch state on every spawn
                            let left = wave_pool.saturating_sub(enemy_template.cost);
                            director.state = if left > 0 {
                                DirectorState::Spawning(left)
                            } else {
                                DirectorState::WaitingForDead
                            }
                        }
                    }
                }
            }
        }
        DirectorState::WaitingForDead => {
            if enemy_query.iter(subworld).count() == 0 {
                messages.insert(mooeye::UiMessage::Extern(GameMessage::NextWave(
                    director.wave as i32 + 1,
                )));
                director.reroll_wave_enemies();
                director.state = DirectorState::WaitingForMenu
            }
        }
        DirectorState::WaitingForMenu => {}
    }
}

/// The state of a [Director].
/// States should be used only in sequence.
/// One rotation = one wave.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum DirectorState {
    /// The director is currently spawning enemies.
    /// The payload is the wave_pool left to spawn until this wave ends.
    Spawning(u32),
    /// The director has emptied its wave pool and is waiting for all spawned enemies to be removed.
    WaitingForDead,
    /// All enemies have despawned and the director has notified the player of the end of the wave.
    /// The director is waiting for the player to init the next wave.
    WaitingForMenu,
}

#[derive(Debug, Clone)]
/// A template for spawning an enemy. Also contains descriptions and icon to display in wave menu.
pub struct EnemyTemplate {
    /// The icon of this enemy, to be displayed in the wave menu.
    pub icon: sprite::Sprite,
    /// The name of the enemy.
    pub name: String,
    /// A short description of the enemies abilities.
    pub description: String,
    /// The spawning cost of the enemy, determining its frequency.
    cost: u32,
    /// A wrapped function pointer to spawn the enemy.
    spawner: EnemySpawner,
}

impl EnemyTemplate {
    // Creates a new enemy template.
    pub fn new(
        icon: sprite::Sprite,
        name: &str,
        description: &str,
        cost: u32,
        spawner: EnemySpawnFunction,
    ) -> Self {
        Self {
            icon: icon,
            name: name.to_owned(),
            description: description.to_owned(),
            cost,
            spawner: EnemySpawner { _spawner: spawner },
        }
    }
}

/// Contains a function pointer to spawn an enemy
#[derive(Clone)]
struct EnemySpawner {
    _spawner: EnemySpawnFunction,
}

impl Debug for EnemySpawner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EnemySpawner").finish()
    }
}

type EnemySpawnFunction =
    fn(&mut CommandBuffer, components::Position) -> Result<(), GameError>;
