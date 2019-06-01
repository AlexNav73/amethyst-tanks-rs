use crate::components::Pos;
use crate::consts::{BATTLEFIELD_SIZE, CELL_SIZE};
use crate::map::{Map, MapCell};

use amethyst::{
    core::transform::Transform,
    ecs::prelude::Component,
    prelude::*,
    renderer::{SpriteRender, SpriteSheetHandle},
};

pub struct Battlefield {
    map: Map,
}

impl Battlefield {
    pub fn new(map: Map) -> Self {
        Self { map }
    }

    pub fn init_grid(self, world: &mut World, sprite_sheet: SpriteSheetHandle) {
        let grid_width = self.map.size();
        let scale = BATTLEFIELD_SIZE / (CELL_SIZE * grid_width);
        let width = CELL_SIZE * scale;
        let mut top = BATTLEFIELD_SIZE - (CELL_SIZE * scale * 0.5);
        let mut left;
        for line in self.map.grid {
            left = width * 0.5;
            for cell in line {
                let pos = Pos::new(left, top);
                match cell {
                    MapCell::Wall(w) => place(w, pos, 0, scale, world, sprite_sheet.clone()),
                    MapCell::Grass(g) => place(g, pos, 1, scale, world, sprite_sheet.clone()),
                    MapCell::Player(p) => {
                        //use crate::components::Grass;

                        //place(Grass, pos, 1, scale, world, sprite_sheet.clone());
                        //place(p, pos, 2, scale, world, sprite_sheet.clone());
                    }
                }
                left += width;
            }
            top -= CELL_SIZE * scale;
        }
    }
}

fn place<T>(
    comp: T,
    pos: Pos,
    sprite: usize,
    scale: f32,
    world: &mut World,
    sprite_sheet: SpriteSheetHandle,
) where
    T: Component + Send + Sync,
{
    let mut transform = Transform::default();

    transform.set_xyz(pos.x, pos.y, 0.0);
    transform.set_scale(scale, scale, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet,
        sprite_number: sprite,
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(comp)
        .with(transform)
        .build();
}
