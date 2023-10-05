use bevy::prelude::*;

use crate::asset_manager::resources::AssetType;

use super::resources::{AssetsLoaded, AssetsToLoad};

pub fn start_assets(asset_server: Res<AssetServer>, mut assets_to_load: ResMut<AssetsToLoad>) {
    let wall: Handle<Scene> = asset_server.load("./models/wall.gltf#Scene0");
    assets_to_load.insert_asset(
        "models/building/wall",
        AssetType::Scene(wall.clone()),
        "./models/wall.gltf#Scene0",
    );

    let window: Handle<Scene> = asset_server.load("./models/window.gltf#Scene0");
    assets_to_load.insert_asset(
        "models/building/window",
        AssetType::Scene(window.clone()),
        "./models/window.gltf#Scene0",
    );
}

pub fn check_assets_ready(
    server: Res<AssetServer>,
    mut assets_to_load: ResMut<AssetsToLoad>,
    mut assets_loaded: ResMut<AssetsLoaded>,
) {
    use bevy::asset::LoadState;
    let mut assets_to_remove: Vec<String> = Vec::new();

    for (name, asset) in &assets_to_load.assets {
        match server.get_load_state(asset.data.handle_id()) {
            LoadState::Failed => {
                panic!("Asset from path: {} failed to load.", asset.path);
            }
            LoadState::Loaded => {
                info!("Asset from path: {} was loaded.", asset.path);

                assets_loaded.insert_asset(&name, asset.data.clone());

                assets_to_remove.push(name.clone());
            }
            _ => {
                info!("Asset from path: {} still loading.", asset.path);
            }
        }
    }

    for name in assets_to_remove {
        assets_to_load.remove_asset(&name);
    }
}
