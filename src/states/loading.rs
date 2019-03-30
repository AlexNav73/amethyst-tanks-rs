use crate::consts::{FONT, WALL_SPRITESHEET, WALL_TEXTURE};
use crate::states::gameplay::GamePlay;

use amethyst::{
    assets::{AssetStorage, Loader, ProgressCounter},
    prelude::*,
    renderer::{
        PngFormat, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata,
    },
    ui::{Anchor, FontAsset, FontHandle, TtfFormat, UiText, UiTransform},
};

pub struct LoadingState {
    progress_counter: ProgressCounter,
    sprite_sheet_handle: Option<SpriteSheetHandle>,
}

impl SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;

        let font_handle = self.load_font(world);
        view_debug_text(world, font_handle);

        self.sprite_sheet_handle = Some(self.load_sprite_sheet(world));
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        _event: StateEvent,
    ) -> SimpleTrans {
        if self.progress_counter.is_complete() {
            let sprite = self.sprite_sheet_handle.take().unwrap();
            return Trans::Switch(Box::new(GamePlay::new(sprite)));
        }

        Trans::None
    }
}

impl LoadingState {
    pub fn new() -> Self {
        Self {
            progress_counter: ProgressCounter::new(),
            sprite_sheet_handle: None,
        }
    }

    fn load_sprite_sheet(&mut self, world: &mut World) -> SpriteSheetHandle {
        let loader = world.read_resource::<Loader>();

        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        let texture_handle = loader.load(
            WALL_TEXTURE,
            PngFormat,
            TextureMetadata::srgb_scale(),
            &mut self.progress_counter,
            &texture_storage,
        );

        let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            WALL_SPRITESHEET,
            SpriteSheetFormat,
            texture_handle,
            &mut self.progress_counter,
            &sprite_sheet_store,
        )
    }

    fn load_font(&mut self, world: &mut World) -> FontHandle {
        let loader = world.read_resource::<Loader>();
        let font_storage = world.read_resource::<AssetStorage<FontAsset>>();
        loader.load(
            FONT,
            TtfFormat,
            Default::default(),
            &mut self.progress_counter,
            &font_storage,
        )
    }
}

fn view_debug_text(world: &mut World, font_handle: FontHandle) {
    use crate::components::DebugText;

    let transform = UiTransform::new(
        "log".to_string(),
        Anchor::TopLeft,
        50.,
        -50.,
        1.,
        800.,
        20.,
        0,
    );

    let log = world
        .create_entity()
        .with(transform)
        .with(UiText::new(
            font_handle,
            "".to_string(),
            [1., 1., 1., 1.],
            14.,
        ))
        .build();

    world.add_resource(DebugText { log });
}