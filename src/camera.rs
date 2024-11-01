use bevy::input::mouse::MouseScrollUnit;
use bevy::{input::mouse::MouseWheel, prelude::*};

const CAMERA_SPEED: f32 = 200.0;

#[derive(Component, Debug)]
pub struct Camera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, (move_camera, zoom_camera));
    }
}

fn setup_camera(mut commands: Commands) {
    // Set up a 2D orthographic camera
    commands.spawn((Camera2dBundle::default(), Camera));
}

fn move_camera(
    mut query: Query<(&mut Transform, &OrthographicProjection), With<Camera>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut direction = Vec2::ZERO;
    let (mut transform, projection) = query.single_mut();
    if input.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if input.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }
    if input.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if input.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }
    if direction != Vec2::ZERO {
        direction = direction.normalize();
    }
    transform.translation.x += direction.x * CAMERA_SPEED * projection.scale * time.delta_seconds();
    transform.translation.y += direction.y * CAMERA_SPEED * projection.scale * time.delta_seconds();
}

fn zoom_camera(
    mut query: Query<&mut OrthographicProjection, With<Camera>>,
    mut scroll: EventReader<MouseWheel>,
    time: Res<Time>,
) {
    let mut projection = query.single_mut();

    for event in scroll.read() {
        match event.unit {
            MouseScrollUnit::Line => {
                // Adjust the scale (zoom) based on scroll direction
                let zoom_delta = -event.y * 5.0 * time.delta_seconds();
                let new_scale = projection.scale * (1.0 + zoom_delta);
                projection.scale = new_scale.clamp(0.2, 100.0);
            }
            MouseScrollUnit::Pixel => {
                println!("Scroll (pixel units): vertical:  horizontal:");
            }
        }
    }
}