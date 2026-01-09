use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::components::player::{Player, Aura, HasAura};
use crate::components::bullet::Bullet;

const AURA_RADIUS: f32 = 60.0;             
const AURA_DIAMETER: f32 = AURA_RADIUS * 2.2; 

const AURA_PATH: &str = "sprites/ui/aura.png";
const AURA_SCALE: f32 = 0.5;

const ALPHA_NORMAL: f32 = 0.05; 
const ALPHA_ACTIVE: f32 = 0.2; 

const AURA_COLOR: Color = Color::srgb(0.0, 1.0, 1.0);

const BULLET_SPEED_SLOW: f32 = 50.0;
const BULLET_SPEED_KICK: f32 = 400.0; 

const ROTATE_SPEED_NORMAL: f32 = 1.5; 
const ROTATE_SPEED_ACTIVE: f32 = 10.0;

#[derive(Component)]
pub struct AuraVisual;

pub fn spawn_aura(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_query: Query<Entity, (With<Player>, Without<HasAura>)>,
) {
    if let Ok(player_entity) = player_query.get_single() {
        let aura = commands.spawn((
            SpriteBundle {
                texture: asset_server.load(AURA_PATH),
                transform: Transform::from_xyz(0.0, 0.0, -1.0)
                    .with_scale(Vec3::splat(AURA_SCALE)),
                sprite: Sprite {
                    color: AURA_COLOR.with_alpha(ALPHA_NORMAL),
                    custom_size: Some(Vec2::splat(AURA_DIAMETER)),
                    ..default()
                },
                ..default()
            },
            Aura,
            AuraVisual,
        )).id();

        commands.entity(player_entity)
            .add_child(aura)
            .insert(HasAura);
    }
}

pub fn aura_visual_system(
    time: Res<Time>, 
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut aura_query: Query<(&mut Sprite, &mut Transform), With<AuraVisual>>,
) {
    let is_active =
        keyboard_input.pressed(KeyCode::ShiftLeft) ||
        keyboard_input.pressed(KeyCode::Space);

    if let Ok((mut sprite, mut transform)) = aura_query.get_single_mut() {
        let alpha = if is_active { ALPHA_ACTIVE } else { ALPHA_NORMAL };
        sprite.color.set_alpha(alpha);

        let rotate_speed = if is_active { ROTATE_SPEED_ACTIVE } else { ROTATE_SPEED_NORMAL };
        transform.rotate_z(rotate_speed * time.delta_seconds());
    }
}

pub fn aura_logic_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,

    player_query: Query<&Transform, With<Player>>,
    mut bullet_query: Query<(&Transform, &mut Bullet)>,
) {
    let Ok(player_transform) = player_query.get_single() else { return };
    let player_pos = player_transform.translation.truncate();

    let is_slowing = keyboard_input.pressed(KeyCode::ShiftLeft);
    let is_kicking = keyboard_input.just_pressed(KeyCode::Space);

    let (camera, camera_transform) = camera_query.single();
    let window = window_query.single();
    let cursor_world = window
        .cursor_position()
        .and_then(|p| camera.viewport_to_world_2d(camera_transform, p));

    for (bullet_transform, mut bullet) in bullet_query.iter_mut() {
        let bullet_pos = bullet_transform.translation.truncate();

        if player_pos.distance(bullet_pos) > AURA_RADIUS {
            continue;
        }

        if is_kicking {
            if let Some(target) = cursor_world {
                bullet.direction = (target - player_pos).normalize_or_zero();
                bullet.speed = BULLET_SPEED_KICK;
                bullet.lifetime.reset();
                bullet.is_despawning = false;
                
                continue; 
            }
        }
        if is_slowing {
            bullet.speed = BULLET_SPEED_SLOW;
        }
    }
}