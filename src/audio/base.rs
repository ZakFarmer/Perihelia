use bevy::prelude::*;
use bevy_kira_audio::prelude::Audio;

use super::music::start_background_music;

pub fn setup_audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    start_background_music(asset_server, audio);
}
