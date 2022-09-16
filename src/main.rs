use crate::physics::sim::*;
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, time::FixedTimestep};
use bevy_flycam::{MovementSettings, PlayerPlugin};
use spawners::*;
use ui::debug::{setup_debug_ui, update_fps_label};

pub mod physics;
pub mod spawners;
pub mod ui;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct FixedUpdateStage;

const DELTA_TIME: f64 = 0.01;

const SCREEN_WIDTH: usize = 1920;
const SCREEN_HEIGHT: usize = 1080;

#[bevy_main]
fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Perihelia".to_string(),
            width: SCREEN_WIDTH as f32,
            height: SCREEN_HEIGHT as f32,
            present_mode: bevy::window::PresentMode::Fifo,
            mode: bevy::window::WindowMode::BorderlessFullscreen,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(AmbientLight {
            brightness: 0.03,
            ..default()
        })
        .add_startup_system(spawn_bodies)
        .add_startup_system(setup_debug_ui)
        .add_system(update_fps_label)
        .add_stage_after(
            CoreStage::Update,
            FixedUpdateStage,
            SystemStage::parallel()
                .with_run_criteria(FixedTimestep::step(DELTA_TIME))
                .with_system(attract_bodies)
                .with_system(integrate),
        )
        .insert_resource(ClearColor(Color::hex("141414").unwrap()))
        .insert_resource(MovementSettings {
            sensitivity: 0.00015,
            speed: 12.0,
        })
        .run();
}
