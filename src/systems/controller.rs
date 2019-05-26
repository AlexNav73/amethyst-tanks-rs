use crate::components::Player;

use amethyst::{
    core::{nalgebra::Vector3, timing::Time, Transform},
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    input::InputHandler,
};

pub struct ControllerSystem;

impl<'s> System<'s> for ControllerSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<String, String>>,
        Read<'s, Time>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut transforms, player, input, time) = data;
        let delta = time.delta_seconds() as f64;
        for (transform, _) in (&mut transforms, &player).join() {
            if let Some(vertical) = input.axis_value("vertical") {
                let direction = Vector3::y_axis().into_inner() * vertical as f32;
                transform.move_local(direction);
            }
            if let Some(horizontal) = input.axis_value("horizontal") {
                let angle = (horizontal * delta * 5.0) as f32;
                transform.rotate_local(Vector3::z_axis(), -angle);
            }
        }
    }
}
