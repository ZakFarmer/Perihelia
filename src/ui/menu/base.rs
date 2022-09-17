use bevy::prelude::*;

use crate::constants::*;
use crate::ui::constants::*;
use crate::ui::inspector::base::setup_inspector;





pub fn root() -> NodeBundle {
    NodeBundle {
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
    }
}

pub fn setup_menu(commands: Commands, asset_server: Res<AssetServer>) {
    /*commands.spawn_bundle(root()).with_children(|parent| {
        parent.spawn_bundle(vstack()).with_children(|parent| {
            parent.spawn_bundle(label(&asset_server, "Fuck yeah!"));
            parent.spawn_bundle(label(&asset_server, "Fuck yeah! ^ 2"));
        });
    });*/

    setup_inspector(commands, asset_server);
}
