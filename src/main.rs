use bevy::{prelude::*, winit::WinitSettings};
use gameplay::GameplayPlugin;
use mouse::MousePlugin;
use ui::UIPlugin;

mod gameplay;
mod mouse;
mod ui;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Bouncing Ball".into(),
                    resolution: (850.0, 720.0).into(),
                    resizable: false,
                    ..default()
                }),
                ..default()
            }),
            UIPlugin,
            GameplayPlugin,
            MousePlugin,
        ))
        .insert_resource(WinitSettings::game())
        .add_systems(Update, frame_limiter_system)
        .run();
}

fn frame_limiter_system() {
    use std::{thread, time};
    thread::sleep(time::Duration::from_millis(15));
}
