mod components;
mod consts;
mod states;

use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage},
    utils::application_root_dir,
    Logger,
};

use crate::states::{gameplay::GamePlay, main_menu::MainMenu};

fn main() -> amethyst::Result<()> {
    Logger::from_config(Default::default())
        .level_for("gfx_device_gl", amethyst::LogLevelFilter::Warn)
        .start();

    let app_root = application_root_dir();
    let path = format!("{}/resources/display_config.ron", app_root);
    let config = DisplayConfig::load(&path);

    let stage = Stage::with_backbuffer()
        .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
        .with_pass(DrawFlat2D::new());
    let pipe = Pipeline::build().with_stage(stage);

    let bundle = RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor();
    let game_data = GameDataBuilder::default()
        .with_bundle(bundle)?
        .with_bundle(TransformBundle::new())?;
    let mut game = Application::new("./", GamePlay, game_data)?;

    game.run();

    Ok(())
}
