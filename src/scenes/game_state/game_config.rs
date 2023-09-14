#[derive(Clone, Debug)]
pub struct GameConfig {
    // --- Director Config ---
    /// The base amounts of credits per second
    pub base_credits: f32,
    /// The amount of additional credits granted each second per round passed.
    pub wave_credits: f32,
    /// The initial enemies
    pub wave_enemies: [usize; super::director::WAVE_SIZE],

    // --- Spell Config ---
    pub base_spells: [usize; 4],
    pub base_slots: usize,

    // --- Game Data Config ---
    pub starting_gold: i32,
    pub starting_city_health: i32,
    pub starting_wave: u32,

    // --- Other ---
    pub base_speed: f32,

    pub tutorial: bool,

    pub achievements_unlocked: super::achievements::AchievementProgressSource,

    pub initial_camera_offset: f32,
}

impl GameConfig {
    /// Constructs a default game config for debugging the game
    pub fn debug() -> Self {
        Self {
            base_credits: 50.,
            wave_credits: 6.,
            wave_enemies: [14, 14, 14, 14],
            base_spells: [1, 7, 13, 14],
            base_slots: 6,
            base_speed: 150.,
            starting_gold: 10000,
            starting_city_health: 10,
            starting_wave: 1,
            tutorial: true,
            achievements_unlocked: super::achievements::AchievementProgressSource::Percentage(1.),
            initial_camera_offset: 0.,
        }
    }
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            base_credits: 15.,
            wave_credits: 6.,
            wave_enemies: [0, 0, 1, 1],
            base_spells: [1, 7, 0, 0],
            base_slots: 4,
            base_speed: 150.,
            starting_gold: 0,
            starting_city_health: 10,
            starting_wave: 1,
            tutorial: true,
            achievements_unlocked: super::achievements::AchievementProgressSource::Cache,
            initial_camera_offset: 1500.,
        }
    }
}
