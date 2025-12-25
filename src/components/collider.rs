use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Collider {
    pub size: Vec2,
    pub half_size: Vec2,
}

impl Collider {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            size: Vec2::new(width, height),
            half_size: Vec2::new(width / 2.0, height / 2.0),
        }
    }
}

#[derive(Component)]
pub struct Wall;