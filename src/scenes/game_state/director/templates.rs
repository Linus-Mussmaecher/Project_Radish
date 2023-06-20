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
            "Runner",
            "A nimble enemy that walks faster and also sideways, but has less health than the basic skeleton.",
            070,
            super::spawners::spawn_fast_skeleton,
        ),
        // Dodge skeleton
        EnemyTemplate::new(
            sprite_pool.init_sprite("/sprites/enemies/skeleton_sword", Duration::ZERO)?,
            "Dodger",
            "A nimble enemy that walks faster and slightly sideways, but has less health than the basic skeleton. It also does a short sprint from time to time.",
            080,
            super::spawners::spawn_dodge_skeleton,
        ),
        // Jump skeleton
        EnemyTemplate::new(
            sprite_pool.init_sprite("/sprites/enemies/skeleton_jump", Duration::ZERO)?,
            "Bone Jumper",
            "A nimble enemy that rapidly jumps sideways when taking damage.",
            080,
            super::spawners::spawn_jump_skeleton,
        ),
        // Dynamite skeleton
        EnemyTemplate::new(
            sprite_pool.init_sprite("/sprites/enemies/skeleton_dynamite", Duration::ZERO)?,
            "Dynamite Carrier",
            "A basic skeleton with extra health and a bomb strapped to its back. Deals high damage to your city, but explodes on death.",
            090,
            super::spawners::spawn_dynamite_skeleton,
        ),
        // catapult
        EnemyTemplate::new(
            sprite_pool.init_sprite("/sprites/enemies/catapult", Duration::ZERO)?,
            "Catapult",
            "A stationary siege weapons that regularly grabs nearby allies and catapults them towards the city.",
            110,
            super::spawners::spawn_catapult,
        ),
        // loot
        EnemyTemplate::new(
            sprite_pool.init_sprite("/sprites/enemies/skeleton_loot", Duration::ZERO)?,
            "Loot Goblin",
            "An enemy that doesn't threaten your city but lingers at a fixed distance, disappearing after a time. Drops large amounts of gold on death.",
            180,
            super::spawners::spawn_loot_skeleton,
        ),
        // catapult
        
        
        // Tanky skeleton
        EnemyTemplate::new(
            sprite_pool.init_sprite("/sprites/enemies/skeleton_tank", Duration::ZERO)?,
            "Guardian",
            "An enemy that walks slowly, but reduces damage taken on nearby allies and heals them on death.",
            100,
            super::spawners::spawn_tank_skeleton,
        ),
        // Speed-up skeleton
        EnemyTemplate::new(
            sprite_pool.init_sprite("/sprites/enemies/skeleton_flag", Duration::ZERO)?,
            "Bannerman",
            "A skeleton with decent speed and suvivability. Speeds up nearby allies, with a huge speed bost on death.",
            110,
            super::spawners::spawn_charge_skeleton,
        ),
        // Wizard
        EnemyTemplate::new(
            sprite_pool.init_sprite("/sprites/enemies/skeleton_wizard", Duration::ZERO)?,
            "Lightning Wizard",
            "A slow and flimsy enemy that regularly speeds up nearby allies and heals them.",
            150,
            super::spawners::spawn_wizard_skeleton,
        ),
        // Wizard 2
        EnemyTemplate::new(
            sprite_pool.init_sprite("/sprites/enemies/skeleton_wizard2", Duration::ZERO)?,
            "Earth Wizard",
            "A slow and flimsy enemy that regularly gives nearby allies a damage reducing shield and heals them.",
            150,
            super::spawners::spawn_wizard_skeleton2,
        ),
        // Wizard 3
        EnemyTemplate::new(
            sprite_pool.init_sprite("/sprites/enemies/skeleton_wizard3", Duration::ZERO)?,
            "Necromancer",
            "A slow enemy that ressurects additional skeletons and can damage groups of enemies to increase their speed.",
            170,
            super::spawners::spawn_wizard_skeleton3,
        ),
        // Splitter
        EnemyTemplate::new(
            sprite_pool.init_sprite("/sprites/enemies/golem", Duration::ZERO)?,
            "Bone Golem",
            "A slow and tanky enemy that spawns multiple smaller skeletons on death.",
            200,
            super::spawners::spawn_splitter,
        ),
        // getting-faster
        EnemyTemplate::new(
            sprite_pool.init_sprite("/sprites/enemies/ghost", Duration::ZERO)?,
            "Ghost",
            "A slow enemy that, whenever it takes damage, speeds up and becomes temporarily immune.",
            200,
            super::spawners::spawn_ghost,
        ),
        // distributing damage
        EnemyTemplate::new(
            sprite_pool.init_sprite("/sprites/enemies/armor", Duration::ZERO)?,
            "Animated Armor",
            "A high-health creature that distributes damage taken amongst nearby allies and heals them on death.",
            200,
            super::spawners::spawn_animated_armor,
        ),
        // periodic bracing
        EnemyTemplate::new(
            sprite_pool.init_sprite("/sprites/enemies/legionnaire", Duration::ZERO)?,
            "Legionnaire",
            "A tanky enemy that periodically braces itself, slowing down while gaining high damage reduction.",
            220,
            super::spawners::spawn_legionnaire,
        ),
    ])
}
