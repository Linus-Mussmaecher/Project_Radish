use ggez::{
    glam::Vec2,
    graphics::{self, Canvas, DrawParam, Drawable, MeshBuilder, Rect},
    Context,
};
use mooeye::sprite;

use legion::*;

use crate::PALETTE;

use super::{Health, Position, Velocity};

const PIXEL_SIZE: f32 = 4.;

/// Draws all the sprites in the world to their respective positions on the canvas.
pub fn draw_sprites(
    world: &mut World,
    ctx: &Context,
    canvas: &mut Canvas,
) -> Result<(), ggez::GameError> {
    for (pos, sprite, vel, health) in <(
        &Position,
        &mut sprite::Sprite,
        Option<&Velocity>,
        Option<&Health>,
    )>::query()
    .iter_mut(world)
    {
        // get factors/position for image mirrogin
        let factor = if match vel {
            Some(v) => v.get_dx() < 0.,
            None => false,
        } {
            -1.
        } else {
            1.
        };

        let n_pos = *pos
            + Vec2::new(
                -sprite.dimensions(ctx).unwrap_or_default().w * PIXEL_SIZE / 2. * factor,
                -sprite.dimensions(ctx).unwrap_or_default().w * PIXEL_SIZE / 2.,
            );

        // draw the sprite
        sprite.draw_sprite(
            ctx,
            canvas,
            DrawParam::default()
                .dest(n_pos)
                .scale(Vec2::new(PIXEL_SIZE * factor, PIXEL_SIZE)),
        );

        // draw the health bar
        if let Some(health) = health {
            let blip_size = 2;
            // get starting point
            let area = Rect::new(
                pos.x - (1 + (blip_size + 1) * health.get_max_health()) as f32 * PIXEL_SIZE / 2.,
                pos.y - ((blip_size + 3) as f32 + sprite.dimensions(ctx).expect("Could not unwrap dimension.").h/2.) * PIXEL_SIZE,
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
