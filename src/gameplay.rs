use bevy::prelude::*;

const GRAVITY: f32 = 1200.0;
pub const BALL_HEIGHT: f32 = 50.;

#[derive(Component)]
pub struct Ball;

#[derive(Resource, Debug)]
pub struct BallState {
    pub x: f32,
    pub dx: f32,
    pub y: f32,
    pub dy: f32,
}

impl Default for BallState {
    fn default() -> Self {
        Self {
            x: 0.,
            y: 0.,
            dx: 200.,
            dy: -300.,
        }
    }
}

pub struct GameplayPlugin;
impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BallState>();
        app.add_systems(Update, ball_update_system);
    }
}

pub fn ball_update_system(
    windows: Query<&Window>,
    time: Res<Time>,
    mut ball_state: ResMut<BallState>,
    mut ball_query: Query<&mut Transform, With<Ball>>,
) {
    let window = windows.single();
    let window_width = window.width();
    let window_height = window.height();

    let window_top = window_height / 2.0;
    let window_bottom = -window_height / 2.0;
    let window_right = window_width / 2.0;
    let window_left = -window_width / 2.0;

    let mut horizontal_coll = false;
    let mut vertical_coll = false;

    ball_state.dy -= GRAVITY * time.delta_secs();

    // vertical collision
    if (ball_state.y - BALL_HEIGHT) <= window_bottom {
        ball_state.y = window_bottom + BALL_HEIGHT;
        vertical_coll = true;
    } else if (ball_state.y + BALL_HEIGHT) > window_top {
        ball_state.y = window_top - BALL_HEIGHT;
        vertical_coll = true;
    }

    // horizontal collision
    if (ball_state.x - BALL_HEIGHT) <= window_left {
        ball_state.x = window_left + BALL_HEIGHT;
        horizontal_coll = true;
    } else if (ball_state.x + BALL_HEIGHT) > window_right {
        ball_state.x = window_right - BALL_HEIGHT;
        horizontal_coll = true;
    }

    if vertical_coll {
        ball_state.dy = -ball_state.dy * 0.8;
        ball_state.dx *= 0.9;
    }

    if horizontal_coll {
        ball_state.dx = -ball_state.dx * 0.9;
    }

    let speed = (ball_state.dx.powf(2.0) + ball_state.dy.powf(2.0)).sqrt();
    if speed < 100.0 && ball_state.y <= window_bottom + BALL_HEIGHT + 2.0 {
        ball_state.dy *= 0.8;
        ball_state.y = window_bottom + BALL_HEIGHT;
    }

    let mut ball = ball_query.single_mut();
    ball_state.x += ball_state.dx * time.delta_secs();
    ball_state.y += ball_state.dy * time.delta_secs();
    ball.translation.x = ball_state.x;
    ball.translation.y = ball_state.y;
}
