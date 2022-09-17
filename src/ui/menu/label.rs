use bevy::prelude::*;

use crate::ui::constants::{LABEL_FONT_SIZE, MAIN_FONT_PATH, UI_LABEL_COLOUR};

pub fn label(asset_server: &Res<AssetServer>, label: &str) -> TextBundle {
    TextBundle::from_sections([TextSection::new(
        label,
        TextStyle {
            font: asset_server.load(MAIN_FONT_PATH),
            font_size: LABEL_FONT_SIZE,
            color: Color::GOLD,
        },
    )])
    .with_style(Style {
        margin: UiRect::all(Val::Px(10.)),
        ..default()
    })
}
