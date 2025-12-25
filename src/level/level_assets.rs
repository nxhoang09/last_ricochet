use bevy::prelude::*;

pub const WORLD_TILE_SIZE: f32 = 48.0;
pub const TILESET_SIZE: f32 = 16.0;
pub const TILESET_COLS: usize = 7;
pub const TILESET_ROWS: usize = 4;

pub const TILESET_PATH: &str = "sprites/tiles/tileset.png";

#[derive(Resource)]
pub struct LevelAssets {
    // Sửa thành Vec để chứa nhiều map
    pub map_handles: Vec<Handle<Image>>, 
    pub track_maps: Vec<Handle<Image>>,
    
    pub tileset_texture: Handle<Image>,
    pub tileset_layout: Handle<TextureAtlasLayout>,
}

pub fn setup_level_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let map_handles = vec![
        asset_server.load("maps/level_1.png"),
        asset_server.load("maps/level_1.png"),
        asset_server.load("maps/level_3.png"),
    ];

    let track_maps = vec![
        asset_server.load("maps/level_1_track.png"),
        asset_server.load("maps/level_2_track.png"),
        asset_server.load("maps/level_3_track.png"),
    ];

    let texture = asset_server.load(TILESET_PATH);

    let layout = TextureAtlasLayout::from_grid(
        UVec2::splat(TILESET_SIZE as u32),
        TILESET_COLS as u32,
        TILESET_ROWS as u32,
        None,
        None,
    );

    let layout_handle = layouts.add(layout);

    commands.insert_resource(LevelAssets {
        map_handles,
        track_maps,
        tileset_texture: texture,
        tileset_layout: layout_handle,
    });
}


#[derive(Resource)]
pub struct GameAssets {
    pub coin_texture: Handle<Image>,
}

pub fn setup_game_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let coin_texture = asset_server.load("sprites/ui/coin.png");

    commands.insert_resource(GameAssets {
        coin_texture,
    });
}