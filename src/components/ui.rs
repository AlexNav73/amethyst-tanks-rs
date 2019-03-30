use amethyst::ecs::prelude::Entity;

#[derive(Default)]
pub struct DebugLog {
    pub log: String,
}

pub struct DebugText {
    pub log: Entity,
}
