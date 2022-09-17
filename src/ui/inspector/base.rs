use bevy::prelude::*;

use crate::constants::*;
use crate::ui::constants::*;
use crate::ui::types::*;

pub fn setup_inspector(mut commands: Commands, asset_server: Res<AssetServer>) {
    setup_inspector_background(commands, asset_server);
}

/// Sets up the background for the inspector
pub fn setup_inspector_background(mut commands: Commands, _asset_server: Res<AssetServer>) {
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
