use std::time::Duration;

use ggez::{
    glam::Vec2,
    graphics::{self, Canvas, DrawParam, MeshBuilder, Rect},
    Context,
};
use mooeye::sprite;

use legion::{system, IntoQuery};
use tinyvec::TinyVec;

use crate::PALETTE;

use super::{actions::GameAction, Health, Position, Velocity};

pub const PIXEL_SIZE: f32 = 4.;

#[derive(Debug, Clone)]
/// The graphics component of an entity, containing a sprite to be drawn to the screen and a container for multiple additional particles.
pub struct Graphics {
    /// The main sprite to represent this object.
    sprite: SpriteWrapper,
    /// Container for particles added to this and managed by this object.
    particles: TinyVec<[Particle; 4]>,
}

impl Graphics {
    pub fn new(path: impl AsRef<std::path::Path>, frame_time: Duration) -> Self {
        Self {
            sprite: SpriteWrapper::PreInit(path.as_ref().to_string_lossy().to_string(), {
                let mut sprite = sprite::Sprite::default();
                sprite.set_frame_time(frame_time);
                sprite
            }),
            particles: TinyVec::new(),
        }
    }

    /// Sets the variant of the underlying sprite and returns the graphics component builder-pattern style.
    pub fn with_sprite_variant(mut self, variant: u32) -> Self {
        match &mut self.sprite {
            SpriteWrapper::PreInit(_, pre_init) => {
                pre_init.set_variant(variant);
            }
            SpriteWrapper::Initialized(sprite) => {
                sprite.set_variant(variant);
            }
        }
        self
    }

    /// Returns the objects size in the world, already multiplied by PIXEL_SIZE.
    pub fn get_size(&self) -> (f32, f32) {
        (
            self.get_sprite().get_dimensions().0 * PIXEL_SIZE,
            self.get_sprite().get_dimensions().1 * PIXEL_SIZE,
        )
    }

    /// Returns a reference to this graphic's sprite, or a default sprite if it is not yet initialized.
    pub fn get_sprite(&self) -> &sprite::Sprite {
        match &self.sprite {
            SpriteWrapper::PreInit(_, pre_init) => pre_init,
            SpriteWrapper::Initialized(sprite) => sprite,
        }
    }

    /// Returns a mutable reference to this graphic's sprite, or a default sprite if it is not yet initialized.
    pub fn get_sprite_mut(&mut self) -> &mut sprite::Sprite {
        match &mut self.sprite {
            SpriteWrapper::PreInit(_, pre_init) => pre_init,
            SpriteWrapper::Initialized(sprite) => sprite,
        }
    }
}

impl From<sprite::Sprite> for Graphics {
    fn from(value: sprite::Sprite) -> Self {
        Self {
            sprite: SpriteWrapper::Initialized(value),
            particles: TinyVec::new(),
        }
    }
}

#[derive(Debug, Clone)]
/// A wrapper that represents a sprite as held in a graphics component.
/// The sprite can either be initialized (as a basic sprite) or just be a path and a default sprite that need to later be initialized via a sprite pool.
enum SpriteWrapper {
    PreInit(String, sprite::Sprite),
    Initialized(sprite::Sprite),
}

impl SpriteWrapper {
    /// Checks if the sprite wrapper needs to be initialized and does so.
    fn init(
        &mut self,
        ctx: &ggez::Context,
        sprite_pool: &mut sprite::SpritePool,
    ) -> Result<&mut sprite::Sprite, ggez::GameError> {
        if let Self::PreInit(path, pre_init) = self {
            let mut sprite = sprite_pool.init_sprite_lazy(ctx, path, pre_init.get_frame_time())?;
            sprite.set_variant(pre_init.get_variant());
            *self = Self::Initialized(sprite);
        }
        Ok(match self {
            SpriteWrapper::PreInit(_, _) => panic!("Should have been initialized already."),
            SpriteWrapper::Initialized(sprite) => sprite,
        })
    }
}

impl Default for SpriteWrapper {
    fn default() -> Self {
        Self::Initialized(sprite::Sprite::default())
    }
}

/// Draws all the sprites in the world to their respective positions on the canvas.
pub fn draw_sprites(
    world: &mut legion::World,
    resources: &mut legion::Resources,
    ctx: &Context,
    canvas: &mut Canvas,
    animate: bool,
    camera_offset: &mut (f32, f32),
) -> Result<(), ggez::GameError> {
    // get boundaries for relative moving
    let boundaries = *resources
        .get::<Rect>()
        .ok_or_else(|| ggez::GameError::CustomError("Could not unpack boundaries.".to_owned()))?;
    let (screen_w, screen_h) = ctx.gfx.drawable_size();

    camera_offset.0 = 0_f32.max(camera_offset.0 - 256. * ctx.time.delta().as_secs_f32());

    // get sprite pool for inits
    let mut sprite_pool = resources
        .get_mut::<sprite::SpritePool>()
        .ok_or_else(|| ggez::GameError::CustomError("Could not unpack boundaries.".to_owned()))?;

    for (pos, gfx, vel, health) in
        <(&Position, &mut Graphics, Option<&Velocity>, Option<&Health>)>::query().iter_mut(world)
    {
        // get sprite
        let sprite = gfx.sprite.init(ctx, &mut sprite_pool)?;

        // get factors/position for image mirrogin
        let factor = if match vel {
            Some(v) => v.get_dx() < 0.,
            None => false,
        } {
            -1.
        } else {
            1.
        };

        // position within the world
        let n_pos = *pos
            // move as the world is positioned on screen
            + Vec2::new(
                ((screen_w - boundaries.w)/2.).floor(),
                ((screen_h - boundaries.h)/2. + camera_offset.0).floor(),
            )
            // move to draw to correct position based on flip
            + Vec2::new(
                -sprite.get_dimensions().0 * PIXEL_SIZE / 2. * factor,
                -sprite.get_dimensions().1 * PIXEL_SIZE / 2.,
            );

        // draw the sprite
        if animate {
            sprite.draw_sprite(
                ctx,
                canvas,
                DrawParam::default()
                    .dest(n_pos)
                    .scale(Vec2::new(PIXEL_SIZE * factor, PIXEL_SIZE)),
            );
        } else {
            graphics::Drawable::draw(
                sprite,
                canvas,
                DrawParam::default()
                    .dest(n_pos)
                    .scale(Vec2::new(PIXEL_SIZE * factor, PIXEL_SIZE)),
            );
        }

        // draw the health bar
        if let Some(health) = health {
            let mut bar = Rect::new(
                n_pos.x
                    - if factor < 1. {
                        sprite.get_dimensions().0 * PIXEL_SIZE
                    } else {
                        0.
                    },
                n_pos.y - 5. * PIXEL_SIZE,
                sprite.get_dimensions().0 * PIXEL_SIZE,
                4. * PIXEL_SIZE,
            );

            let mut health_bar_builder = MeshBuilder::new();
            // border
            health_bar_builder.rectangle(
                graphics::DrawMode::fill(),
                bar,
                graphics::Color::from_rgb_u32(PALETTE[15]),
            )?;
            bar.x += PIXEL_SIZE;
            bar.y += PIXEL_SIZE;
            bar.w -= 2. * PIXEL_SIZE;
            bar.h -= 2. * PIXEL_SIZE;

            // background
            health_bar_builder.rectangle(
                graphics::DrawMode::fill(),
                bar,
                graphics::Color::from_rgb_u32(PALETTE[14]),
            )?;
            let w = bar.w;
            //snapshot bar
            bar.w = PIXEL_SIZE
                * (w * health.get_snapshot() / health.get_max_health() as f32 / PIXEL_SIZE).floor();
            health_bar_builder.rectangle(
                graphics::DrawMode::fill(),
                bar,
                graphics::Color::from_rgb_u32(PALETTE[12]),
            )?;

            // health bar
            bar.w = PIXEL_SIZE
                * (w * health.get_current_health() as f32
                    / health.get_max_health() as f32
                    / PIXEL_SIZE)
                    .floor();
            health_bar_builder.rectangle(
                graphics::DrawMode::fill(),
                bar,
                graphics::Color::from_rgb_u32(PALETTE[6]),
            )?;

            canvas.draw(
                &graphics::Mesh::from_data(ctx, health_bar_builder.build()),
                DrawParam::default(),
            );
        }

        // draw the sprites particles

        for part in gfx.particles.iter_mut() {
            let part_sprite = part.sprite.init(ctx, &mut sprite_pool)?;
            part_sprite.draw_sprite(
                ctx,
                canvas,
                DrawParam::default()
                    .dest(
                        *pos + part.rel_pos
                            - Vec2::from(part_sprite.get_dimensions()) * PIXEL_SIZE / 2.
                            + Vec2::new(
                                (screen_w - boundaries.w) / 2.,
                                (screen_h - boundaries.h) / 2.,
                            ),
                    )
                    .scale(Vec2::new(PIXEL_SIZE, PIXEL_SIZE)),
            );
        }
    }

    Ok(())
}

#[derive(Debug, Clone, Default)]
/// A struct that represents a Particle that can be added to a graphics component to be displayed on top of the main sprite.
pub struct Particle {
    /// The drawable to be displayed.
    sprite: SpriteWrapper,
    /// Relative position of the sprites center to the center of the main sprite.
    rel_pos: Vec2,
    /// Velocity this particle moves at (in pixels/s).
    vel: Vec2,
    /// The remaining duration of this particle. If None, it will stay indefinitely.
    duration: Option<Duration>,
}

impl Particle {
    /// Creates a new particle with the passed sprite, infinite duration and no velocity or offset.
    pub fn new(path: impl AsRef<std::path::Path>, frame_time: Duration) -> Self {
        Self {
            sprite: SpriteWrapper::PreInit(path.as_ref().to_string_lossy().to_string(), {
                let mut sprite = sprite::Sprite::default();
                sprite.set_frame_time(frame_time);
                sprite
            }),
            rel_pos: Vec2::ZERO,
            vel: Vec2::ZERO,
            duration: None,
        }
    }

    /// Sets the variant of the underlying sprite and returns the particle builder-pattern style.
    pub fn with_sprite_variant(mut self, variant: u32) -> Self {
        match &mut self.sprite {
            SpriteWrapper::PreInit(_, pre_init) => {
                pre_init.set_variant(variant);
            }
            SpriteWrapper::Initialized(sprite) => {
                sprite.set_variant(variant);
            }
        }
        self
    }

    /// Sets the duration of this particle and returns it builder-pattern style.
    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Sets the relative position of this particle and returns it builder-pattern style.
    pub fn with_relative_position(mut self, rel_x: f32, rel_y: f32) -> Self {
        self.rel_pos = Vec2::new(rel_x, rel_y);
        self
    }

    #[allow(dead_code)]
    /// Sets the relative position of this particle and returns it builder-pattern style.
    pub fn with_relative_position_vec(mut self, rel_pos: Vec2) -> Self {
        self.rel_pos = rel_pos;
        self
    }

    /// Sets the velocity of this particle and returns it builder-pattern style.
    pub fn with_velocity(mut self, dx: f32, dy: f32) -> Self {
        self.vel = Vec2::new(dx, dy);
        self
    }

    #[allow(dead_code)]
    /// Sets the velocity of this particle and returns it builder-pattern style.
    pub fn with_velocity_vec(mut self, vel: Vec2) -> Self {
        self.vel = vel;
        self
    }
}

#[system(for_each)]
/// A system that adds, moves and removes particles from a graphics component
pub fn handle_particles(
    gfx: &mut Graphics,
    actions: &super::Actions,
    #[resource] ix: &super::super::controller::Interactions,
) {
    // Move particles and reduce their durations.
    for particle in gfx.particles.iter_mut() {
        particle.duration = particle.duration.map(|d| d.saturating_sub(ix.delta));
        particle.rel_pos += particle.vel * ix.delta.as_secs_f32();
    }

    // Remove particles that have run out.
    gfx.particles.retain(|par| match par.duration {
        Some(dur) => !dur.is_zero(),
        None => true,
    });

    // Add new particles.
    for action in actions.get_actions() {
        if let GameAction::AddParticle(particle) = action {
            gfx.particles.push(particle.clone());
        }
    }
}
