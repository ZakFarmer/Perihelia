use crate::constants::*;
use crate::physics::sim::*;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, time::FixedTimestep};
use bevy_egui::EguiPlugin;
use bevy_flycam::MovementSettings;

use camera::{
    base::setup_camera,
    constants::{CAMERA_LOOK_SENSITIVITY, CAMERA_MOVE_SPEED},
    systems::{camera_movement_system, mouse_motion_system},
};

use input::{base::esc_to_menu, cursor_grab::cursor_grab_system};
use physics::{
    constants::DELTA_TIME,
    types::{Acceleration, BodyBundle, LinearMomentum, PhysicsBody},
};
use spawners::*;
use state::base::SimState;
use ui::{
    base::{configure_visuals, show_ui},
    debug::*,
    inspector::{base::inspector_panel_system, types::OccupiedScreenSpace},
};
use wasm_bindgen::prelude::*;

pub mod audio;
pub mod camera;
pub mod constants;
pub mod graphics;
pub mod input;
pub mod physics;
pub mod spawners;
pub mod state;
pub mod ui;
pub mod utils;
pub mod world;

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
    .init_resource::<OccupiedScreenSpace>()
    .init_resource::<PhysicsBody>()
    .init_resource::<BodyBundle>()
    .init_resource::<Acceleration>()
    .init_resource::<LinearMomentum>()
    .add_plugins(DefaultPlugins)
    .add_plugin(EguiPlugin)
    .add_plugin(FrameTimeDiagnosticsPlugin::default())
    .add_state(SimState::SimRunning)
    .insert_resource(AmbientLight {
        brightness: 0.5,
        ..default()
    })
    .insert_resource(Msaa { samples: 4 })
    //.add_startup_system(setup_audio)
    .add_system(inspector_panel_system)
    .add_startup_system(configure_visuals)
    .add_startup_system(setup_camera)
    .add_startup_system(spawn_bodies)
    //.add_startup_system(setup_debug_ui)
    //.add_startup_system(write_save.exclusive_system().at_end())
    .add_system(cursor_grab_system)
    .add_system(esc_to_menu)
    .add_system(show_ui)
    .add_system(camera_movement_system)
    .add_system(mouse_motion_system)
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
        sensitivity: CAMERA_LOOK_SENSITIVITY,
        speed: CAMERA_MOVE_SPEED,
    });

    #[cfg(target_arch = "wasm32")]
    web(&mut app);

    app.run();
}

#[cfg(target_arch = "wasm32")]
fn web(app: &mut App) {
    app.add_plugin(web_canvas_resizer::WebCanvasResizerPlugin);
}
