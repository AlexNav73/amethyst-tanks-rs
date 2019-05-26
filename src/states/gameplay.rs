use crate::consts::{BATTLEFIELD_HEIGHT, BATTLEFIELD_WIDTH};

use amethyst::{
    core::transform::Transform,
    input::{is_close_requested, is_key_down},
    prelude::*,
    renderer::{Camera, Projection, VirtualKeyCode},
};

pub struct GamePlay;

impl SimpleState for GamePlay {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { mut world, .. } = data;

        add_camera(&mut world);
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }
        }

        Trans::None
    }
}

fn add_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);

    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            BATTLEFIELD_WIDTH,
            0.0,
            BATTLEFIELD_HEIGHT,
        )))
        .with(transform)
        .build();
}
