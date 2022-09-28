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
            ui.horizontal(|ui| {
                ui.label("File to load: ");
                if ui.button("Open file…").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        println!("File path: {:?}", path);
                        //load_file(path.display().to_string()));
                    }
                }
            });
        });
}
