use crate::components::{Grass, Pos, Wall};
use crate::consts::{BATTLEFIELD_HEIGHT, BATTLEFIELD_WIDTH, CELL_HEIGHT, CELL_WIDTH};
use crate::map::Map;

use amethyst::{
    core::transform::Transform,
    ecs::prelude::Component,
    prelude::*,
    renderer::{SpriteRender, SpriteSheetHandle},
};

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
        let grid_width = self.map.first().map(|x| x.len() as f32).unwrap_or(1.0);
        let scale = BATTLEFIELD_WIDTH / (CELL_WIDTH * grid_width);
        let width = CELL_WIDTH * scale;
        let mut top = BATTLEFIELD_HEIGHT - (CELL_HEIGHT * scale * 0.5);
        let mut left;
        for line in self.map {
            left = width * 0.5;
            for cell in line {
                let pos = Pos::new(left, top);
                match cell {
                    MapCell::Wall(w) => place(w, pos, 0, scale, world, sprite_sheet.clone()),
                    MapCell::Grass(g) => place(g, pos, 1, scale, world, sprite_sheet.clone()),
                }
                left += width;
            }
            top -= CELL_HEIGHT * scale;
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

fn parce_line(line: &String) -> Vec<MapCell> {
    line.chars()
        .map(|c| match c {
            'g' => MapCell::Grass(Grass),
            'w' => MapCell::Wall(Wall),
            _ => unreachable!(),
        })
        .collect()
}
