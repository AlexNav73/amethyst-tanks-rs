use crate::components::Wall;
use crate::consts::{BATTLEFIELD_HEIGHT, WALL_HEIGHT, WALL_WIDTH};
use crate::map::Map;

use amethyst::{
    core::transform::Transform,
    prelude::*,
    renderer::{SpriteRender, SpriteSheetHandle},
};
use pathfinding::prelude::{absdiff, astar};

const SCALE: f32 = 0.01;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Pos(i32, i32);

impl Pos {
    #[allow(dead_code)]
    fn distance(&self, other: &Pos) -> u32 {
        (absdiff(self.0, other.0) + absdiff(self.1, other.1)) as u32
    }

    #[allow(dead_code)]
    fn successors(&self) -> Vec<(Pos, u32)> {
        let &Pos(x, y) = self;
        vec![
            (Pos(x, y - 1), 2),
            (Pos(x + 1, y - 1), 1),
            (Pos(x + 1, y), 2),
            (Pos(x + 1, y + 1), 1),
            (Pos(x, y + 1), 2),
            (Pos(x - 1, y + 1), 1),
            (Pos(x - 1, y), 2),
            (Pos(x - 1, y - 1), 1),
        ]
    }
}

pub struct Battlefield {
    map: Vec<Vec<bool>>,
}

impl Battlefield {
    pub fn from_file(map: &Map) -> Self {
        let map = map.0.iter().map(parce_line).collect();
        Self { map }
    }

    #[allow(dead_code)]
    pub fn find(&self) {
        let result = astar(
            &Pos(1, 1),
            |p| p.successors(),
            |p| p.distance(&Pos(4, 6)),
            |p| *p == Pos(4, 6),
        );
        dbg!(result);
    }

    pub fn add_walls(&self, world: &mut World, sprite_sheet: SpriteSheetHandle) {
        const WIDTH: f32 = WALL_WIDTH * SCALE;

        let mut top = BATTLEFIELD_HEIGHT - (WALL_HEIGHT * SCALE * 0.5);
        let mut left;
        for line in &self.map {
            left = WIDTH * 0.5;
            for cell in line {
                if *cell {
                    let wall = Wall::new(left, top);
                    place_wall(wall, world, sprite_sheet.clone());
                }
                left += WIDTH;
            }
            top -= WALL_HEIGHT * SCALE;
        }
    }
}

fn place_wall(wall: Wall, world: &mut World, sprite_sheet: SpriteSheetHandle) {
    let mut transform = Transform::default();

    transform.set_xyz(wall.x, wall.y, 0.0);
    transform.set_scale(SCALE, SCALE, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet,
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(wall)
        .with(transform)
        .build();
}

fn parce_line(line: &String) -> Vec<bool> {
    line.chars()
        .map(|c| if c == '0' { false } else { true })
        .collect()
}
