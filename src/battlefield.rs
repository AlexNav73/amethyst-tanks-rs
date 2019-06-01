use crate::components::Pos;
use crate::consts::{BATTLEFIELD_SIZE, CELL_SIZE};
use crate::map::{Map, MapCell};
use crate::utils::place;

use amethyst::{prelude::*, renderer::SpriteSheetHandle};

pub fn init_grid(map: Map, world: &mut World, sprite_sheet: SpriteSheetHandle) {
    let grid_width = map.size();
    let scale = BATTLEFIELD_SIZE / (CELL_SIZE * grid_width);
    let width = CELL_SIZE * scale;
    let mut top = BATTLEFIELD_SIZE - (CELL_SIZE * scale * 0.5);
    let mut left;
    for line in map.grid {
        left = width * 0.5;
        for cell in line {
            let pos = Pos::new(left, top);
            match cell {
                MapCell::Wall(w) => place(w, pos, scale, world, sprite_sheet.clone()),
                MapCell::Grass(g) => place(g, pos, scale, world, sprite_sheet.clone()),
                MapCell::Player(_) => {
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
