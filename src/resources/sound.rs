use bevy::prelude::*;
use bevy::asset::LoadState;

#[derive(Resource)]
pub struct SoundAssets {
    pub select: Handle<AudioSource>,
    pub coin: Handle<AudioSource>,
    pub item: Handle<AudioSource>,
    pub defeated: Handle<AudioSource>,
    pub victory: Handle<AudioSource>,

    pub bgm: Handle<AudioSource>,

    pub bounce: Handle<AudioSource>,
    pub shoot: Handle<AudioSource>,
    pub hurt: Handle<AudioSource>,
}

#[derive(Component)]
pub struct BackgroundMusic;

pub fn setup_sound_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(SoundAssets {
        select: asset_server.load("sounds/select.wav"), 
        coin: asset_server.load("sounds/coin.wav"),
        item: asset_server.load("sounds/item.wav"),
        defeated: asset_server.load("sounds/defeated.wav"),
        victory: asset_server.load("sounds/victory.wav"),

        bgm: asset_server.load("sounds/bgm.mp3"),

        bounce: asset_server.load("sounds/bounce.wav"),
        shoot: asset_server.load("sounds/shoot.wav"),
        hurt: asset_server.load("sounds/hurt.wav"),


    });
}


pub fn start_background_music_when_ready(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    sound_assets: Res<SoundAssets>,
    music_query: Query<Entity, With<BackgroundMusic>>,
) {
    if !music_query.is_empty() {
        return;
    }

    if !matches!(
        asset_server.get_load_state(&sound_assets.bgm),
        Some(LoadState::Loaded)
    ) {
        return;
    }

    commands.spawn((
        AudioBundle {
            source: sound_assets.bgm.clone(),
            settings: PlaybackSettings::LOOP
                .with_volume(bevy::audio::Volume::new(0.25)),
        },
        BackgroundMusic,
    ));
}

pub fn stop_background_music(
    mut commands: Commands,
    music_query: Query<Entity, With<BackgroundMusic>>,
) {
    for entity in music_query.iter() {
        commands.entity(entity).despawn();
    }
}