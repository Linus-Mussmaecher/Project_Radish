use ggez::{
    glam::Vec2,
    graphics::{self, Canvas, DrawParam, Drawable, MeshBuilder, Rect},
    Context,
};
use mooeye::sprite::{self, Sprite};

use legion::*;
use tinyvec::TinyVec;

use crate::PALETTE;

use super::{Health, Position, Velocity};

struct Particle{

}

impl Default for Particle{
    fn default() -> Self {
        Self {  }
    }
}
pub struct Graphics{
    sprite: Sprite,
    particles: TinyVec<[Particle; 4]>,
}

impl Graphics{
    
}

impl From<Sprite> for Graphics{
    fn from(value: Sprite) -> Self {
        Self { sprite: value, particles: TinyVec::new() }
    }
}

const PIXEL_SIZE: f32 = 4.;

/// Draws all the sprites in the world to their respective positions on the canvas.
pub fn draw_sprites(
    world: &mut World,
    resources: &mut Resources,
    ctx: &Context,
    canvas: &mut Canvas,
    animate: bool,
) -> Result<(), ggez::GameError> {
    for (pos, gfx, vel, health) in <(
        &Position,
        &mut Graphics,
        Option<&Velocity>,
        Option<&Health>,
    )>::query()
    .iter_mut(world)
    {
        // get boundaries for relative moving
        let boundaries = *resources
            .get::<Rect>()
            .ok_or_else(|| ggez::GameError::CustomError("Could not unpack boundaries.".to_owned()))?;
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

        let n_pos = 
            // position within the world
            *pos
            // move as the world is positioned on screen
            + Vec2::new(
                (screen_w - boundaries.w)/2.,
                (screen_h - boundaries.h)/2.,
            )
            // move to draw to correct position based on flip
            + Vec2::new(
                -gfx.sprite.dimensions(ctx).unwrap_or_default().w * PIXEL_SIZE / 2. * factor,
                -gfx.sprite.dimensions(ctx).unwrap_or_default().w * PIXEL_SIZE / 2.,
            );

        // draw the sprite
        if animate{
        gfx.sprite.draw_sprite(
            ctx,
            canvas,
            DrawParam::default()
                .dest(n_pos)
                .scale(Vec2::new(PIXEL_SIZE * factor, PIXEL_SIZE)),
        );
    } else {
        gfx.sprite.draw(canvas, DrawParam::default()
        .dest(n_pos)
        .scale(Vec2::new(PIXEL_SIZE * factor, PIXEL_SIZE)));
    }

        // draw the health bar
        if let Some(health) = health {
            let blip_size = 2;
            // get starting point
            let area = Rect::new(
                pos.x - (1 + (blip_size + 1) * health.get_max_health()) as f32 * PIXEL_SIZE / 2. + (screen_w - boundaries.w)/2.,
                pos.y - ((blip_size + 3) as f32 + gfx.sprite.dimensions(ctx).expect("Could not unwrap dimension.").h/2.) * PIXEL_SIZE + (screen_h - boundaries.h)/2.,
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
