use bevy::prelude::*;
use bevy_kira_audio::prelude::Audio;
use bevy_kira_audio::AudioControl;

pub fn start_background_music(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio
        .play(asset_server.load("music/goingdown.mp3"))
        .looped();
}
