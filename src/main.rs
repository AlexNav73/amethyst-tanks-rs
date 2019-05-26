mod battlefield;
mod components;
mod config;
mod consts;
mod map;
mod states;
mod systems;

use amethyst::{
    assets::Processor,
    config::Config,
    core::transform::TransformBundle,
    input::InputBundle,
    prelude::*,
    renderer::{DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage},
    ui::{DrawUi, UiBundle},
    utils::application_root_dir,
    Logger,
};

use crate::config::GameConfig;
use crate::map::Map;
use crate::states::loading::LoadingState;
use crate::systems::{controller::ControllerSystem, debugging::DebuggingSystem};

fn main() -> amethyst::Result<()> {
    Logger::from_config(Default::default())
        .level_for("amethyst_renderer", amethyst::LogLevelFilter::Warn)
        .level_for("gfx_device_gl", amethyst::LogLevelFilter::Warn)
        .level_for("amethyst_assets", amethyst::LogLevelFilter::Warn)
        .start();

    let app_root = application_root_dir();

    let path = format!("{}/resources/display_config.ron", app_root);
    let display_config = DisplayConfig::load(&path);

    let path = format!("{}/resources/bindings_config.ron", app_root);
    let input_bundle = InputBundle::<String, String>::new().with_bindings_from_file(path)?;

    let path = format!("{}/resources/game_config.ron", app_root);
    let game_config = GameConfig::load_no_fallback(&path);

    let stage = Stage::with_backbuffer()
        .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
        .with_pass(DrawFlat2D::new())
        .with_pass(DrawUi::new());
    let pipe = Pipeline::build().with_stage(stage);

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(display_config)).with_sprite_sheet_processor())?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<String, String>::new())?
        .with(Processor::<Map>::new(), "", &[])
        .with(ControllerSystem, "controller_system", &["input_system"])
        .with(DebuggingSystem, "debugging_system", &[]);

    let mut game = Application::build("./", LoadingState::new())?
        .with_resource(game_config)
        .build(game_data)?;

    game.run();

    Ok(())
}
