use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

#[derive(Component)]
pub struct EnemySpeed {
    pub speed: f32,
}

impl Health {
    pub fn new(amount: f32) -> Self {
        Self { current: amount, max: amount }
    }
}

#[derive(Component)]
pub struct Damage {
    pub amount: f32,
}

#[derive(Component)]
pub struct EnemyAnimationTimer {
    pub timer: Timer,
}

impl Default for EnemyAnimationTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        }
    }
}