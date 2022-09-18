use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::{constants::NUM_BODIES, physics::constants::TIMESCALE};

use super::menu::base::setup_menu;

/// Sets up the main UI
pub fn setup_ui(mut egui_context: ResMut<EguiContext>) {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "monofonto".to_owned(),
        egui::FontData::from_static(include_bytes!("fonts/monofonto.otf")),
    );

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "monofonto".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("monofonto".to_owned());

    egui_context.ctx_mut().set_fonts(fonts);

    egui::Window::new("World Details").show(egui_context.ctx_mut(), |ui| {
        ui.heading(format!("Particle Count: {}", NUM_BODIES));
        ui.heading(format!("Timescale: {}", TIMESCALE));
        ui.heading(format!("FPS: {}", 1.0));
    });
}
