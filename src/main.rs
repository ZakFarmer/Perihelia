use crate::constants::*;
use crate::physics::sim::*;

use audio::base::setup_audio;
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, time::FixedTimestep};
use bevy_flycam::{MovementSettings, PlayerPlugin};
use bevy_kira_audio::AudioPlugin;
use physics::constants::DELTA_TIME;
use spawners::*;
use ui::debug::*;
use wasm_bindgen::prelude::*;

pub mod audio;
pub mod constants;
pub mod input;
pub mod physics;
pub mod spawners;
pub mod ui;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct FixedUpdateStage;

#[cfg(target_arch = "wasm32")]
mod web_canvas_resizer;

fn main() {
    start();
}

#[wasm_bindgen]
pub fn start() {
    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
        title: "Perihelia".to_string(),
        width: SCREEN_WIDTH as f32,
        height: SCREEN_HEIGHT as f32,
        present_mode: bevy::window::PresentMode::Fifo,
        mode: bevy::window::WindowMode::BorderlessFullscreen,
        ..default()
    })
    .add_plugins(DefaultPlugins)
    //.add_plugin(AudioPlugin)
    .add_plugin(PlayerPlugin)
    .add_plugin(FrameTimeDiagnosticsPlugin::default())
    .insert_resource(AmbientLight {
        brightness: 0.03,
        ..default()
    })
    //.add_startup_system(setup_audio)
    .add_startup_system(spawn_bodies)
    .add_startup_system(setup_debug_ui)
    //.add_startup_system(setup_ui)
    .add_system(update_dt_label)
    .add_system(update_fps_label)
    .add_stage_after(
        CoreStage::Update,
        FixedUpdateStage,
        SystemStage::parallel()
            .with_run_criteria(FixedTimestep::step(DELTA_TIME as f64))
            .with_system(attract_bodies)
            .with_system(integrate),
    )
    .insert_resource(ClearColor(Color::hex("141414").unwrap()))
    .insert_resource(MovementSettings {
        sensitivity: 0.00015,
        speed: 12.0,
    });

    #[cfg(target_arch = "wasm32")]
    web(&mut app);

    app.run();
}

#[cfg(target_arch = "wasm32")]
fn web(app: &mut App) {
    app.add_plugin(web_canvas_resizer::WebCanvasResizerPlugin);
}
