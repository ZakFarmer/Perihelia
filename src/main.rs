use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, time::FixedTimestep, window::WindowMode,
};
use bevy_flycam::{MovementSettings, PlayerPlugin};
use bevy_mod_picking::*;
use bevy_web_fullscreen::FullViewportPlugin;
use body::BodyBundle;
use physics::*;
use spawners::*;
use ui::{setup_ui, update_fps_label};
use world::save::write_save;

pub mod body;
pub mod collision;
pub mod physics;
pub mod spawners;
pub mod ui;
pub mod utils;
pub mod world;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct FixedUpdateStage;

const DELTA_TIME: f64 = 0.01;

const SCREEN_WIDTH: usize = 1280;
const SCREEN_HEIGHT: usize = 720;

#[bevy_main]
fn main() {
    let mut world = World::new();

    App::new()
        .init_resource::<BodyBundle>()
        .insert_resource(WindowDescriptor {
            title: "Perihelia".to_string(),
            width: SCREEN_WIDTH as f32,
            height: SCREEN_HEIGHT as f32,
            present_mode: bevy::window::PresentMode::Fifo,
            mode: WindowMode::Windowed,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        //.add_plugin(FullViewportPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(AmbientLight {
            brightness: 0.03,
            ..default()
        })
        .add_startup_system(spawn_bodies)
        .add_startup_system(setup_ui)
        .add_system(write_save.exclusive_system().at_end())
        //.add_startup_system(spawn_black_hole)
        //.add_startup_system(spawn_star)
        .add_system(update_fps_label)
        .add_stage_after(
            CoreStage::Update,
            FixedUpdateStage,
            SystemStage::parallel()
                .with_run_criteria(FixedTimestep::step(DELTA_TIME))
                .with_system(attract_bodies)
                .with_system(integrate),
        )
        //.add_system(look_at_center)
        .insert_resource(ClearColor(Color::hex("141414").unwrap()))
        .insert_resource(MovementSettings {
            sensitivity: 0.00015, // default: 0.00012
            speed: 12.0,          // default: 12.0
        })
        .run()
}
