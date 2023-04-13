use ggez::{Context, graphics::{DrawParam, Canvas}, glam::Vec2};
use mooeye::sprite;

use legion::*;

use super::Position;

const PIXEL_SIZE: f32 = 4.;

pub fn draw_sprites(world: &mut World, ctx: &Context, canvas: &mut Canvas){
    for (pos, sprite) in <(&Position, &mut sprite::Sprite)>::query().iter_mut(world){
        sprite.draw_sprite(ctx, canvas, DrawParam::default().dest(*pos).scale(Vec2::new(PIXEL_SIZE, PIXEL_SIZE)));
    }
}