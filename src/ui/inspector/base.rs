use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Color32},
    EguiContext,
};

use crate::{
    constants::NUM_BODIES,
    physics::constants::TIMESCALE,
    state::types::{SimSettings, SimState},
    ui::menu::{base::show_main_menu, state::MenuState},
};

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
    sim_settings: Res<SimSettings>,
    sim_state: Res<State<SimState>>,
    time: Res<Time>,
    mut menu_state: ResMut<State<MenuState>>,
    mut egui_context: ResMut<EguiContext>,
    mut occupied_screen_space: ResMut<OccupiedScreenSpace>,
) {
    occupied_screen_space.right = egui::SidePanel::right("right_panel")
        .resizable(true)
        .show(egui_context.ctx_mut(), |ui| {
            ui.heading(format!("Particle Count: {}", NUM_BODIES));
            ui.heading(format!("Timescale: {}", sim_settings.timescale));
            ui.heading(format!(
                "FPS: {}",
                (1000.0 / time.delta().as_millis() as f64).round()
            ));
            ui.heading(format!("State: {:?}", sim_state.current()));

            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();

    occupied_screen_space.top = egui::TopBottomPanel::top("top_panel")
        .resizable(true)
        .show(egui_context.ctx_mut(), |ui| {
            show_main_menu(ui, menu_state);
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .height();
}
