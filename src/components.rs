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
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            ..Default::default()
        }
    }
}

impl Default for Wall {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: WALL_WIDTH,
            height: WALL_HEIGHT,
        }
    }
}

impl Component for Wall {
    type Storage = DenseVecStorage<Self>;
}

pub struct Player;

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}
