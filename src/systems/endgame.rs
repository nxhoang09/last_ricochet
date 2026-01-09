use bevy::prelude::*;
use crate::states::AppState;
use crate::resources::game_stats::GameStats;
use crate::components::stats::PlayerStats;
use crate::components::player::Player;
use crate::resources::level::LevelManager;
use crate::resources::sound::SoundAssets;

const TITLE_COLOR: Color = Color::srgb(1.0, 0.84, 0.0);
const TEXT_COLOR: Color = Color::WHITE;
const BUTTON_NORMAL: Color = Color::srgb(0.2, 0.2, 0.2);
const BUTTON_HOVER: Color = Color::srgb(0.3, 0.3, 0.3);
const FONT_PATH: &str = "fonts/pixel_3.ttf";

#[derive(Component)]
pub struct EndgameUI;

#[derive(Component)]
pub enum EndgameButtonAction {
    Restart,
    Menu,
}

pub fn setup_game_over(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_stats: Res<GameStats>,
    player_query: Query<&PlayerStats, With<Player>>,
    sound_assets: Res<SoundAssets>,
) {
    commands.spawn(AudioBundle {
        source: sound_assets.defeated.clone(),
        settings: PlaybackSettings::DESPAWN, 
    });

    spawn_endgame_screen(commands, asset_server, game_stats, player_query, "GAME OVER", Color::srgb(1.0, 0.0, 0.0));
}

pub fn setup_victory(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_stats: Res<GameStats>,
    player_query: Query<&PlayerStats, With<Player>>,
    sound_assets: Res<SoundAssets>
) {
    commands.spawn(AudioBundle{
        source: sound_assets.victory.clone(),
        settings: PlaybackSettings::DESPAWN,
    });
    spawn_endgame_screen(commands, asset_server, game_stats, player_query, "VICTORY!", TITLE_COLOR);
}

fn spawn_endgame_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_stats: Res<GameStats>,
    player_query: Query<&PlayerStats, With<Player>>,
    title: &str,
    title_color: Color,
) {
    let font = asset_server.load(FONT_PATH);
    let board_texture = asset_server.load("sprites/ui/board.png"); 
    
    let money = if let Ok(stats) = player_query.get_single() { stats.money } else { 0 };
    
    let time = game_stats.total_time;
    let minutes = (time / 60.0).floor() as u32;
    let seconds = (time % 60.0).floor() as u32;

    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::srgba(0.0, 0.0, 0.0, 0.7).into(), 
            ..default()
        },
        EndgameUI,
    )).with_children(|parent| {

        parent.spawn(ImageBundle {
            style: Style {
                width: Val::Px(350.0),
                height: Val::Px(500.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(15.0),
                ..default()
            },
            image: UiImage::new(board_texture),
            ..default()
        }).with_children(|board| {
            board.spawn(TextBundle::from_section(
                title,
                TextStyle {
                    font: font.clone(),
                    font_size: 45.0,
                    color: title_color,
                },
            ).with_style(Style {
                margin: UiRect::bottom(Val::Px(30.0)),
                ..default()
            }));

            // STATS
            let stats_style = TextStyle {
                font: font.clone(),
                font_size: 20.0,
                color: TEXT_COLOR,
            };

            board.spawn(TextBundle::from_section(
                format!("TIME: {:02}:{:02}", minutes, seconds),
                stats_style.clone(),
            ));

            board.spawn(TextBundle::from_section(
                format!("SHOTS: {}", game_stats.shots_fired),
                stats_style.clone(),
            ));

            board.spawn(TextBundle::from_section(
                format!("MONEY: {}", money),
                stats_style.clone(),
            ));

            // BUTTONS
            spawn_button(board, &font, "RESTART", EndgameButtonAction::Restart);
            spawn_button(board, &font, "MENU", EndgameButtonAction::Menu);
        });
    });
}

fn spawn_button(
    parent: &mut ChildBuilder,
    font: &Handle<Font>,
    text: &str,
    action: EndgameButtonAction,
) {
    parent.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Px(180.0),
                height: Val::Px(45.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::top(Val::Px(10.0)),
                ..default()
            },
            background_color: BUTTON_NORMAL.into(),
            ..default()
        },
        action,
    )).with_children(|btn| {
        btn.spawn(TextBundle::from_section(
            text,
            TextStyle {
                font: font.clone(),
                font_size: 20.0,
                color: TEXT_COLOR,
            },
        ));
    });
}

pub fn endgame_action(
    mut commands: Commands,
    sound_assets: Res<SoundAssets>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &EndgameButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
    mut game_stats: ResMut<GameStats>,
    mut level_manager: ResMut<LevelManager>,
) {
    for (interaction, mut color, action) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                commands.spawn(AudioBundle {
                    source: sound_assets.select.clone(),
                    settings: PlaybackSettings::DESPAWN, 
                });
                match action {
                    EndgameButtonAction::Restart => {
                        *game_stats = GameStats::default();
                        level_manager.load_level(1); 
                        next_state.set(AppState::Playing);
                    }
                    EndgameButtonAction::Menu => {
                        next_state.set(AppState::Menu);
                    }
                }
            }
            Interaction::Hovered => *color = BUTTON_HOVER.into(),
            Interaction::None => *color = BUTTON_NORMAL.into(),
        }
    }
}

pub fn cleanup_endgame(mut commands: Commands, query: Query<Entity, With<EndgameUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}