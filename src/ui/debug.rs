use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use crate::spawners::NUM_BODIES;
use crate::ui::types::*;

/// Sets up the debug UI
pub fn setup_debug_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    setup_debug_labels(commands, asset_server);
}

/// Sets up the labels for the debug UI
fn setup_debug_labels(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(
            TextBundle::from_sections([
                TextSection::new(
                    "FPS: ",
                    TextStyle {
                        font: asset_server.load("fonts/snap.otf"),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                ),
                TextSection::from_style(TextStyle {
                    font: asset_server.load("fonts/snap.otf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                }),
            ])
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(5.0),
                    left: Val::Px(15.0),
                    ..default()
                },
                ..default()
            }),
        )
        .insert(FPSText);

    commands
        .spawn_bundle(
            TextBundle::from_sections([
                TextSection::new(
                    "Count: ",
                    TextStyle {
                        font: asset_server.load("fonts/snap.otf"),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                ),
                TextSection::new(
                    format!("{}", NUM_BODIES),
                    TextStyle {
                        font: asset_server.load("fonts/snap.otf"),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                ),
            ])
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(25.0),
                    left: Val::Px(15.0),
                    ..default()
                },
                ..default()
            }),
        )
        .insert(CountText);
}

/// Updates the FPS label's value with the current FPS
pub fn update_fps_label(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FPSText>>) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.sections[1].value = format!("{average:.2}");
            }
        }
    }
}
