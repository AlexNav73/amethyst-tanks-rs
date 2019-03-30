use crate::battlefield::Battlefield;
use crate::consts::{BATTLEFIELD_HEIGHT, BATTLEFIELD_WIDTH, WALL_SPRITESHEET, WALL_TEXTURE};
//use crate::states::paused::PausedState;

use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    input::is_key_down,
    prelude::*,
    renderer::{
        Camera, PngFormat, Projection, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle, Texture,
        TextureMetadata, VirtualKeyCode,
    },
};

pub struct GamePlay {
    battlefield: Battlefield,
}

impl GamePlay {
    pub fn new() -> Self {
        Self {
            battlefield: Battlefield,
        }
    }
}

impl SimpleState for GamePlay {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { mut world, .. } = data;

        let sprite_sheet_handle = load_sprite_sheet(world);

        add_camera(&mut world);
        self.battlefield.add_walls(&mut world, sprite_sheet_handle);
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
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

fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            WALL_TEXTURE,
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        WALL_SPRITESHEET,
        SpriteSheetFormat,
        texture_handle,
        (),
        &sprite_sheet_store,
    )
}
