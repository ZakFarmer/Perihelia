use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use super::types::OccupiedScreenSpace;

pub fn setup_inspector(_commands: Commands, _asset_server: Res<AssetServer>) {
    /*commands
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
    .insert(MenuRectangle);*/

    //setup_body_labels(commands, asset_server);
}

pub fn inspector_panel_system(
    mut egui_context: ResMut<EguiContext>,
    mut occupied_screen_space: ResMut<OccupiedScreenSpace>,
) {
    occupied_screen_space.right = egui::SidePanel::right("right_panel")
        .resizable(true)
        .show(egui_context.ctx_mut(), |ui| {
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
            ui.label("world");
        })
        .response
        .rect
        .width();
}
