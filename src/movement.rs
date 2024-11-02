use bevy::prelude::*;

const SPEED: f32 = 200.0;

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_position);
    }
}

fn update_position(
    mut query: Query<(&mut Velocity, &mut Transform)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut velocity, mut transform) in query.iter_mut() {
        velocity.y = 0.0;
        velocity.x = 0.0;
        if input.pressed(KeyCode::ArrowLeft) {
            velocity.x = -1.0;
        }
        if input.pressed(KeyCode::ArrowRight) {
            velocity.x = 1.0;
        }
        if input.pressed(KeyCode::ArrowUp) {
            velocity.y = 1.0;
        }
        if input.pressed(KeyCode::ArrowDown) {
            velocity.y = -1.0;
        }

        transform.translation.x += velocity.x * SPEED * time.delta_seconds();
        transform.translation.y += velocity.y * SPEED * time.delta_seconds();
    }
}
