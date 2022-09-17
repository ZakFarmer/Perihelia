use bevy::prelude::*;

use crate::ui::constants::UI_BACKGROUND_COLOUR;

pub fn hstack() -> NodeBundle {
    NodeBundle {
        style: Style {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Baseline,
            flex_direction: FlexDirection::Row,
            ..default()
        },
        color: UI_BACKGROUND_COLOUR.into(),
        ..default()
    }
}
