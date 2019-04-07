use crate::consts::{WALL_HEIGHT, WALL_WIDTH};

use amethyst::ecs::prelude::{Component, DenseVecStorage, Entity};

pub struct DebugText {
    pub log: Entity,
}

pub struct Wall {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Wall {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width,
            height,
        }
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
