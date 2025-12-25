use bevy::prelude::*;

#[derive(Component)]
pub struct Particle;

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Lifetime {
    pub timer: Timer,
}

impl Lifetime {
    pub fn new(seconds: f32) -> Self {
        Self {
            timer: Timer::from_seconds(seconds, TimerMode::Once),
        }
    }
}