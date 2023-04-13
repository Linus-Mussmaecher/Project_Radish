use ggez::{
    glam::Vec2,
    graphics::{Canvas, DrawParam},
    Context,
};
use mooeye::sprite;

use legion::*;

use super::{Position, Velocity};

const PIXEL_SIZE: f32 = 4.;

pub fn draw_sprites(world: &mut World, ctx: &Context, canvas: &mut Canvas) {
    for (pos, sprite, vel) in
        <(&Position, &mut sprite::Sprite, Option<&Velocity>)>::query().iter_mut(world)
    {
        let factor = match vel {
            Some(v) => {
                if v.get_dx() >= 0. {
                    1.
                } else {
                    -1.
                }
            }
            None => 1.,
        };
        sprite.draw_sprite(
            ctx,
            canvas,
            DrawParam::default()
                .dest(*pos)
                .scale(Vec2::new(PIXEL_SIZE * factor, PIXEL_SIZE)),
        );
    }
}
