use std::{
    error::Error,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    time::SystemTime,
};

use bevy::{prelude::*, reflect::TypeRegistryArc, tasks::IoTaskPool};

use crate::body::{Acceleration, AngularMomentum, BodyBundle, LinearMomentum, Mass, Radius};

const SAVES_FILE_PATH: &str = "saves";
const SAVE_FILENAME: &str = "og-save.ron";

fn drop_empty(scene: DynamicScene) -> DynamicScene {
    let new_entities = scene
        .entities
        .into_iter()
        .filter(|entity| entity.components.len() > 0);

    DynamicScene {
        entities: new_entities.collect(),
    }
}

pub fn load_save(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(DynamicSceneBundle {
        scene: asset_server.load(&format!("{}/{}", SAVES_FILE_PATH, SAVE_FILENAME)),
        ..default()
    });

    asset_server.watch_for_changes().unwrap();
}

pub fn write_save(world: &mut World) {
    println!("{:?}", world);
    let type_registry = world.get_resource::<TypeRegistryArc>().unwrap();
    type_registry.write().register::<BodyBundle>();
    type_registry.write().register::<Mass>();
    type_registry.write().register::<Radius>();
    type_registry.write().register::<Acceleration>();
    type_registry.write().register::<AngularMomentum>();
    type_registry.write().register::<LinearMomentum>();

    println!("{:?}", type_registry);

    let scene = DynamicScene::from_world(world, &type_registry);

    let scene = drop_empty(scene);

    let serialized_scene = scene.serialize_ron(&type_registry).unwrap();

    info!("{}", serialized_scene);

    let path: String = format!(
        "{}/{}",
        SAVES_FILE_PATH,
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );

    #[cfg(not(target_arch = "wasm32"))]
    IoTaskPool::get()
        .spawn(async move {
            File::create(format!("{}.ron", path))
                .and_then(|mut file| file.write(serialized_scene.as_bytes()))
                .expect("Error while writing scene to file");
        })
        .detach();
}
