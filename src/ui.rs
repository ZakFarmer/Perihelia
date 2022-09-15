use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use crate::spawners::NUM_BODIES;

#[derive(Component)]
pub struct MenuRectangle;

#[derive(Component)]
pub struct MenuLabel;

#[derive(Component)]
pub struct MenuButton;

#[derive(Component)]
pub struct RadiusText;

#[derive(Component)]
pub struct MassText;

#[derive(Component)]
pub struct FPSText;

#[derive(Component)]
pub struct CountText;

const SCREEN_WIDTH: usize = 1920;
const SCREEN_HEIGHT: usize = 1080;

const UI_BACKGROUND_COLOUR: Color = Color::rgba(0.1, 0.1, 0.1, 0.3);

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
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

    setup_labels(commands, asset_server);
}

fn setup_labels(mut commands: Commands, asset_server: Res<AssetServer>) {
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

pub fn update_fps_label(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FPSText>>) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.sections[1].value = format!("{average:.2}");
            }
        }
    }
}
