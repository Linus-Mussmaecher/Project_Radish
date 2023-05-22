use std::time::Duration;

use ggez::GameError;
use mooeye;

use super::EnemyTemplate;

/// Generates all standard enemy templates.
pub(super) fn generate_templates(
    sprite_pool: &mooeye::sprite::SpritePool,
) -> Result<Vec<EnemyTemplate>, GameError> {
    Ok(vec![
        // Basic skeleton
        EnemyTemplate::new(
            sprite_pool.init_sprite("/sprites/enemies/skeleton_basic", Duration::ZERO)?,
            "Skeleton",
            "A basic enemy. Neither strong nor fast, but expect a lot of them.",
            040,
            super::spawners::spawn_basic_skeleton,
        ),
        // Fast skeleton
        EnemyTemplate::new(
            sprite_pool.init_sprite("/sprites/enemies/skeleton_sword", Duration::ZERO)?,
            "Skeleton Runner",
            "A nimble enemy that walks faster and also sideways, but has less health than the basic skeleton.",
            070,
            super::spawners::spawn_fast_skeleton,
        ),
        // Tanky skeleton
        EnemyTemplate::new(
            sprite_pool.init_sprite("/sprites/enemies/skeleton_tank", Duration::ZERO)?,
            "Skeleton Guardian",
            "An enemy that walks slows, but reduces damage taken on nearby allies and heals them on death.",
            100,
            super::spawners::spawn_tank_skeleton,
        ),
        // Speed-up skeleton
        EnemyTemplate::new(
            sprite_pool.init_sprite("/sprites/enemies/skeleton_flag", Duration::ZERO)?,
            "Skeleton Bannerman",
            "A skeleton with decent speed and suvivability. Speeds up nearby allies, with a huge speed bost on death.",
            110,
            super::spawners::spawn_charge_skeleton,
        ),
        // Wizard
        EnemyTemplate::new(
            sprite_pool.init_sprite("/sprites/enemies/skeleton_wizard", Duration::ZERO)?,
            "Skeleton Wizard",
            "A slow and flimsy enemy that regularly speeds up nearby allies and heals them.",
            150,
            super::spawners::spawn_wizard_skeleton,
        ),
        // Splitter
        EnemyTemplate::new(
            sprite_pool.init_sprite("/sprites/enemies/golem", Duration::ZERO)?,
            "Bone Golem",
            "A slow and tanky enemy that spawns multiple smaller skeletons on death.",
            200,
            super::spawners::spawn_splitter,
        ),
        // loot
        EnemyTemplate::new(
            sprite_pool.init_sprite("/sprites/enemies/skeleton_loot", Duration::ZERO)?,
            "Loot goblin",
            "An enemy that doesn't threaten your city but lingers at a fixed height, disappearing after a time. Drops huge amounts of gold on death.",
            180,
            super::spawners::spawn_loot_skeleton,
        ),
        // getting-faster
        EnemyTemplate::new(
            sprite_pool.init_sprite("/sprites/enemies/ghost", Duration::ZERO)?,
            "Ghost",
            "A slow enemy that, whenever it takes damage, speeds up and becomes temporarily immune.",
            200,
            super::spawners::spawn_ghost,
        ),

    ])
}
