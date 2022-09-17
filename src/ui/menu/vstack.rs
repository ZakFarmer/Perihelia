use bevy::prelude::*;

use crate::ui::constants::UI_BACKGROUND_COLOUR;

pub fn vstack() -> NodeBundle {
    NodeBundle {
        style: Style {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Baseline,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        color: UI_BACKGROUND_COLOUR.into(),
        ..default()
    }
}
