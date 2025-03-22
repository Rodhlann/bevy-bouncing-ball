use bevy::prelude::*;

use crate::gameplay::Ball;

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (global_ui_setup, viewport_setup));
    }
}

fn global_ui_setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn viewport_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Ball,
        Mesh2d(meshes.add(Circle::new(50.0))),
        MeshMaterial2d(materials.add(Color::linear_rgb(1.0, 0.0, 0.0))),
        Transform::from_xyz(-100., 0., 0.1),
    ));
}
