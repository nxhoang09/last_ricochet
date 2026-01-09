use bevy::prelude::*;
use crate::components::enemy::{Enemy, EnemyAnimationTimer, EnemySpeed};

pub fn animate_enemies(
    time: Res<Time>,
    mut query: Query<(&mut EnemyAnimationTimer, &mut TextureAtlas)>,
) {
    for (mut timer, mut atlas) in query.iter_mut() {
        timer.timer.tick(time.delta());
        if timer.timer.just_finished() {  
            atlas.index = (atlas.index + 1) % 4;
        }
    }
}

pub fn enemy_movement(
    time: Res<Time>,
    mut enemy_query: Query<(&mut Transform, &mut Sprite, &EnemySpeed), With<Enemy>>, 
    player_query: Query<&Transform, (With<crate::components::player::Player>, Without<Enemy>)>,
) {
    let Ok(player_transform) = player_query.get_single() else { return };
    let player_pos = player_transform.translation.truncate();

    for (mut enemy_transform, mut sprite, enemy_speed) in enemy_query.iter_mut() {
        let enemy_pos = enemy_transform.translation.truncate();
        let direction = (player_pos - enemy_pos).normalize_or_zero();

        enemy_transform.translation += (direction * enemy_speed.speed * time.delta_seconds()).extend(0.0);

        sprite.flip_x = direction.x < 0.0;
    }
}