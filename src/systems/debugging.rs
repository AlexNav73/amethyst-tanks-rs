use crate::components::ui::DebugText;
use crate::components::wall::Wall;

use amethyst::{
    core::Transform,
    ecs::{Join, ReadExpect, ReadStorage, System, WriteStorage},
    ui::UiText,
};

pub struct DebuggingSystem;

impl<'s> System<'s> for DebuggingSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Wall>,
        WriteStorage<'s, UiText>,
        ReadExpect<'s, DebugText>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut transforms, walls, mut ui_text, debug) = data;
        for (_, transform) in (&walls, &mut transforms).join() {
            if let Some(text) = ui_text.get_mut(debug.log) {
                let t = transform.translation();
                text.text = format!("{:?} : {:?}", t.x as u32, t.y as u32);
            }
        }
    }
}
