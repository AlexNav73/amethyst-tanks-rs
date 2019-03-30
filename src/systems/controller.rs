use crate::components::wall::Wall;

use amethyst::{
    core::{timing::Time, Transform},
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    input::InputHandler,
};

pub struct ControllerSystem;

impl<'s> System<'s> for ControllerSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Wall>,
        Read<'s, InputHandler<String, String>>,
        Read<'s, Time>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut transforms, walls, input, time) = data;
        let delta = time.delta_seconds() as f64;
        for (_, transform) in (&walls, &mut transforms).join() {
            if let Some(vertical) = input.axis_value("vertical") {
                transform.translate_y((vertical * delta * 5.0) as f32);
            }
            if let Some(horizontal) = input.axis_value("horizontal") {
                transform.translate_x((horizontal * delta * 5.0) as f32);
            }
        }
    }
}
