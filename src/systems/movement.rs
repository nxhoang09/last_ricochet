use bevy::prelude::*;
use crate::components::player::{Player, MovementStats};

pub fn player_movement(
    time: Res<Time>, 
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &MovementStats, &mut Sprite), With<Player>>,
) {
    let mut direction = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    if direction.length_squared() > 0.0 {
        direction = direction.normalize();
    }

    for (mut transform, stats, mut sprite) in query.iter_mut() {
        transform.translation += direction.extend(0.0) * stats.speed * time.delta_seconds();

        if direction.x < 0.0 {
            sprite.flip_x = true;
        } else if direction.x > 0.0 {
            sprite.flip_x = false;
        }
    }
}