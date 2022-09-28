use bevy::prelude::*;
use bevy_mod_picking::PickableBundle;
use rand::thread_rng;
use rand::Rng;

use crate::constants::NUM_BODIES;
use crate::physics::types::*;

pub fn spawn_bodies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let mesh = meshes.add(Mesh::from(shape::Icosphere {
        radius: 1.0,
        subdivisions: 3,
    }));

    let mut rng = thread_rng();

    for _i in 0..NUM_BODIES {
        let radius: f32 = rng.gen_range(0.5..1.0);
        let mass: f32 = 1000. * 10.;

        let material = materials.add(StandardMaterial {
            base_color_texture: asset_server
                .load("textures/planets/earth/surface_map.jpeg")
                .into(),
            metallic: 0.0,
            normal_map_texture: asset_server
                .load("textures/planets/earth/normal_map.jpeg")
                .into(),
            unlit: false,
            ..Default::default()
        });

        let position = Vec3::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        )
        .normalize()
            * rng.gen_range(0.2f32..1.0).powf(1. / 3.)
            * 15.;

        commands
            .spawn()
            .insert(Name::new("Body"))
            .insert_bundle(BodyBundle {
                pbr: PbrBundle {
                    transform: Transform {
                        translation: position,
                        scale: Vec3::splat(radius),
                        ..default()
                    },
                    mesh: mesh.clone(),
                    material: material,
                    ..default()
                },
                mass: Mass(mass),
                radius: Radius(radius),
                acceleration: Acceleration(Vec3::ZERO),
                angular_momentum: AngularMomentum(Vec3::ZERO),
                linear_momentum: LinearMomentum(Vec3::ZERO),
                orientation: Orientation(Quat::IDENTITY),
            })
            .insert_bundle(PickableBundle::default());
    }
}
