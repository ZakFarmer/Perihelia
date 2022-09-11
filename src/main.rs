use bevy::{prelude::*, time::FixedTimestep};
use bevy_flycam::{MovementSettings, PlayerPlugin};
use physics::*;
use spawners::*;

pub mod body;
pub mod collision;
pub mod physics;
pub mod spawners;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct FixedUpdateStage;

const DELTA_TIME: f64 = 0.01;

#[bevy_main]
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .insert_resource(AmbientLight {
            brightness: 0.03,
            ..default()
        })
        .add_startup_system(spawn_bodies)
        .add_startup_system(spawn_black_hole)
        //.add_startup_system(spawn_star)
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
        .run();
}
