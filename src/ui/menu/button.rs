use bevy::prelude::*;

use crate::ui::constants::UI_BUTTON_COLOUR;

pub fn button() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(125.0), Val::Px(50.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        color: UI_BUTTON_COLOUR.into(),
        ..default()
    }
}
