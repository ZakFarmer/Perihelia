use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Ui},
    EguiContext,
};

use crate::ui::menu::state::MenuState;

pub fn show_load_menu(mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("Load World")
        .vscroll(true)
        .show(egui_context.ctx_mut(), |ui| {
            ui.label("Load World");
        });
}
