use crate::constants::*;
use crate::physics::sim::*;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, time::FixedTimestep};
use bevy_egui::{egui::plot::Line, EguiPlugin};
use bevy_flycam::MovementSettings;

use bevy_inspector_egui::{InspectorPlugin, RegisterInspectable, WorldInspectorPlugin};
use bevy_mod_picking::DefaultPickingPlugins;
use camera::{
    base::setup_camera,
    constants::{CAMERA_LOOK_SENSITIVITY, CAMERA_MOVE_SPEED},
    systems::{camera_movement_system, mouse_motion_system},
};

use input::{
    base::esc_to_menu, cursor_grab::cursor_grab_system, timescale::change_timescale_system,
};
use physics::{
    constants::{DELTA_TIME, TIMESCALE},
    types::{Acceleration, AngularMomentum, BodyBundle, LinearMomentum, Mass, PhysicsBody, Radius},
};
use spawners::{base::spawn_bodies, *};
use state::{
    base::setup_state,
    types::{SimSettings, SimState},
};

use ui::{
    base::{configure_visuals, show_ui},
    debug::*,
    inspector::{base::inspector_panel_system, types::OccupiedScreenSpace},
    menu::{base::show_dialog_menu, state::MenuState},
};
use wasm_bindgen::prelude::*;
use world::save::write_save;

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
    .init_resource::<BodyBundle>()
    .init_resource::<PhysicsBody>()
    .init_resource::<BodyBundle>()
    .init_resource::<Acceleration>()
    .init_resource::<LinearMomentum>()
    .add_plugins(DefaultPlugins)
    .add_plugins(DefaultPickingPlugins)
    .add_plugin(EguiPlugin)
    .add_plugin(WorldInspectorPlugin::new())
    .add_plugin(FrameTimeDiagnosticsPlugin::default())
    .add_state(MenuState::None)
    .add_state(SimState::SimRunning)
    .register_inspectable::<Acceleration>()
    .register_inspectable::<AngularMomentum>()
    .register_inspectable::<LinearMomentum>()
    .register_inspectable::<Mass>()
    .register_inspectable::<Radius>()
    .insert_resource(AmbientLight {
        brightness: 1.0,
        ..default()
    })
    .insert_resource(Msaa { samples: 4 })
    //.add_startup_system(setup_audio)
    .add_system(inspector_panel_system)
    .add_startup_system(setup_state)
    .add_startup_system(configure_visuals)
    .add_startup_system(setup_camera)
    .add_startup_system(spawn_bodies)
    //.add_startup_system(setup_debug_ui)
    .add_startup_system(write_save.exclusive_system())
    //.add_system(cursor_grab_system)
    .add_system(esc_to_menu)
    .add_system(show_dialog_menu)
    .add_system(show_ui)
    .add_system(camera_movement_system)
    .add_system(change_timescale_system)
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
    .insert_resource(SimSettings {
        timescale: TIMESCALE,
    })
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
