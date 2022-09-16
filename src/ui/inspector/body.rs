use bevy::prelude::*;

use crate::ui::types::*;

fn setup_body_labels(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(
            TextBundle::from_sections([
                TextSection::new(
                    "Mass: ",
                    TextStyle {
                        font: asset_server.load("fonts/snap.otf"),
                        font_size: 28.0,
                        color: Color::GOLD,
                    },
                ),
                TextSection::new(
                    "--.--kg",
                    TextStyle {
                        font: asset_server.load("fonts/snap.otf"),
                        font_size: 28.0,
                        color: Color::WHITE,
                    },
                ),
            ])
            .with_text_alignment(TextAlignment::TOP_CENTER)
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(200.0),
                    left: Val::Px(40.0),
                    ..default()
                },
                ..default()
            }),
        )
        .insert(MassText);

    commands
        .spawn_bundle(
            TextBundle::from_sections([
                TextSection::new(
                    "Radius: ",
                    TextStyle {
                        font: asset_server.load("fonts/snap.otf"),
                        font_size: 28.0,
                        color: Color::GOLD,
                    },
                ),
                TextSection::new(
                    "--.--m",
                    TextStyle {
                        font: asset_server.load("fonts/snap.otf"),
                        font_size: 28.0,
                        color: Color::WHITE,
                    },
                ),
            ])
            .with_text_alignment(TextAlignment::TOP_CENTER)
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(140.0),
                    left: Val::Px(40.0),
                    ..default()
                },
                ..default()
            }),
        )
        .insert(RadiusText);
}
