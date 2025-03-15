use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};

use crate::gameplay::BallState;

pub struct MousePlugin;
impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, mouse_input_system);
    }
}

pub fn mouse_input_system(
    mut ball_state: ResMut<BallState>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    let window = windows.single();
    let (camera, camera_transform) = camera.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.truncate())
    {
        if mouse_button_input.pressed(MouseButton::Left) {
            info!("Hold! {}, {}", world_position.x, world_position.y);

            ball_state.dx = 0.0;
            ball_state.dy = 0.0;
            ball_state.x = world_position.x;
            ball_state.y = world_position.y;
        }
    }
}
