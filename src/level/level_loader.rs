use bevy::prelude::*;
use crate::components::collider::{Collider, Wall};
use super::level_assets::*;
use super::tile_config::TILE_CONFIGS;
use crate::resources::level::LevelManager;

#[derive(Component)]
pub struct MapEntity;

pub fn spawn_level_from_image(
    mut commands: Commands,
    assets: Res<LevelAssets>, 
    images: Res<Assets<Image>>,
    level_manager: Res<LevelManager>, 
) {
    let level_index = if level_manager.current_level > 0 {
        level_manager.current_level - 1
    } else {
        0
    };

    if level_index >= assets.map_handles.len() {
        warn!("No map found for Level {}", level_manager.current_level);
        return;
    }

    let map_handle = &assets.map_handles[level_index];
    let track_handle = &assets.track_maps[level_index];

    let Some(bg_image) = images.get(map_handle) else {
        return;
    };

    let Some(track_image) = images.get(track_handle) else {    
        return;
    };

    info!("Spawning Map for Level {}", level_manager.current_level);
    
    spawn_layer(&mut commands, &assets, bg_image);
    spawn_layer(&mut commands, &assets, track_image);
}

pub fn despawn_map(
    mut commands: Commands,
    query: Query<Entity, With<MapEntity>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    info!("Map Despawned");
}

fn spawn_layer(
    commands: &mut Commands,
    assets: &LevelAssets,
    image: &Image,
) {
    let width = image.texture_descriptor.size.width;
    let height = image.texture_descriptor.size.height;
    let data = &image.data;

    let offset_x = -(width as f32 * WORLD_TILE_SIZE) / 2.0 + WORLD_TILE_SIZE / 2.0;
    let offset_y = -(height as f32 * WORLD_TILE_SIZE) / 2.0 + WORLD_TILE_SIZE / 2.0;

    let scale = WORLD_TILE_SIZE / TILESET_SIZE;

    for y in 0..height {
        for x in 0..width {
            let i = ((y * width + x) * 4) as usize;

            if i + 3 >= data.len() || data[i + 3] == 0 {
                continue;
            }

            let tile_index = data[i] as usize;

            if tile_index >= TILE_CONFIGS.len() {
                continue;
            }

            let cfg = TILE_CONFIGS[tile_index];

            let world_x = x as f32 * WORLD_TILE_SIZE + offset_x;
            let world_y = (height - 1 - y) as f32 * WORLD_TILE_SIZE + offset_y;

            let transform = Transform::from_xyz(world_x, world_y, cfg.z)
                .with_scale(Vec3::splat(scale));

            let mut entity = commands.spawn((
                SpriteBundle {
                    texture: assets.tileset_texture.clone(),
                    transform,
                    ..default()
                },
                TextureAtlas {
                    layout: assets.tileset_layout.clone(),
                    index: tile_index,
                },
                MapEntity, 
            ));

            if cfg.has_collision {
                entity.insert((Wall, Collider::new(WORLD_TILE_SIZE, WORLD_TILE_SIZE)));
            }
        }
    }
}