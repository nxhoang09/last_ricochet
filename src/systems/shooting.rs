use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::components::player::Player;
use crate::components::stats::PlayerStats;
use crate::components::bullet::Bullet;
use crate::components::collider::Collider;
use crate::resources::game_stats::GameStats;
use crate::resources::sound::SoundAssets;

const BULLET_SPRITE_PATH: &str = "sprites/bullet/bullet.png";
const BULLET_SIZE: f32 = 16.0;
const BULLET_COST: u32 = 1;
const BULLET_FRAMES: usize = 4;

#[derive(Resource)]
pub struct BulletAssets {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

pub fn setup_bullet_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load(BULLET_SPRITE_PATH);
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(BULLET_SIZE as u32), BULLET_FRAMES as u32, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.insert_resource(BulletAssets {
        texture,
        layout: texture_atlas_layout,
    });
}

pub fn spawn_bullet(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut player_query: Query<(&Transform, &mut PlayerStats), With<Player>>,
    bullet_query: Query<&Bullet>,
    bullet_assets: Res<BulletAssets>,
    mut game_stats: ResMut<GameStats>,
    sound_assets: Res<SoundAssets>
) {

    if !bullet_query.is_empty() { return; }

    let (camera, camera_transform) = camera_query.single();
    let window = window_query.single();

    if let Ok((player_transform, mut stats)) = player_query.get_single_mut() {
        if stats.money < BULLET_COST { return; }

        if mouse_input.just_pressed(MouseButton::Left) {
            commands.spawn(AudioBundle{
                source: sound_assets.shoot.clone(),
                settings: PlaybackSettings:: DESPAWN
            });
            game_stats.shots_fired += 1;
            if let Some(world_position) = window
                .cursor_position()
                .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
            {
                stats.money -= BULLET_COST;
                let damage = stats.damage; 

                let player_pos = player_transform.translation.truncate();
                let direction = (world_position - player_pos).normalize_or_zero();

                commands.spawn((
                    SpriteBundle {
                        texture: bullet_assets.texture.clone(),
                        transform: Transform::from_translation(player_pos.extend(5.0))
                            .with_scale(Vec3::splat(2.0)),
                        ..default()
                    },
                    TextureAtlas {
                        layout: bullet_assets.layout.clone(),
                        index: 0,
                    },
                    Bullet::default().into_builder(direction, damage),
                    Collider::new(12.0, 12.0),
                ));
            }
        }
    }
}

impl Bullet {
    fn into_builder(mut self, direction: Vec2, damage: f32) -> Self {
        self.direction = direction;
        self.damage = damage;
        self
    }
}

pub fn move_and_animate_bullet(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut TextureAtlas, &mut Bullet)>,
) {
    for (entity, mut transform, mut atlas, mut bullet) in query.iter_mut() {
        transform.translation += bullet.direction.extend(0.0) * bullet.speed * time.delta_seconds();

        if !bullet.is_despawning {
            bullet.lifetime.tick(time.delta());
            
            atlas.index = 0; 

            if bullet.lifetime.finished() {
                bullet.is_despawning = true;
            }
        } else {
            bullet.animation_timer.tick(time.delta());
            
            if bullet.animation_timer.just_finished() {
                if atlas.index < BULLET_FRAMES - 1 {
                    atlas.index += 1;
                } else {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}

const MAP_WIDTH: f32 = 1280.0;
const MAP_HEIGHT: f32 = 720.0;

pub fn cleanup_bullet_bounds(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Bullet>>,
) {
    let half_width = MAP_WIDTH / 2.0;
    let half_height = MAP_HEIGHT / 2.0;

    for (entity, transform) in query.iter() {
        let pos = transform.translation;
        if pos.x < -half_width || pos.x > half_width || pos.y < -half_height || pos.y > half_height {
            commands.entity(entity).despawn();
        }
    }
}