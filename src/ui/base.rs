use bevy::prelude::*;

use crate::{ui::types::*, SCREEN_HEIGHT, SCREEN_WIDTH};

const UI_BACKGROUND_COLOUR: Color = Color::rgba(0.1, 0.1, 0.1, 0.3);

/// Sets up the main UI
pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    setup_menu_background(commands, asset_server);
}

/// Sets up the background for the menu
pub fn setup_menu_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(
                    Val::Px(SCREEN_WIDTH as f32),
                    Val::Px((SCREEN_HEIGHT as f32) / 4.),
                ),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Baseline,
                ..default()
            },
            color: UI_BACKGROUND_COLOUR.into(),
            ..default()
        })
        .insert(MenuRectangle);
}
