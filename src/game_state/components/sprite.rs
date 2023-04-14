use ggez::{
    glam::Vec2,
    graphics::{Canvas, DrawParam, Drawable},
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
        let (factor, pos) = if match vel {
            Some(v) => v.get_dx() < 0.,
            None => false,
        }{
            (-1., *pos + Vec2::new(sprite.dimensions(ctx).unwrap_or_default().w * PIXEL_SIZE, 0.))
        } else {
            (1., *pos)
        };

        sprite.draw_sprite(
            ctx,
            canvas,
            DrawParam::default()
                .dest(pos)
                .scale(Vec2::new(PIXEL_SIZE * factor, PIXEL_SIZE)),
        );
    }
}
