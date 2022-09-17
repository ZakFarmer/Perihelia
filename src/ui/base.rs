use bevy::prelude::*;

use super::menu::base::{root, setup_menu};

/// Sets up the main UI
pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    setup_menu(commands, asset_server);
}
