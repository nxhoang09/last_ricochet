use bevy::prelude::*;

#[derive(Component)]
pub struct Bullet {
    pub direction: Vec2,
    pub speed: f32,
    pub lifetime: Timer,
    pub animation_timer: Timer,
    pub is_despawning: bool,
    pub hit_entities: Vec<Entity>,
    pub damage: f32,
}

impl Default for Bullet {
    fn default() -> Self {
        Self {
            direction: Vec2::X,
            speed: 400.0,
            lifetime: Timer::from_seconds(5.0, TimerMode::Once),
            animation_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            is_despawning: false,
            hit_entities: Vec::new(),
            damage: 1.0,
        }
    }
}