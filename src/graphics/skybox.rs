use bevy::prelude::*;
use bevy::render::texture::CompressedImageFormats;

use super::constants::SKYBOX_CUBEMAP;
use super::cubemap::Cubemap;

pub fn setup_skybox(mut commands: Commands, asset_server: Res<AssetServer>) {
    let skybox_handle: Handle<Image> = asset_server.load(SKYBOX_CUBEMAP.0);

    commands.insert_resource(Cubemap {
        is_loaded: false,
        index: 1,
        image_handle: skybox_handle,
    });
}
