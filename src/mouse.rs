use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};

use crate::gameplay::BallState;

#[derive(Resource, Debug, Default)]
pub struct MouseState {
    pub pressed: bool,
    pub just_released: bool,
}

pub struct MousePlugin;
impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MouseState>();
        app.add_systems(Update, mouse_input_system);
    }
}

pub fn mouse_input_system(
    mut ball_state: ResMut<BallState>,
    mut mouse_state: ResMut<MouseState>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    let window = windows.single();
    let (camera, camera_transform) = camera.single();
    let mouse_delta = accumulated_mouse_motion.delta;

    let mut mouse = Vec2::new(0.0, 0.0);

    if let Some(mouse_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.truncate())
    {
        mouse = mouse_position;

        if mouse_button_input.pressed(MouseButton::Left) {
            mouse_state.pressed = true;
            mouse_state.just_released = false;
        } else if mouse_button_input.just_released(MouseButton::Left) {
            mouse_state.pressed = false;
            mouse_state.just_released = true;
        } else {
            mouse_state.pressed = false;
            mouse_state.just_released = false;
        }
    } else {
        if mouse_state.pressed == true {
            mouse_state.just_released = true;
        } else {
            mouse_state.just_released = false;
        }
        mouse_state.pressed = false;
    }

    if mouse_state.pressed {
        ball_state.dx = 0.0;
        ball_state.dy = 0.0;
        ball_state.x -= ball_state.x - mouse.x;
        ball_state.y -= ball_state.y - mouse.y;
    }

    if mouse_state.just_released {
        ball_state.dx = mouse_delta.x * 10.;
        ball_state.dy = -mouse_delta.y * 10.;
    }
}
