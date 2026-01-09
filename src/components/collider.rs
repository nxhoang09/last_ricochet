use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Collider {
    pub half_size: Vec2,
}

impl Collider {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            half_size: Vec2::new(width / 2.0, height / 2.0),
        }
    }
}

#[derive(Component)]
pub struct Wall;