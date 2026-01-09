use bevy::prelude::*;
use crate::components::stats::{PlayerStats, HpText, MoneyText};
use crate::components::player::Player; 
use crate::states::AppState;
use crate::resources::level::LevelManager; 
use crate::resources::game_stats::GameStats;

const FONT_SIZE: f32 = 40.0;
const ICON_SIZE: f32 = 48.0;

const FONT_PATH: &str = "fonts/pixel_3.ttf";
const HEART_ICON_PATH: &str = "sprites/ui/hearts.png";
const COIN_ICON_PATH: &str = "sprites/ui/coin.png";

#[derive(Component)]
pub struct WaveText;

#[derive(Component)]
pub struct TimerText;

#[derive(Component)]
pub struct GameUI;

//SETUP HUD
pub fn setup_ui(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    player_query: Query<&PlayerStats, With<Player>>
) {
    let font = asset_server.load(FONT_PATH);
    let heart_icon = asset_server.load(HEART_ICON_PATH);
    let coin_icon = asset_server.load(COIN_ICON_PATH);

    let (current_hp, current_money) = if let Ok(stats) = player_query.get_single() {
        (stats.current_hp, stats.money)
    } else {
        (5.0, 10)
    };

    // STATS BÊN TRÁI
    commands
        .spawn(( 
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(20.0),
                    top: Val::Px(20.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(10.0), 
                    ..default()
                },
                ..default()
            },
            GameUI, 
        ))
        .with_children(|parent| {
            // HEALTH
            parent.spawn(NodeBundle {
                style: Style { align_items: AlignItems::Center, column_gap: Val::Px(10.0), ..default() },
                ..default()
            }).with_children(|row| {
                row.spawn(ImageBundle {
                    style: Style { width: Val::Px(ICON_SIZE), height: Val::Px(ICON_SIZE), ..default() },
                    image: UiImage::new(heart_icon.clone()), // clone handle
                    ..default()
                });
                row.spawn((
                    TextBundle::from_section(format!("{:.0}", current_hp), TextStyle { font: font.clone(), font_size: FONT_SIZE, color: Color::WHITE }),
                    HpText,
                ));
            });

            // MONEY
            parent.spawn(NodeBundle {
                style: Style { align_items: AlignItems::Center, column_gap: Val::Px(10.0), ..default() },
                ..default()
            }).with_children(|row| {
                row.spawn(ImageBundle {
                    style: Style { width: Val::Px(ICON_SIZE), height: Val::Px(ICON_SIZE), ..default() },
                    image: UiImage::new(coin_icon.clone()),
                    ..default()
                });
                row.spawn((
                    TextBundle::from_section(format!("{}", current_money), TextStyle { font: font.clone(), font_size: FONT_SIZE, color: Color::srgb(1.0, 0.84, 0.0) }),
                    MoneyText,
                ));
            });

            // WAVE INFO
            parent.spawn((
                TextBundle::from_section("Wave 1", TextStyle { font: font.clone(), font_size: 30.0, color: Color::srgb(0.0, 1.0, 1.0), }),
                WaveText,
            ));
        });

    //TIMER
    commands.spawn((
        TextBundle::from_section(
            "00:00", 
            TextStyle { font: font.clone(), font_size: 30.0, color: Color::WHITE },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),   
            right: Val::Px(20.0), 
            ..default()
        }),
        TimerText,
        GameUI, 
    ));
}

pub fn update_ui(
    player_query: Query<&PlayerStats, (With<Player>, Changed<PlayerStats>)>,
    mut hp_text_query: Query<&mut Text, (With<HpText>, Without<MoneyText>)>,
    mut money_text_query: Query<&mut Text, (With<MoneyText>, Without<HpText>)>,
) {
    if let Ok(stats) = player_query.get_single() {
        for mut text in hp_text_query.iter_mut() {
            text.sections[0].value = format!("{:.0}", stats.current_hp);
        }
        for mut text in money_text_query.iter_mut() {
            text.sections[0].value = format!("{}", stats.money);
        }
    }
}

pub fn update_wave_ui(
    level_manager: Res<LevelManager>,
    mut query: Query<&mut Text, With<WaveText>>,
) {
    if level_manager.is_changed() {
        for mut text in query.iter_mut() {
            if level_manager.level_completed {
                text.sections[0].value = "VICTORY!".to_string();
                text.sections[0].style.color = Color::srgb(0.0, 1.0, 0.0);
            } else {
                let current = level_manager.current_wave_index + 1;
                text.sections[0].value = format!("Wave {}", current);
            }
        }
    }
}

pub fn update_timer_ui(
    stats: Res<GameStats>,
    mut query: Query<&mut Text, With<TimerText>>,
) {
    for mut text in query.iter_mut() {
        let time = stats.total_time;
        let minutes = (time / 60.0).floor() as u32;
        let seconds = (time % 60.0).floor() as u32;
        text.sections[0].value = format!("{:02}:{:02}", minutes, seconds);
    }
}

pub fn check_game_over(
    player_query: Query<&PlayerStats, With<Player>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if let Ok(stats) = player_query.get_single() {
        if stats.current_hp <= 0.0 {
            info!("HP is 0! Game Over.");
            next_state.set(AppState::GameOver);
        }
    }
}

pub fn cleanup_game_ui(mut commands: Commands, query: Query<Entity, With<GameUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
