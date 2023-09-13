use std::time::Duration;

use good_web_game::GameError;
use mooeye;

use super::EnemyDescriptor;

/// Generates all standard enemy templates.
pub(super) fn generate_descriptors(
    sprite_pool: &mut mooeye::sprite::SpritePool,
    ctx: &mut good_web_game::Context,
    gfx_ctx: &mut good_web_game::event::GraphicsContext,
) -> Result<Vec<EnemyDescriptor>, GameError> {
    Ok(vec![
        // Basic skeleton
        EnemyDescriptor::new(
            sprite_pool.init_sprite_fmt("./sprites/enemies/skeleton_basic_16_16.png", ctx, gfx_ctx, Duration::ZERO)?,
            "Skeleton",
            "A basic enemy. Neither strong nor fast, but expect a lot of them.",
            40,
            super::spawners::spawn_basic_skeleton,
        ),
        // Fast skeleton
        EnemyDescriptor::new(
            sprite_pool.init_sprite_fmt("./sprites/enemies/skeleton_sword_16_16.png", ctx, gfx_ctx, Duration::ZERO)?,
            "Runner",
            "A nimble enemy that walks faster and also sideways, but has less health than the basic skeleton.",
            70,
            super::spawners::spawn_fast_skeleton,
        ),
        // Dodge skeleton
        EnemyDescriptor::new(
            sprite_pool.init_sprite_fmt("./sprites/enemies/skeleton_sword_16_16.png", ctx, gfx_ctx, Duration::ZERO)?,
            "Dodger",
            "A nimble enemy that walks faster and slightly sideways, but has less health than the basic skeleton. It also does a short sprint from time to time.",
            80,
            super::spawners::spawn_dodge_skeleton,
        ),
        // Jump skeleton
        EnemyDescriptor::new(
            sprite_pool.init_sprite_fmt("./sprites/enemies/skeleton_jump_16_16.png", ctx, gfx_ctx, Duration::ZERO)?,
            "Bone Jumper",
            "A nimble enemy that rapidly jumps sideways when taking damage.",
            80,
            super::spawners::spawn_jump_skeleton,
        ),
        // Dynamite skeleton
        EnemyDescriptor::new(
            sprite_pool.init_sprite_fmt("./sprites/enemies/skeleton_dynamite_16_16.png", ctx, gfx_ctx, Duration::ZERO)?,
            "Dynamite Carrier",
            "A basic skeleton with extra health and a bomb strapped to its back. Deals high damage to your city, but explodes on death.",
            90,
            super::spawners::spawn_dynamite_skeleton,
        ),
        // catapult
        EnemyDescriptor::new(
            sprite_pool.init_sprite_fmt("./sprites/enemies/catapult_16_16.png", ctx, gfx_ctx, Duration::ZERO)?,
            "Catapult",
            "A stationary siege weapons that regularly grabs nearby allies and catapults them towards the city.",
            110,
            super::spawners::spawn_catapult,
        ),
        // loot
        EnemyDescriptor::new(
            sprite_pool.init_sprite_fmt("./sprites/enemies/skeleton_loot_16_16.png", ctx, gfx_ctx, Duration::ZERO)?,
            "Loot Goblin",
            "An enemy that doesn't threaten your city but lingers at a fixed distance, disappearing after a time. Drops large amounts of gold on death.",
            180,
            super::spawners::spawn_loot_skeleton,
        ),
        // Tanky skeleton
        EnemyDescriptor::new(
            sprite_pool.init_sprite_fmt("./sprites/enemies/skeleton_tank_16_16.png", ctx, gfx_ctx, Duration::ZERO)?,
            "Guardian",
            "An enemy that walks slowly, but reduces damage taken on nearby allies and heals them on death.",
            100,
            super::spawners::spawn_tank_skeleton,
        ),
        // Speed-up skeleton
        EnemyDescriptor::new(
            sprite_pool.init_sprite_fmt("./sprites/enemies/skeleton_flag_16_16.png", ctx, gfx_ctx, Duration::ZERO)?,
            "Bannerman",
            "A skeleton with decent speed and suvivability. Speeds up nearby allies, with a huge speed bost on death.",
            110,
            super::spawners::spawn_charge_skeleton,
        ),
        // Wizard
        EnemyDescriptor::new(
            sprite_pool.init_sprite_fmt("./sprites/enemies/skeleton_wizard_16_16.png", ctx, gfx_ctx, Duration::ZERO)?,
            "Lightning Wizard",
            "A slow and flimsy enemy that regularly speeds up nearby allies and heals them.",
            150,
            super::spawners::spawn_wizard_skeleton,
        ),
        // Wizard 2
        EnemyDescriptor::new(
            sprite_pool.init_sprite_fmt("./sprites/enemies/skeleton_wizard2_16_16.png", ctx, gfx_ctx, Duration::ZERO)?,
            "Earth Wizard",
            "A slow and flimsy enemy that regularly gives nearby allies a damage reducing shield and heals them.",
            150,
            super::spawners::spawn_wizard_skeleton2,
        ),
        // Wizard 3
        EnemyDescriptor::new(
            sprite_pool.init_sprite_fmt("./sprites/enemies/skeleton_wizard3_16_16.png", ctx, gfx_ctx, Duration::ZERO)?,
            "Necromancer",
            "A slow enemy that ressurects additional skeletons and can damage groups of enemies to increase their speed.",
            170,
            super::spawners::spawn_wizard_skeleton3,
        ),
        // Splitter
        EnemyDescriptor::new(
            sprite_pool.init_sprite_fmt("./sprites/enemies/golem_16_16.png", ctx, gfx_ctx, Duration::ZERO)?,
            "Bone Golem",
            "A slow and tanky enemy that spawns multiple smaller skeletons on death.",
            200,
            super::spawners::spawn_splitter,
        ),
        // getting-faster
        EnemyDescriptor::new(
            sprite_pool.init_sprite_fmt("./sprites/enemies/ghost_16_16.png", ctx, gfx_ctx, Duration::ZERO)?,
            "Ghost",
            "A slow enemy that, whenever it takes damage, speeds up and becomes temporarily immune.",
            200,
            super::spawners::spawn_ghost,
        ),
        // distributing damage
        EnemyDescriptor::new(
            sprite_pool.init_sprite_fmt("./sprites/enemies/armor_16_16.png", ctx, gfx_ctx, Duration::ZERO)?,
            "Animated Armor",
            "A high-health creature that distributes damage taken amongst nearby allies and heals them on death.",
            200,
            super::spawners::spawn_animated_armor,
        ),
        // periodic bracing
        EnemyDescriptor::new(
            sprite_pool.init_sprite_fmt("./sprites/enemies/legionnaire_16_16.png", ctx, gfx_ctx, Duration::ZERO)?,
            "Legionnaire",
            "A tanky enemy that periodically braces itself, slowing down while gaining high damage reduction.",
            220,
            super::spawners::spawn_legionnaire,
        ),
    ])
}
