use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct MovementStats {
    pub speed: f32,
}

impl Default for MovementStats {
    fn default() -> Self {
        Self {
            speed: 250.0, 
        }
    }
}

#[derive(Component)]
pub struct Aura;

#[derive(Component)]
pub struct HasAura;