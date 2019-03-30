use crate::battlefield::Battlefield;
use crate::consts::{BATTLEFIELD_HEIGHT, BATTLEFIELD_WIDTH};
//use crate::states::paused::PausedState;

use amethyst::{
    core::transform::Transform,
    input::{is_close_requested, is_key_down},
    prelude::*,
    renderer::{Camera, Projection, SpriteSheetHandle, VirtualKeyCode},
};

pub struct GamePlay {
    battlefield: Battlefield,
    sprite_sheet_handle: SpriteSheetHandle,
}

impl GamePlay {
    pub fn new(sprite_sheet_handle: SpriteSheetHandle) -> Self {
        Self {
            battlefield: Battlefield,
            sprite_sheet_handle,
        }
    }
}

impl SimpleState for GamePlay {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { mut world, .. } = data;

        add_camera(&mut world);
        self.battlefield
            .add_walls(&mut world, self.sprite_sheet_handle.clone());
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                //return Trans::Push(Box::new(PausedState));
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
