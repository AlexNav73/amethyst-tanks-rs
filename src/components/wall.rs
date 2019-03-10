use crate::consts::{WALL_HEIGHT, WALL_WIDTH};

use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Wall {
    pub width: f32,
    pub height: f32,
}

impl Wall {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

impl Default for Wall {
    fn default() -> Self {
        Self::new(WALL_WIDTH, WALL_HEIGHT)
    }
}

impl Component for Wall {
    type Storage = DenseVecStorage<Self>;
}
