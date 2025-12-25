use bevy::prelude::*;
use rand::Rng;

#[derive(Event)]
pub struct ScreenShakeEvent {
    pub intensity: f32,
    pub duration: f32,
}

#[derive(Component)]
pub struct CameraShaker {
    pub intensity: f32,
    pub timer: Timer,
    pub original_pos: Option<Vec3>,
}

#[derive(Component)]
pub struct HitFlash {
    pub timer: Timer,
    pub original_color: Color,
}

fn handle_screen_shake(
    mut commands: Commands,
    mut events: EventReader<ScreenShakeEvent>,
    mut camera_query: Query<(Entity, &mut Transform, Option<&mut CameraShaker>), With<Camera2d>>,
    time: Res<Time>,
) {
    let mut rng = rand::thread_rng();
    for (camera_entity, mut transform, shaker_opt) in camera_query.iter_mut() {
        for event in events.read() {
            commands.entity(camera_entity).insert(CameraShaker {
                intensity: event.intensity,
                timer: Timer::from_seconds(event.duration, TimerMode::Once),
                original_pos: None,
            });
        }
        
        // Thực hiện rung
        if let Some(mut shaker) = shaker_opt {
            if shaker.original_pos.is_none() {
                shaker.original_pos = Some(transform.translation);
            }
            shaker.timer.tick(time.delta());
            
            if shaker.timer.finished() {
                if let Some(org_pos) = shaker.original_pos {
                    transform.translation = org_pos;
                }
                commands.entity(camera_entity).remove::<CameraShaker>();
            } else {
                let x = rng.gen_range(-shaker.intensity..shaker.intensity);
                let y = rng.gen_range(-shaker.intensity..shaker.intensity);
                if let Some(org_pos) = shaker.original_pos {
                    transform.translation = org_pos + Vec3::new(x, y, 0.0);
                }
            }
        }
    }
}

pub fn update_hit_flash(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Sprite, &mut HitFlash)>, 
) {
    for (entity, mut sprite, mut flash) in query.iter_mut() {
        flash.timer.tick(time.delta());

        if flash.timer.finished() {
            sprite.color = flash.original_color; 
            commands.entity(entity).remove::<HitFlash>();
        }
    }
}

pub struct VfxPlugin;

impl Plugin for VfxPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ScreenShakeEvent>()
           .add_systems(Update, (
               handle_screen_shake,
               update_hit_flash
           ));
    }
}