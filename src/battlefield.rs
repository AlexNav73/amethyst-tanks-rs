use crate::consts::BATTLEFIELD_HEIGHT;
use crate::wall::Wall;

use amethyst::{
    core::transform::Transform,
    prelude::*,
    renderer::{SpriteRender, SpriteSheetHandle},
};
use pathfinding::prelude::{absdiff, astar};

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

pub struct Battlefield;

impl Battlefield {
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
        let mut wall: Wall = Default::default();
        wall.y = BATTLEFIELD_HEIGHT / 2.0;
        wall.x = wall.width * 0.5;

        add_wall_to_world(wall, world, sprite_sheet);
    }
}

fn add_wall_to_world(wall: Wall, world: &mut World, sprite_sheet: SpriteSheetHandle) {
    let mut transform = Transform::default();

    transform.set_xyz(wall.x * 1.5, wall.y, 0.0);
    transform.set_scale(1.5, 1.5, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(wall)
        .with(transform)
        .build();
}
