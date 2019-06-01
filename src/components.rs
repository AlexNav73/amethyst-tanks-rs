use amethyst::ecs::prelude::{Component, DenseVecStorage, Entity};

pub struct DebugText {
    pub log: Entity,
}

#[derive(Copy, Clone)]
pub struct Pos {
    pub x: f32,
    pub y: f32,
}

impl Pos {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Default for Pos {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

pub struct Wall;
pub struct Grass;
pub struct Player;

macro_rules! impl_comp_for {
    ($t:ty) => {
        impl Component for $t {
            type Storage = DenseVecStorage<Self>;
        }
    };
}

impl_comp_for!(Pos);
impl_comp_for!(Wall);
impl_comp_for!(Grass);
impl_comp_for!(Player);

pub trait Drawable: Component + Send + Sync {
    const SPRITE: usize;
}

macro_rules! impl_drawable {
    ($t:ty, $s:literal) => {
        impl Drawable for $t {
            const SPRITE: usize = $s;
        }
    };
}

impl_drawable!(Wall, 0);
impl_drawable!(Grass, 1);
impl_drawable!(Player, 2);
