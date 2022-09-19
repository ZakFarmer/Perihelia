use bevy_egui::{
    egui::{menu, Ui},
    EguiContext,
};

use bevy::prelude::*;

use super::{file::load::show_load_menu, state::MenuState};

pub fn show_main_menu(ui: &mut Ui, mut menu_state: ResMut<State<MenuState>>) {
    menu::bar(ui, |ui| {
        ui.menu_button("File", |ui| {
            if ui.button("Load").clicked() {
                menu_state.set(MenuState::FileLoad).unwrap();
            }

            if ui.button("Save").clicked() {}
            if ui.button("Exit").clicked() {}
        });

        ui.menu_button("About", |ui| {
            if ui.button("Version").clicked() {}
            if ui.button("Credits").clicked() {}
            if ui.button("Open-Source Libraries").clicked() {}
        });

        ui.menu_button("Settings", |ui| {
            if ui.button("Audio").clicked() {}
            if ui.button("Appearance").clicked() {}
            if ui.button("Debug").clicked() {}
            if ui.button("Filesystem").clicked() {}
            if ui.button("Network").clicked() {}
            if ui.button("Physics").clicked() {}
        });

        ui.menu_button("Online", |ui| {
            if ui.button("Profile").clicked() {}
            if ui.button("Create Server").clicked() {}
            if ui.button("Connect to IP").clicked() {}
            if ui.button("Server List").clicked() {}
        });

        ui.menu_button("Quit", |ui| {});
    });
}

pub fn show_dialog_menu(egui_context: ResMut<EguiContext>, menu_state: Res<State<MenuState>>) {
    match menu_state.current() {
        MenuState::FileLoad => {
            show_load_menu(egui_context);
        }
        _ => (),
    }
}
