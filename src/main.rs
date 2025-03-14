use bevy::{prelude::*, winit::WinitSettings};
use gameplay::GameplayPlugin;
use ui::UIPlugin;

mod gameplay;
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
        ))
        .insert_resource(WinitSettings::game())
        .run();
}
