use bevy::prelude::*;
use crate::resources::game_stats::GameStats;

pub fn update_timer(
    time: Res<Time>,
    mut stats: ResMut<GameStats>,
) {
    stats.total_time += time.delta_seconds();
}