use bevy::prelude::*;
use rand::{Rng, thread_rng}; 
use std::f32::consts::TAU;
use crate::components::bullet::Bullet;
use crate::components::collider::{Collider, Wall};
use crate::components::item::Coin;
use crate::components::player::Player;
use crate::components::enemy::{Enemy, Damage, Health};
use crate::components::stats::PlayerStats;
use crate::components::particle::{Particle, Velocity, Lifetime};
use crate::level::level_assets::GameAssets;
use crate::resources::sound::SoundAssets;
use crate::utils::math::{check_collision, CollisionSide};
use crate::vfx::{HitFlash, ScreenShakeEvent};

const PARTICLE_COUNT: usize = 10;  
const PARTICLE_SIZE: f32 = 6.0;    
const PARTICLE_SPEED_MIN: f32 = 50.0;  
const PARTICLE_SPEED_MAX: f32 = 150.0;
const PARTICLE_LIFETIME: f32 = 0.5;
const DROP_RATE: f64 = 0.5;

pub fn bullet_ricochet(
    mut commands: Commands,
    sound_assets: Res<SoundAssets>,
    mut bullet_query: Query<(&mut Transform, &Collider, &mut Bullet), Without<Wall>>,
    wall_query: Query<(&Transform, &Collider), With<Wall>>,
) {
    for (mut bullet_transform, bullet_collider, mut bullet) in bullet_query.iter_mut() {
        let mut has_reflected = false;
        
        for (wall_transform, wall_collider) in wall_query.iter() {
            if has_reflected {
                break; 
            }
            
            let collision = check_collision(
                &bullet_transform, 
                bullet_collider, 
                wall_transform, 
                wall_collider
            );

            if collision == CollisionSide::None {
                continue;
            }

            commands.spawn(AudioBundle {
                source: sound_assets.bounce.clone(), 
                settings: PlaybackSettings::DESPAWN,   
            });

            let offset_x = wall_collider.half_size.x + bullet_collider.half_size.x + 1.0;
            let offset_y = wall_collider.half_size.y + bullet_collider.half_size.y + 1.0;

            match collision {
                CollisionSide::Left => {
                    bullet.direction.x = -bullet.direction.x.abs();
                    bullet_transform.translation.x = wall_transform.translation.x - offset_x;
                },
                CollisionSide::Right => {
                    bullet.direction.x = bullet.direction.x.abs();
                    bullet_transform.translation.x = wall_transform.translation.x + offset_x;
                },
                CollisionSide::Top => {
                    bullet.direction.y = bullet.direction.y.abs(); 
                    bullet_transform.translation.y = wall_transform.translation.y + offset_y;
                },
                CollisionSide::Bottom => {
                    bullet.direction.y = -bullet.direction.y.abs(); 
                    bullet_transform.translation.y = wall_transform.translation.y - offset_y;
                },
                CollisionSide::None => {}
            }
            bullet.hit_entities.clear();
            has_reflected = true;
        }
    }
}
pub fn player_wall_collision(
    mut player_query: Query<(&mut Transform, &Collider), (With<Player>, Without<Wall>)>,
    wall_query: Query<(&Transform, &Collider), With<Wall>>,
) {
    if let Ok((mut player_transform, player_collider)) = player_query.get_single_mut() {
        for (wall_transform, wall_collider) in wall_query.iter() {
            let collision = check_collision(
                &player_transform,
                player_collider,
                wall_transform,
                wall_collider,
            );

            if collision == CollisionSide::None {
                continue;
            }

            let offset_x = wall_collider.half_size.x + player_collider.half_size.x + 2.0;
            let offset_y = wall_collider.half_size.y + player_collider.half_size.y + 2.0;

            match collision {
                CollisionSide::Left => {
                    player_transform.translation.x = wall_transform.translation.x - offset_x;
                },
                CollisionSide::Right => {
                    player_transform.translation.x = wall_transform.translation.x + offset_x;
                },
                CollisionSide::Top => {
                    player_transform.translation.y = wall_transform.translation.y + offset_y;
                },
                CollisionSide::Bottom => {
                    player_transform.translation.y = wall_transform.translation.y - offset_y;
                },
                CollisionSide::None => {}
            }
            
        }
    }
}

pub fn enemy_wall_collision(
    mut enemy_query: Query<(&mut Transform, &Collider), (With<Enemy>, Without<Wall>)>,
    wall_query: Query<(&Transform, &Collider), With<Wall>>,
) {
    for (mut enemy_transform, enemy_collider) in enemy_query.iter_mut() {
        for (wall_transform, wall_collider) in wall_query.iter() {
            let collision = check_collision(
                &enemy_transform,
                enemy_collider,
                wall_transform,
                wall_collider,
            );

            if collision == CollisionSide::None {
                continue;
            }

            let offset_x = wall_collider.half_size.x + enemy_collider.half_size.x + 2.0;
            let offset_y = wall_collider.half_size.y + enemy_collider.half_size.y + 2.0;

            match collision {
                CollisionSide::Left => {
                    enemy_transform.translation.x = wall_transform.translation.x - offset_x;
                },
                CollisionSide::Right => {
                    enemy_transform.translation.x = wall_transform.translation.x + offset_x;
                },
                CollisionSide::Top => {
                    enemy_transform.translation.y = wall_transform.translation.y + offset_y;
                },
                CollisionSide::Bottom => {
                    enemy_transform.translation.y = wall_transform.translation.y - offset_y;
                },
                CollisionSide::None => {}
            }
        }
    }
}

pub fn bullet_enemy_collision(
    mut commands: Commands,
    mut bullet_query: Query<(Entity, &Transform, &Collider, &mut Bullet), With<Bullet>>,
    mut enemy_query: Query<(Entity, &Transform, &Collider, &mut Health, &mut Sprite, Option<&mut HitFlash>), With<Enemy>>,
    mut ev_shake: EventWriter<ScreenShakeEvent>,
    game_assets: Res<GameAssets>,
    sound_assets: Res<SoundAssets>
) {
    let mut rng = rand::thread_rng();
    
    for (_, bullet_transform, bullet_collider, mut bullet) in bullet_query.iter_mut() {
        bullet.hit_entities.retain(|&enemy_id| {
            if let Ok((_, enemy_transform, enemy_collider, _, _, _)) = enemy_query.get(enemy_id) {
                let collision = check_collision(
                    bullet_transform, 
                    bullet_collider, 
                    enemy_transform, 
                    enemy_collider
                );
                collision != CollisionSide::None 
            } else {
                false 
            }
        });

        for (enemy_entity, enemy_transform, enemy_collider, mut enemy_health, mut sprite, existing_flash) in enemy_query.iter_mut() {
            
            if bullet.hit_entities.contains(&enemy_entity) {
                continue;
            }
            
            let collision = check_collision(
                bullet_transform,
                bullet_collider,
                enemy_transform,
                enemy_collider,
            );

            if collision != CollisionSide::None {
                bullet.hit_entities.push(enemy_entity);

                enemy_health.current -= bullet.damage;
                println!("Enemy Hit! HP: {}/{}", enemy_health.current, enemy_health.max);
                if let Some(mut flash) = existing_flash {
                    flash.timer.reset(); 
                } 
                else {
                    let original_color = sprite.color; 
                    
                    sprite.color = Color::srgb(1.0, 0.0, 0.0); 

                    commands.entity(enemy_entity).insert(HitFlash {
                        timer: Timer::from_seconds(0.1, TimerMode::Once),
                        original_color,
                    });
                }

                if enemy_health.current <= 0.0 {
                    ev_shake.send(ScreenShakeEvent { 
                        intensity: 2.0, 
                        duration: 0.1 
                    });
                    commands.spawn(AudioBundle {
                        source: sound_assets.hurt.clone(),
                        settings: PlaybackSettings::DESPAWN, 
                    });
                    spawn_death_particles(&mut commands, enemy_transform.translation);
                    commands.entity(enemy_entity).despawn_recursive();
                    if rng.gen_bool(DROP_RATE) {
                        let coin_transform = Transform::from_translation(enemy_transform.translation)
                            .with_scale(Vec3::splat(1.0));
                        commands.spawn((
                            SpriteBundle {
                                texture: game_assets.coin_texture.clone(),
                                transform: coin_transform,
                                sprite: Sprite {
                                    custom_size: Some(Vec2::splat(24.0)), 
                                    ..default()
                                },
                                ..default()
                            },
                            Coin { value: 1 }, 
                            Collider::new(24.0, 24.0),
                        ));
                    }
                    println!("Enemy Killed!");
                }
                
            }
        }
    }
}

pub fn enemy_player_collision(
    mut commands: Commands,
    mut player_query: Query<(&Transform, &Collider, &mut Health, &mut PlayerStats), With<Player>>, 
    mut ev_shake: EventWriter<ScreenShakeEvent>,
    enemy_query: Query<(Entity, &Transform, &Collider, &Damage), With<Enemy>>,
    sound_assets: Res<SoundAssets>,
    game_assets: Res<GameAssets>
) {
    let mut rng = rand::thread_rng();


    if let Ok((player_transform, player_collider, mut player_health, mut player_stats)) = player_query.get_single_mut() {
        
        for (enemy_entity, enemy_transform, enemy_collider, damage) in enemy_query.iter() {
            let collision = check_collision(
                player_transform,
                player_collider,
                enemy_transform,
                enemy_collider,
            );

            if collision != CollisionSide::None {
                commands.spawn(AudioBundle {
                    source: sound_assets.hurt.clone(),
                    settings: PlaybackSettings::DESPAWN, 
                });
                ev_shake.send(ScreenShakeEvent { 
                    intensity: 3.0, 
                    duration: 0.1 
                });
                player_health.current -= damage.amount;
                player_stats.current_hp -= damage.amount;

                if rng.gen_bool(DROP_RATE){
                    let coin_transform = Transform::from_translation(enemy_transform.translation)
                        .with_scale(Vec3::splat(1.0));

                    commands.spawn((
                        SpriteBundle {
                            texture: game_assets.coin_texture.clone(),
                            transform: coin_transform,
                            sprite: Sprite {
                                custom_size: Some(Vec2::splat(24.0)), 
                                ..default()
                            },
                            ..default()
                        },
                        Coin { value: 1 }, 
                        Collider::new(24.0, 24.0),
                    ));
                } 

                commands.entity(enemy_entity).despawn_recursive();

                if player_health.current <= 0.0 {
                    println!("GAME OVER");
                    player_stats.current_hp = 0.0;
                }
            }
        }
    }
}

fn spawn_death_particles(commands: &mut Commands, position: Vec3) {
    let mut rng = thread_rng();

    for _ in 0..PARTICLE_COUNT {
        let angle = rng.gen_range(0.0..TAU);
        let speed = rng.gen_range(PARTICLE_SPEED_MIN..PARTICLE_SPEED_MAX);
        
        let velocity = Vec2::new(angle.cos(), angle.sin()) * speed;

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE, 
                    custom_size: Some(Vec2::splat(PARTICLE_SIZE)),
                    ..default()
                },
                transform: Transform::from_translation(position), 
                ..default()
            },
            Particle,
            Velocity(velocity),
            Lifetime::new(PARTICLE_LIFETIME), 
        ));
    }
}

// pub fn draw_colliders(
//     mut gizmos: Gizmos,
//     player_query: Query<(&Transform, &Collider), With<Player>>,
//     wall_query: Query<(&Transform, &Collider), With<Wall>>,
//     aura_query: Query<&GlobalTransform, With<Aura>>,
//     enemy_query: Query<(&Transform, &Collider), With<Enemy>>,
// ) {
  
//     for (transform, collider) in player_query.iter() {
//         gizmos.rect_2d(
//             transform.translation.truncate(),
//             0.0,
//             collider.size,
//             Color::srgb(0.0, 1.0, 0.0), // Xanh lá
//         );
//     }
    
//     for (transform, collider) in wall_query.iter() {
//         gizmos.rect_2d(
//             transform.translation.truncate(),
//             0.0,
//             collider.size,
//             Color::srgb(1.0, 0.0, 0.0), // Đỏ
//         );
//     }

//     const AURA_RADIUS: f32 = 60.0;
//     for global_transform in aura_query.iter() {
//         gizmos.circle_2d(
//             global_transform.translation().truncate(),
//             AURA_RADIUS,
//             Color::srgb(0.0, 1.0, 1.0), // Cyan
//         );
//     }
//     for (transform, collider) in enemy_query.iter() {
//         gizmos.rect_2d(
//             transform.translation.truncate(),
//             0.0,
//             collider.size,
//             Color::srgb(1.0, 0.5, 0.5), 
//         );
//     }
// }