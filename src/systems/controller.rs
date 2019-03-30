use crate::wall::Wall;

use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;

pub struct ControllerSystem;

impl<'s> System<'s> for ControllerSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Wall>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (mut transforms, walls, input): Self::SystemData) {
        for (_, transform) in (&walls, &mut transforms).join() {
            if let Some(vertical) = input.axis_value("vertical") {
                transform.translate_y(vertical as f32);
            }
            if let Some(horizontal) = input.axis_value("horizontal") {
                transform.translate_x(horizontal as f32);
            }
        }
    }
}
