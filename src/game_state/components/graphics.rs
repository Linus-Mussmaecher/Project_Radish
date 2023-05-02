use std::time::Duration;

use ggez::{
    glam::Vec2,
    graphics::{self, Canvas, DrawParam, Drawable, MeshBuilder, Rect},
    Context,
};
use mooeye::sprite::Sprite;

use legion::*;
use tinyvec::TinyVec;

use crate::{game_state::controller::Interactions, PALETTE};

use super::{actions::GameAction, Health, Position, Velocity};

pub const PIXEL_SIZE: f32 = 4.;

/// The graphics component of an entity, containing a sprite to be drawn to the screen and a container for multiple additional particles.
pub struct Graphics {
    /// The main sprite to represent this object.
    sprite: Sprite,
    /// Container for particles added to this and managed by this object.
    particles: TinyVec<[Particle; 4]>,
}

impl Graphics {
    /// Retunrs the underlying sprite of the object.
    pub fn get_sprite(&self) -> &Sprite {
        &self.sprite
    }

    /// Returns the objects size in the world, already multiplied by PIXEL_SIZE.
    pub fn get_size(&self) -> (f32, f32) {
        (
            self.sprite.get_dimensions().0 * PIXEL_SIZE,
            self.sprite.get_dimensions().1 * PIXEL_SIZE,
        )
    }
}

impl From<Sprite> for Graphics {
    fn from(value: Sprite) -> Self {
        Self {
            sprite: value,
            particles: TinyVec::new(),
        }
    }
}

/// Draws all the sprites in the world to their respective positions on the canvas.
pub fn draw_sprites(
    world: &mut World,
    resources: &mut Resources,
    ctx: &Context,
    canvas: &mut Canvas,
    animate: bool,
) -> Result<(), ggez::GameError> {
    for (pos, gfx, vel, health) in
        <(&Position, &mut Graphics, Option<&Velocity>, Option<&Health>)>::query().iter_mut(world)
    {
        // get boundaries for relative moving
        let boundaries = *resources.get::<Rect>().ok_or_else(|| {
            ggez::GameError::CustomError("Could not unpack boundaries.".to_owned())
        })?;
        let (screen_w, screen_h) = ctx.gfx.drawable_size();

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
                ((screen_h - boundaries.h)/2.).floor(),
            )
            // move to draw to correct position based on flip
            + Vec2::new(
                -gfx.sprite.get_dimensions().0 * PIXEL_SIZE / 2. * factor,
                -gfx.sprite.get_dimensions().1 * PIXEL_SIZE / 2.,
            );

        // draw the sprite
        if animate {
            gfx.sprite.draw_sprite(
                ctx,
                canvas,
                DrawParam::default()
                    .dest(n_pos)
                    .scale(Vec2::new(PIXEL_SIZE * factor, PIXEL_SIZE)),
            );
        } else {
            gfx.sprite.draw(
                canvas,
                DrawParam::default()
                    .dest(n_pos)
                    .scale(Vec2::new(PIXEL_SIZE * factor, PIXEL_SIZE)),
            );
        }

        // draw the sprites particles

        for part in gfx.particles.iter_mut() {
            if let Some(sprite) = &mut part.sprite {
                sprite.draw_sprite(
                    ctx,
                    canvas,
                    DrawParam::default()
                        .dest(
                            *pos + part.rel_pos
                                - Vec2::from(sprite.get_dimensions()) * PIXEL_SIZE / 2.
                                + Vec2::new(
                                    (screen_w - boundaries.w) / 2.,
                                    (screen_h - boundaries.h) / 2.,
                                ),
                        )
                        .scale(Vec2::new(PIXEL_SIZE, PIXEL_SIZE)),
                );
            }
        }

        // draw the health bar
        if let Some(health) = health {
            let blip_size = 2;
            // get starting point
            let area = Rect::new(
                pos.x - (1 + (blip_size + 1) * health.get_max_health()) as f32 * PIXEL_SIZE / 2.
                    + (screen_w - boundaries.w) / 2.,
                pos.y - ((blip_size + 3) as f32 + gfx.sprite.get_dimensions().1 / 2.) * PIXEL_SIZE
                    + (screen_h - boundaries.h) / 2.,
                (1 + (blip_size + 1) * health.get_max_health()) as f32 * PIXEL_SIZE,
                (blip_size + 2) as f32 * PIXEL_SIZE,
            );

            let mut health_bar_builder = MeshBuilder::new();
            // background
            health_bar_builder.rectangle(
                graphics::DrawMode::fill(),
                area,
                graphics::Color::from_rgb_u32(PALETTE[14]),
            )?;
            // health blips
            for i in 0..health.get_max_health() {
                health_bar_builder.rectangle(
                    graphics::DrawMode::fill(),
                    Rect::new(
                        area.x + (1 + (blip_size + 1) * i) as f32 * PIXEL_SIZE,
                        area.y + PIXEL_SIZE,
                        blip_size as f32 * PIXEL_SIZE,
                        blip_size as f32 * PIXEL_SIZE,
                    ),
                    graphics::Color::from_rgb_u32(if i < health.get_current_health() {
                        PALETTE[6]
                    } else {
                        PALETTE[11]
                    }),
                )?;
            }

            canvas.draw(
                &graphics::Mesh::from_data(ctx, health_bar_builder.build()),
                DrawParam::default(),
            );
        }
    }

    Ok(())
}

#[derive(Debug, Clone, Default)]
/// A struct that represents a Particle that can be added to a graphics component to be displayed on top of the main sprite.
pub struct Particle {
    /// The drawable to be displayed.
    sprite: Option<Sprite>,
    /// Relative position of the sprites center to the center of the main sprite.
    rel_pos: Vec2,
    /// Velocity this particle moves at (in pixels/s).
    vel: Vec2,
    /// The remaining duration of this particle. If None, it will stay indefinitely.
    duration: Option<Duration>,
}

#[allow(dead_code)]
impl Particle {
    /// Creates a new particle with the passed sprite, infinite duration and no velocity or offset.
    pub fn new(sprite: Sprite) -> Self {
        Self {
            sprite: Some(sprite),
            rel_pos: Vec2::ZERO,
            vel: Vec2::ZERO,
            duration: None,
        }
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
    #[resource] ix: &Interactions,
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
