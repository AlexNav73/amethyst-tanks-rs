use crate::components::{Drawable, Pos};

use amethyst::{
    core::transform::Transform,
    prelude::*,
    renderer::{SpriteRender, SpriteSheetHandle},
};

pub fn place<T>(comp: T, pos: Pos, scale: f32, world: &mut World, sprite_sheet: SpriteSheetHandle)
where
    T: Drawable,
{
    let mut transform = Transform::default();

    transform.set_xyz(pos.x, pos.y, 0.0);
    transform.set_scale(scale, scale, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet,
        sprite_number: T::SPRITE,
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(comp)
        .with(transform)
        .build();
}
