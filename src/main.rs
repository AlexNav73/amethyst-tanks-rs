mod states;

use amethyst::{
    prelude::*,
    renderer::{DisplayConfig, DrawFlat, Pipeline, PosNormTex, RenderBundle, Stage},
    utils::application_root_dir,
};

use crate::states::{
    main_menu::MainMenu
};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let path = format!("{}/resources/display_config.ron", application_root_dir());
    let config = DisplayConfig::load(&path);

    let stage = Stage::with_backbuffer()
        .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
        .with_pass(DrawFlat::<PosNormTex>::new());
    let pipe = Pipeline::build().with_stage(stage);

    let bundle = RenderBundle::new(pipe, Some(config));
    let game_data = GameDataBuilder::default().with_bundle(bundle)?;
    let mut game = Application::new("./", MainMenu, game_data)?;

    game.run();

    Ok(())
}
