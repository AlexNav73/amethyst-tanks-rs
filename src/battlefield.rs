use crate::components::{Grass, Pos, Wall};
use crate::consts::{BATTLEFIELD_HEIGHT, CELL_HEIGHT, CELL_WIDTH};
use crate::map::Map;

use amethyst::{
    core::transform::Transform,
    ecs::prelude::Component,
    prelude::*,
    renderer::{SpriteRender, SpriteSheetHandle},
};

const SCALE: f32 = 0.01;

pub enum MapCell {
    Wall(Wall),
    Grass(Grass),
}

pub struct Battlefield {
    map: Vec<Vec<MapCell>>,
}

impl Battlefield {
    pub fn from_file(map: &Map) -> Self {
        let map = map.0.iter().map(parce_line).collect();
        Self { map }
    }

    pub fn init_grid(self, world: &mut World, sprite_sheet: SpriteSheetHandle) {
        const WIDTH: f32 = CELL_WIDTH * SCALE;

        let mut top = BATTLEFIELD_HEIGHT - (CELL_HEIGHT * SCALE * 0.5);
        let mut left;
        for line in self.map {
            left = WIDTH * 0.5;
            for cell in line {
                let pos = Pos::new(left, top);
                match cell {
                    MapCell::Wall(w) => place(w, pos, 0, world, sprite_sheet.clone()),
                    MapCell::Grass(g) => place(g, pos, 1, world, sprite_sheet.clone()),
                }
                left += WIDTH;
            }
            top -= CELL_HEIGHT * SCALE;
        }
    }
}

fn place<T>(comp: T, pos: Pos, sprite: usize, world: &mut World, sprite_sheet: SpriteSheetHandle)
where
    T: Component + Send + Sync,
{
    let mut transform = Transform::default();

    transform.set_xyz(pos.x, pos.y, 0.0);
    transform.set_scale(SCALE, SCALE, 0.0);

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

fn parce_line(line: &String) -> Vec<MapCell> {
    line.chars()
        .map(|c| match c {
            'g' => MapCell::Grass(Grass),
            'w' => MapCell::Wall(Wall),
            _ => unreachable!(),
        })
        .collect()
}
