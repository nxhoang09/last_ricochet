use bevy::prelude::*;
use crate::components::item::Coin;
use crate::components::player::Player;
use crate::components::collider::Collider;
use crate::components::stats::PlayerStats;
use crate::components::enemy::{Health, Enemy};
use crate::components::bullet::Bullet;
use crate::utils::math::{check_collision, CollisionSide};
use crate::resources::game_stats::GameStats;
use crate::resources::level::LevelManager;
use crate::resources::sound::SoundAssets;

pub fn player_collect_coin(
    mut commands: Commands,
    mut player_query: Query<(&Transform, &Collider, &mut PlayerStats), With<Player>>,
    coin_query: Query<(Entity, &Transform, &Collider, &Coin), With<Coin>>,
    sound_assets: Res<SoundAssets>,
) {
    if let Ok((player_transform, player_collider, mut player_stats)) = player_query.get_single_mut() {
        for (coin_entity, coin_transform, coin_collider, coin) in coin_query.iter() {
            let collision = check_collision(
                player_transform, player_collider,
                coin_transform, coin_collider
            );

            if collision != CollisionSide::None {
                commands.spawn(AudioBundle {
                    source: sound_assets.coin.clone(),
                    settings: PlaybackSettings::DESPAWN.with_volume(bevy::audio::Volume::new(0.5)), 
                });
                player_stats.money += coin.value;
                info!("Collected Coin! Value: {} | Total Money: {}", coin.value, player_stats.money);
                
                commands.entity(coin_entity).despawn();
            }
        }
    }
}
pub fn reset_player_position(
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        transform.translation = Vec3::new(0.0, 0.0, 5.0); 
        info!("Player Position Reset");
    }
}

pub fn reset_game_state(
    mut player_query: Query<(&mut Health, &mut PlayerStats), With<Player>>,
    mut game_stats: ResMut<GameStats>,
    mut level_manager: ResMut<LevelManager>, 
) {
    if let Ok((mut health, mut stats)) = player_query.get_single_mut() {
        health.current = health.max;
        stats.current_hp = stats.max_hp;
        stats.money = 10; 
        info!("Player Stats Reset");
        stats.damage = 1.0;
    }
    *game_stats = GameStats::default();

    level_manager.load_level(1);
    
    level_manager.current_wave_index = 0;
    level_manager.enemies_spawned = 0;
    level_manager.level_completed = false;
    
    info!("Game State Fully Reset (Level 1, Wave 1)");
}

pub fn despawn_all_enemies(
    mut commands: Commands,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    for entity in enemy_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    info!("All Enemies Despawned");
}

pub fn cleanup_level_items(
    mut commands: Commands,
    bullet_query: Query<Entity, With<Bullet>>,
    coin_query: Query<Entity, With<Coin>>,
) {
    for entity in bullet_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in coin_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    info!("Bullets and Coins cleaned up");
}