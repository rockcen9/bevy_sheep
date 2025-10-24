use bevy::{audio::PlaybackMode, prelude::*};

pub struct AmbientPlugin;

#[derive(Resource)]
pub struct ForestAmbient;

impl Plugin for AmbientPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup);
    }
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioPlayer::<AudioSource>(asset_server.load("audio/forest.ogg")),
        PlaybackSettings {
            mode: PlaybackMode::Loop,
            ..Default::default()
        },
    ));

    commands.spawn((
        AudioPlayer::<AudioSource>(asset_server.load("audio/main-theme.ogg")),
        PlaybackSettings {
            mode: PlaybackMode::Loop,
            ..Default::default()
        },
    ));

    commands.spawn((
        AudioPlayer::<AudioSource>(asset_server.load("audio/sheep.ogg")),
        PlaybackSettings {
            mode: PlaybackMode::Loop,
            ..Default::default()
        },
    ));
}
