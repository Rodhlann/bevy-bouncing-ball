use bevy::prelude::*;

const GRAVITY: f32 = 1200.0;
pub const BALL_HEIGHT: f32 = 50.;

#[derive(Component)]
pub struct Ball;

#[derive(Resource, Debug)]
pub struct BallState {
    pub pos: Vec2,
    pub delta: Vec2,
}

impl Default for BallState {
    fn default() -> Self {
        Self {
            pos: Vec2 { x: 0.0, y: 0.0 },
            delta: Vec2 {
                x: 200.0,
                y: -300.0,
            },
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

    ball_state.delta.y -= GRAVITY * time.delta_secs();

    // vertical collision
    if (ball_state.pos.y - BALL_HEIGHT) <= window_bottom {
        ball_state.pos.y = window_bottom + BALL_HEIGHT;
        vertical_coll = true;
    } else if (ball_state.pos.y + BALL_HEIGHT) > window_top {
        ball_state.pos.y = window_top - BALL_HEIGHT;
        vertical_coll = true;
    }

    // horizontal collision
    if (ball_state.pos.x - BALL_HEIGHT) <= window_left {
        ball_state.pos.x = window_left + BALL_HEIGHT;
        horizontal_coll = true;
    } else if (ball_state.pos.x + BALL_HEIGHT) > window_right {
        ball_state.pos.x = window_right - BALL_HEIGHT;
        horizontal_coll = true;
    }

    if vertical_coll {
        ball_state.delta.y = -ball_state.delta.y * 0.8;
        ball_state.delta.x *= 0.9;
    }

    if horizontal_coll {
        ball_state.delta.x = -ball_state.delta.x * 0.9;
    }

    let speed = (ball_state.delta.x.powf(2.0) + ball_state.delta.y.powf(2.0)).sqrt();
    if speed < 100.0 && ball_state.pos.y <= window_bottom + BALL_HEIGHT + 2.0 {
        ball_state.delta.y *= 0.8;
        ball_state.pos.y = window_bottom + BALL_HEIGHT;
    }

    let mut ball = ball_query.single_mut();
    ball_state.pos.x += ball_state.delta.x * time.delta_secs();
    ball_state.pos.y += ball_state.delta.y * time.delta_secs();
    ball.translation.x = ball_state.pos.x;
    ball.translation.y = ball_state.pos.y;
}
