use bevy::prelude::*;
use crate::components::particle::{Particle, Velocity, Lifetime};

pub fn update_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &Velocity, &mut Lifetime), With<Particle>>,
) {
    for (entity, mut transform, velocity, mut lifetime) in query.iter_mut() {
        transform.translation += (velocity.0 * time.delta_seconds()).extend(0.0);

        lifetime.timer.tick(time.delta());
        if lifetime.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}