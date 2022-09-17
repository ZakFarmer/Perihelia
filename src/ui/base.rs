use bevy::prelude::*;

use super::menu::base::{setup_menu};

/// Sets up the main UI
pub fn setup_ui(commands: Commands, asset_server: Res<AssetServer>) {
    setup_menu(commands, asset_server);
}
