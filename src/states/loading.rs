use crate::assets::map::{MapAsset, MapFormat, MapHandle, MapSource};
use crate::battlefield::init_grid;
use crate::consts::{FONT, WALL_SPRITESHEET, WALL_TEXTURE};
use crate::map::Map;
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
    map_handle: Option<MapHandle>,
}

impl SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;

        let font_handle = self.load_font(world);
        view_debug_text(world, font_handle);

        self.load_sprite_sheet(world);
        self.load_map(world);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if self.progress_counter.is_complete() {
            use crate::components::{Grass, Wall};

            data.world.register::<Wall>();
            data.world.register::<Grass>();

            self.load_battlefield(data.world);

            return Trans::Switch(Box::new(GamePlay));
        } else {
            use crate::components::DebugText;
            let StateData { world, .. } = data;

            let mut ui_text = world.write_storage::<UiText>();
            let debug_text = world.read_resource::<DebugText>();

            if let Some(text) = ui_text.get_mut(debug_text.log) {
                let total_assets = self.progress_counter.num_assets();
                let loading = self.progress_counter.num_loading();

                text.text = format!("Loading {:?} of {:?}", loading, total_assets);
            }
        }

        Trans::None
    }
}

impl LoadingState {
    pub fn new() -> Self {
        Self {
            progress_counter: ProgressCounter::new(),
            sprite_sheet_handle: None,
            map_handle: None,
        }
    }

    fn load_sprite_sheet(&mut self, world: &mut World) {
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
        let handle = loader.load(
            WALL_SPRITESHEET,
            SpriteSheetFormat,
            texture_handle,
            &mut self.progress_counter,
            &sprite_sheet_store,
        );

        self.sprite_sheet_handle = Some(handle);
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

    fn load_map(&mut self, world: &mut World) {
        {
            let mut loader = world.write_resource::<Loader>();
            loader.add_source("map_source", MapSource);
        }

        let loader = world.read_resource::<Loader>();
        let map_storage = world.read_resource::<AssetStorage<MapAsset>>();
        let map_handle = loader.load_from(
            "resources/maps/map1.ron",
            MapFormat,
            (),
            "map_source",
            &mut self.progress_counter,
            &map_storage,
        );

        self.map_handle = Some(map_handle);
    }

    fn load_battlefield(&mut self, world: &mut World) {
        let sprite = self.sprite_sheet_handle.take().unwrap();
        let map: Map = {
            let map_storage = world.read_resource::<AssetStorage<MapAsset>>();
            let map_handle = self.map_handle.take().unwrap();
            map_storage
                .get(&map_handle)
                .expect("Acquire map by handle failed")
                .into()
        };
        init_grid(map, world, sprite);
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
