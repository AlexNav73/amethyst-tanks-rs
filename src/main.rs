mod battlefield;
mod components;
mod consts;
mod states;
mod systems;
mod wall;

use crate::states::gameplay::GamePlay;
use crate::systems::controller::ControllerSystem;

use amethyst::{
    core::transform::TransformBundle,
    input::InputBundle,
    prelude::*,
    renderer::{DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage},
    utils::application_root_dir,
    Logger,
};

fn main() -> amethyst::Result<()> {
    Logger::from_config(Default::default())
        .level_for("gfx_device_gl", amethyst::LogLevelFilter::Warn)
        .start();

    let app_root = application_root_dir();
    let path = format!("{}/resources/display_config.ron", app_root);
    let display_config = DisplayConfig::load(&path);

    let path = format!("{}/resources/bindings_config.ron", app_root);
    let input_bundle = InputBundle::<String, String>::new().with_bindings_from_file(path)?;

    let stage = Stage::with_backbuffer()
        .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
        .with_pass(DrawFlat2D::new());
    let pipe = Pipeline::build().with_stage(stage);

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(display_config)).with_sprite_sheet_processor())?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(ControllerSystem, "controller_system", &["input_system"]);
    let mut game = Application::new("./", GamePlay::new(), game_data)?;

    game.run();

    Ok(())
}
