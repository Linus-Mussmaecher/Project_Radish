use ggez::{Context, graphics::{DrawParam, Canvas}};
use mooeye::sprite::Sprite;

use legion::*;

use super::Position;

pub fn draw_sprites(world: &mut World, ctx: &Context, canvas: &mut Canvas){
    println!("maybe trying");
    for (pos, sprite) in <(&Position, &mut Sprite)>::query().iter_mut(world){
        sprite.draw_sprite(ctx, canvas, DrawParam::default().dest(*pos));
        println!("Trying")
    }
}