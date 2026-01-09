use bevy::prelude::*;
use bevy::window::WindowResolution;

mod states;
mod components; 
mod systems;    
mod utils;
mod level;
mod resources;
mod vfx;

use vfx::VfxPlugin;
use states::AppState;
use components::player::{Player, MovementStats};
use components::stats::PlayerStats;
use components::collider::Collider;
use components::enemy::Health; 

use resources::level::LevelManager;
use resources::game_stats::GameStats;
use resources::sound::{setup_sound_assets, start_background_music_when_ready, stop_background_music};

use systems::{
    aura::{spawn_aura, aura_visual_system, aura_logic_system},
    movement::player_movement,
    ui::{setup_ui, update_ui, check_game_over, update_wave_ui, update_timer_ui, cleanup_game_ui}, 
    shooting::{setup_bullet_assets, spawn_bullet, move_and_animate_bullet, cleanup_bullet_bounds},
    ricochet::{bullet_ricochet, player_wall_collision, enemy_player_collision, enemy_wall_collision, bullet_enemy_collision},
    enemy_ai::{enemy_movement, animate_enemies},
    particle::update_particles,
    gameplay::{player_collect_coin, reset_player_position, reset_game_state, despawn_all_enemies, cleanup_level_items},
    timer::update_timer,
    wave::wave_system,
    menu::{setup_menu, menu_action, cleanup_menu},
    endgame::{setup_game_over, setup_victory, endgame_action, cleanup_endgame},
    shop::{setup_shop, shop_interaction, shop_next_level, cleanup_shop}, 
};

use level::level_assets::{setup_level_assets, setup_game_assets};
use level::level_loader::{spawn_level_from_image, despawn_map}; 

const PLAYER_SPRITE_PATH: &str = "sprites/player/player.png";
const TILE_SIZE: f32 = 48.0; 

fn main() {
    App::new()
        // PLUGINS & WINDOW 
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()).set(WindowPlugin {
            primary_window: Some(Window {
                title: "Last Ricochet".into(),
                resolution: WindowResolution::new(1280.0, 720.0),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(VfxPlugin)

        // STATES & RESOURCES
        .init_state::<AppState>()
        .init_resource::<LevelManager>()
        .init_resource::<GameStats>()
        
        // STARTUP
        .add_systems(Startup, (setup, setup_bullet_assets, setup_level_assets, setup_game_assets, setup_sound_assets))

        // STATE: MENU
        .add_systems(OnEnter(AppState::Menu), setup_menu)
        .add_systems(Update, menu_action.run_if(in_state(AppState::Menu)))
        .add_systems(
        Update,
        start_background_music_when_ready.run_if(in_state(AppState::Menu))
        )
        .add_systems(OnExit(AppState::Menu), (cleanup_menu, reset_game_state))
        
        // STATE: PLAYING
        .add_systems(OnEnter(AppState::Playing), (spawn_level_from_image, setup_ui, reset_player_position))
        .add_systems(OnExit(AppState::Playing), (despawn_map, cleanup_game_ui, despawn_all_enemies, cleanup_level_items))

        // Physics (Playing)
        .add_systems(
            FixedUpdate, 
            (
                player_movement, 
                player_wall_collision,
                enemy_movement,
                enemy_wall_collision,
            ).chain()
                .run_if(in_state(AppState::Playing))
        )

        // Logic Game (Playing)
        .add_systems(
            Update,
            (
                update_ui,       
                check_game_over,
                update_wave_ui,
                update_timer,  
                update_timer_ui,
                spawn_bullet,
                move_and_animate_bullet,
                cleanup_bullet_bounds,
                bullet_ricochet,
                bullet_enemy_collision, 
                enemy_player_collision,
                spawn_aura,
                aura_visual_system,
                aura_logic_system,
                animate_enemies,
                wave_system,
                player_collect_coin,
                update_particles,
            ).run_if(in_state(AppState::Playing))
        )

        // STATE: BUFF SCREEN
        .add_systems(OnEnter(AppState::BuffScreen), (setup_shop, despawn_map, setup_ui))
        
        .add_systems(
            Update,
            (
                player_movement,     
                shop_interaction,  
                shop_next_level,   
                update_ui,     
            ).run_if(in_state(AppState::BuffScreen))
        )

        .add_systems(OnExit(AppState::BuffScreen), (cleanup_shop, cleanup_game_ui))

        // STATE: VICTORY
        .add_systems(OnEnter(AppState::Victory), (setup_victory, stop_background_music)) 
        .add_systems(Update, endgame_action.run_if(in_state(AppState::Victory)))
        .add_systems(OnExit(AppState::Victory), (cleanup_endgame, reset_game_state)) 

        //STATE: GAME OVER 
        .add_systems(OnEnter(AppState::GameOver), (setup_game_over, stop_background_music))
        .add_systems(Update, endgame_action.run_if(in_state(AppState::GameOver)))
        .add_systems(OnExit(AppState::GameOver), (cleanup_endgame, reset_game_state))

        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2dBundle::default());

    let texture = asset_server.load(PLAYER_SPRITE_PATH);
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE as u32), 6, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout); 
    
    commands.spawn((
        SpriteBundle {
            texture, 
            transform: Transform::from_xyz(0.0, 0.0, 5.0).with_scale(Vec3::splat(2.0)), 
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout,
            index: 0,
        },
        Player,
        MovementStats::default(),
        PlayerStats::default(),
        Health::new(5.0), 
        Collider::new(30.0, 33.0),
    ));
}