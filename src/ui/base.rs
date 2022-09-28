use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Color32},
    EguiContext,
};

use crate::{constants::NUM_BODIES, physics::constants::TIMESCALE};

use super::{constants::UI_LABEL_COLOUR, menu::base::show_main_menu};

/// Sets up the main UI
pub fn show_ui(mut egui_context: ResMut<EguiContext>) {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "orbitron".to_owned(),
        egui::FontData::from_static(include_bytes!("fonts/spaceregular.ttf")),
    );

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "orbitron".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("orbitron".to_owned());

    egui_context.ctx_mut().set_fonts(fonts);
}

pub fn configure_visuals(mut egui_ctx: ResMut<EguiContext>) {
    egui_ctx.ctx_mut().set_visuals(egui::Visuals {
        override_text_color: Some(Color32::WHITE),
        ..default()
    });
}
