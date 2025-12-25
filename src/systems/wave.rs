use bevy::prelude::*;
use rand::Rng;
use crate::components::enemy::{Enemy, Health, Damage, EnemyAnimationTimer, EnemySpeed};
use crate::components::collider::Collider;
use crate::resources::level::LevelManager;
use crate::resources::game_config::EnemyType; 
use crate::states::AppState;


const TILE_SIZE: f32 = 48.0;
const MAP_COLS: f32 = 28.0;
const MAP_ROWS: f32 = 15.0;
const GOLEM_SIZE: f32 = 64.0;
const GOLEM_HITBOX: f32 = 28.0;
const MAX_LEVEL: usize = 3;


pub fn wave_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    time: Res<Time>,
    mut level_manager: ResMut<LevelManager>,
    mut next_state: ResMut<NextState<AppState>>, 
    live_enemies: Query<Entity, With<Enemy>>,
    mut level_finish_timer: Local<Option<Timer>>,
) {
    if level_manager.level_completed {
        return;
    }

   
   let current_wave_data = level_manager.current_waves_data[level_manager.current_wave_index].clone();
    
    let all_spawned = level_manager.enemies_spawned >= current_wave_data.enemy_count;
    let no_enemies_alive = live_enemies.iter().count() == 0;

    if all_spawned && no_enemies_alive {
        if level_manager.current_wave_index < level_manager.current_waves_data.len() - 1 {
            level_manager.current_wave_index += 1;
            level_manager.enemies_spawned = 0;
            level_manager.spawn_timer.reset();
            info!("Next Wave Started!");
        } else {
            if level_finish_timer.is_none() {
                info!("Level Clear! Loot time 3s...");
                *level_finish_timer = Some(Timer::from_seconds(3.0, TimerMode::Once));
            }
            if let Some(timer) = level_finish_timer.as_mut() {
                timer.tick(time.delta());

                if timer.finished() {
                    level_manager.level_completed = true;
                    
                    if level_manager.current_level >= MAX_LEVEL {
                        info!("VICTORY! ALL LEVELS COMPLETED.");
                        next_state.set(AppState::Victory); 
                    } else {
                        info!("LEVEL {} COMPLETED! Switching to Buff Screen...", level_manager.current_level);
                        next_state.set(AppState::BuffScreen);
                    }
                    *level_finish_timer = None;
                }
            }
        }
        return;
    }

    if !all_spawned {
        level_manager.spawn_timer.tick(time.delta());

        if level_manager.spawn_timer.just_finished() {
            let interval = current_wave_data.spawn_interval;
            level_manager.spawn_timer.set_duration(std::time::Duration::from_secs_f32(interval));
            level_manager.spawn_timer.reset();

            spawn_single_enemy(
                &mut commands, 
                &asset_server, 
                &mut texture_atlas_layouts, 
                &level_manager, 
                current_wave_data.enemy_type 
            );
            
            level_manager.enemies_spawned += 1;
        }
    }
}

// HÀM SPAWN CHI TIẾT
fn spawn_single_enemy(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    level_manager: &LevelManager,
    enemy_type: EnemyType,
) {
    let mut rng = rand::thread_rng();

    // 1. Tính Stats dựa trên Multiplier và Enemy Type
    let multiplier = level_manager.difficulty_multiplier;
    
    let (base_hp, base_speed, color_tint, scale) = match enemy_type {
        EnemyType::Normal => (2.0, 100.0, Color::WHITE,1.5),
        EnemyType::Tank => (3.0, 40.0, Color::srgb(0.5, 0.5, 1.0),3.0),   
        EnemyType::Speed => (1.0, 180.0, Color::srgb(1.0, 0.5, 0.5),1.0), 
    };

    let final_hp = base_hp * multiplier; 

    let range_x = (MAP_COLS * TILE_SIZE) / 2.0 - (TILE_SIZE * 2.5);
    let range_y = (MAP_ROWS * TILE_SIZE) / 2.0 - (TILE_SIZE * 2.5);
    let corners = [
        Vec2::new(-range_x, range_y), Vec2::new(range_x, range_y),
        Vec2::new(-range_x, -range_y), Vec2::new(range_x, -range_y),
    ];
    let spawn_pos = corners[rng.gen_range(0..4)];
    let jitter_x = rng.gen_range(-20.0..20.0);
    let jitter_y = rng.gen_range(-20.0..20.0);

    let texture = asset_server.load("sprites/enemy/Golem_Armor_Run.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(GOLEM_SIZE as u32), 4, 1, None, None);
    let texture_atlas_layout = layouts.add(layout);

    commands.spawn((
        SpriteBundle {
            texture,
            transform: Transform::from_xyz(spawn_pos.x + jitter_x, spawn_pos.y + jitter_y, 1.0)
                .with_scale(Vec3::splat(scale)),
            sprite: Sprite {
                color: color_tint, 
                ..default()
            },
            ..default()
        },
        TextureAtlas { layout: texture_atlas_layout, index: 0 },
        EnemyAnimationTimer::default(),
        Enemy,
        Health::new(final_hp),
        Damage { amount: 1.0 },
        Collider::new(GOLEM_HITBOX*scale, GOLEM_HITBOX*scale),
        EnemySpeed { speed: base_speed },
    ));
}