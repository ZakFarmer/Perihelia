use bevy::prelude::*;
use rand::thread_rng;
use rand::Rng;

use crate::body::*;

const NUM_BODIES: usize = 200;

pub fn spawn_bodies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(shape::Icosphere {
        radius: 1.0,
        subdivisions: 3,
    }));

    let mut rng = thread_rng();

    for _ in 0..NUM_BODIES {
        let radius: f32 = rng.gen_range(0.05..0.1);
        let mass: f32 = 1000. * 10.;

        let position = Vec3::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        )
        .normalize()
            * rng.gen_range(0.2f32..1.0).powf(1. / 3.)
            * 15.;

        commands
            .spawn_bundle(BodyBundle {
                pbr: PbrBundle {
                    transform: Transform {
                        translation: position,
                        scale: Vec3::splat(radius),
                        ..default()
                    },
                    mesh: mesh.clone(),
                    material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
                    ..default()
                },
                mass: Mass(mass),
                acceleration: Acceleration(Vec3::ZERO),
                angular_velocity: AngularVelocity(Vec3::ZERO),
                linear_velocity: LinearVelocity(Vec3::ZERO),
            })
            .with_children(|p| {
                p.spawn_bundle(PointLightBundle {
                    point_light: PointLight {
                        color: Color::WHITE,
                        intensity: 10.0,
                        range: 10.0,
                        radius: 1.,
                        ..default()
                    },
                    ..default()
                });
            });
    }
}

pub fn spawn_black_hole(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(shape::Icosphere {
        radius: 1.0,
        subdivisions: 3,
    }));

    let bh_radius: f32 = 1.;

    commands
        .spawn_bundle(BodyBundle {
            pbr: PbrBundle {
                transform: Transform {
                    translation: Vec3::ZERO,
                    scale: Vec3::splat(2.0),
                    ..default()
                },
                mesh: mesh.clone(),
                material: materials.add(Color::rgb(0.0, 0.0, 0.0).into()),
                ..default()
            },
            mass: Mass(500.),
            acceleration: Acceleration(Vec3::ZERO),
            angular_velocity: AngularVelocity(Vec3::ZERO),
            linear_velocity: LinearVelocity(Vec3::ZERO),
        })
        .insert(BlackHole);
}
