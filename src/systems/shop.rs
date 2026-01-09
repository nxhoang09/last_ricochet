use bevy::prelude::*;
use crate::components::player::Player;
use crate::components::stats::PlayerStats;
use crate::components::enemy::Health;
use crate::resources::level::LevelManager;
use crate::resources::sound::SoundAssets;
use crate::states::AppState;

#[derive(Component)]
pub struct ShopUI;

#[derive(Component)]
pub struct ShopItem {
    pub name: String,
    pub cost: u32,
    pub description: String,
    pub buff_type: BuffType,
}

#[derive(Clone, Copy)]
pub enum BuffType {
    Heal,
    DamageUp,
}

#[derive(Component)]
pub struct ShopInfoText;

#[derive(Component)]
pub struct NextLevelZone;

const FONT_PATH: &str = "fonts/pixel_3.ttf";
const SHOP_ITEM_SCALE: f32 = 1.2; 
const SHOP_TABLE_Y: f32 = -120.0; 

pub fn setup_shop(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    let font = asset_server.load(FONT_PATH);
    let bg_texture = asset_server.load("sprites/ui/shop_bg.png"); 
    if let Ok(mut transform) = player_query.get_single_mut() {
        transform.translation = Vec3::new(0.0, -250.0, 5.0);
    }

    commands.spawn((
        SpriteBundle {
            texture: bg_texture,
            sprite: Sprite{
                custom_size: Some(Vec2::new(1280.0, 720.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, -10.0), 
            ..default()
        },
        ShopUI,
    ));

    spawn_shop_item(
        &mut commands, 
        &asset_server, 
        Vec3::new(-220.0, SHOP_TABLE_Y, 1.0),  
        "sprites/ui/hp.png", 
        BuffType::Heal
    );

    spawn_shop_item(
        &mut commands, 
        &asset_server, 
        Vec3::new(220.0, SHOP_TABLE_Y, 1.0),
        "sprites/ui/increase.png", 
        BuffType::DamageUp
    );

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.0, 1.0, 0.0, 0.3), 
                custom_size: Some(Vec2::new(120.0, 80.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 165.0, 1.0), 
            ..default()
        },
        NextLevelZone,
        ShopUI,
    )).with_children(|parent| {
        parent.spawn(Text2dBundle {
            text: Text::from_section("NEXT LEVEL", TextStyle {
                font: font.clone(),
                font_size: 18.0,
                color: Color::WHITE,
            }),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        });
    });

    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font: font.clone(),
                font_size: 30.0,
                color: Color::WHITE, 
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(50.0), 
            left: Val::Auto,
            right: Val::Auto,
            align_self: AlignSelf::Center,
            ..default()
        }),
        ShopInfoText,
        ShopUI,
    ));
}

fn spawn_shop_item(
    commands: &mut Commands, 
    asset_server: &Res<AssetServer>, 
    pos: Vec3, 
    texture_path: &str,
    buff_type: BuffType
) {
    let (name, cost, desc) = match buff_type {
        BuffType::Heal => ("Healing Potion", 2, "+1 HP"),
        BuffType::DamageUp => ("Power Elixir", 4, "+1 Damage"),
    };

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(texture_path.to_string()),
            transform: Transform::from_translation(pos).with_scale(Vec3::splat(SHOP_ITEM_SCALE)),
            ..default()
        },
        ShopItem {
            name: name.to_string(),
            cost,
            description: desc.to_string(),
            buff_type,
        },
        ShopUI,
    ));
}

pub fn shop_interaction(
    mut commands: Commands,
    sound_assets: Res<SoundAssets>,
    player_query: Query<&Transform, With<Player>>,
    mut stats_query: Query<(&mut PlayerStats, &mut Health), With<Player>>,
    item_query: Query<(Entity, &Transform, &ShopItem)>,
    mut text_query: Query<&mut Text, With<ShopInfoText>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let player_transform = player_query.single();
    let (mut stats, mut health) = stats_query.single_mut();
    let mut info_text = text_query.single_mut();

    let mut near_any_item = false;

    for (_, item_transform, item) in item_query.iter() {
        let distance = player_transform.translation.distance(item_transform.translation);

        if distance < 60.0 { 
            near_any_item = true;
            
            info_text.sections[0].value = format!(
                "{} - Cost: ${}\nEffect: {}\n[PRESS SPACE TO BUY]", 
                item.name, item.cost, item.description
            );
            
            info_text.sections[0].style.color = if stats.money >= item.cost {
                 Color::srgb(0.0, 1.0, 0.0) 
            } else {
                 Color::srgb(1.0, 0.0, 0.0) 
            };

            if keyboard.just_pressed(KeyCode::Space) {
                if stats.money >= item.cost {
                    commands.spawn(AudioBundle{
                        source: sound_assets.item.clone(),
                        settings: PlaybackSettings::DESPAWN,
                    });
                    stats.money -= item.cost;
                    
                    match item.buff_type {
                        BuffType::Heal => {
                            health.current += 1.0;
                            stats.current_hp +=1.0;
                        }
                        BuffType::DamageUp => {
                            stats.damage += 1.0;
                        }
                    }
                    
                    info_text.sections[0].value = "PURCHASE SUCCESSFUL!".to_string();
                } else {
                    info_text.sections[0].value = "NOT ENOUGH MONEY!".to_string();
                }
            }
        }
    }

    if !near_any_item {
        info_text.sections[0].value = "".to_string(); 
    }
}

pub fn shop_next_level(
    player_query: Query<&Transform, With<Player>>,
    zone_query: Query<&Transform, With<NextLevelZone>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut level_manager: ResMut<LevelManager>,
) {
    let player = player_query.single();
    if let Ok(zone) = zone_query.get_single() {
        if player.translation.distance(zone.translation) < 60.0 {
            level_manager.next_level();
            next_state.set(AppState::Playing);
        }
    }
}

pub fn cleanup_shop(mut commands: Commands, query: Query<Entity, With<ShopUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}