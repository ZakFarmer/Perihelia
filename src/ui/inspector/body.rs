use bevy::prelude::*;

use crate::{
    constants::{SCREEN_HEIGHT, SCREEN_WIDTH},
    physics::types::Mass,
    physics::types::{PhysicsBody, Radius},
    ui::{
        constants::{LABEL_FONT_SIZE, MAIN_FONT_PATH},
        types::*,
    },
};

use crate::ui::inspector::types::*;

pub fn setup_body_labels(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(
            TextBundle::from_sections([TextSection::new(
                "Mass (kg)\nRadius (m)\nPosition (m)\nLin. Velocity (m/s)",
                TextStyle {
                    font: asset_server.load(MAIN_FONT_PATH),
                    font_size: LABEL_FONT_SIZE,
                    color: Color::GOLD,
                },
            )])
            .with_style(Style {
                align_content: AlignContent::FlexStart,
                align_self: AlignSelf::Auto,
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::RowReverse,
                position: UiRect {
                    bottom: Val::Px(SCREEN_HEIGHT as f32 / 6. - 5.),
                    right: Val::Px(SCREEN_WIDTH as f32 / 4.),
                    ..default()
                },
                ..default()
            }),
        )
        .insert(MassLabelText);
}

pub fn setup_body_values(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(
            TextBundle::from_sections([TextSection::new(
                "Mass (kg)\nRadius (m)\nPosition (m)\nLin. Velocity (m/s)",
                TextStyle {
                    font: asset_server.load(MAIN_FONT_PATH),
                    font_size: LABEL_FONT_SIZE,
                    color: Color::GOLD,
                },
            )])
            .with_style(Style {
                align_content: AlignContent::FlexStart,
                align_self: AlignSelf::Auto,
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::RowReverse,
                position: UiRect {
                    bottom: Val::Px(SCREEN_HEIGHT as f32 / 6. - 5.),
                    right: Val::Px(SCREEN_WIDTH as f32 / 4.),
                    ..default()
                },
                ..default()
            }),
        )
        .insert(MassLabelText);
}
