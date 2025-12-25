use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct GameStats {
    pub total_time: f32,
    pub shots_fired: u32,
}