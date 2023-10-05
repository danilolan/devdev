use bevy::prelude::*;

use super::resources::AssetsToLoad;

pub fn start_assets(asset_server: Res<AssetServer>, mut assets_to_load: ResMut<AssetsToLoad>) {
    let wall: Handle<Scene> = asset_server.load("./models/wall.gltf#Scene0");
    assets_to_load.insert_asset(
        "models/building/wall",
        wall.clone_untyped(),
        "./models/wall.gltf#Scene0",
    );

    let window: Handle<Scene> = asset_server.load("./models/window.gltf#Scene0");
    assets_to_load.insert_asset(
        "models/building/window",
        window.clone_untyped(),
        "./models/window.gltf#Scene0",
    );
}

pub fn check_assets_ready(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut assets_to_load: ResMut<AssetsToLoad>,
) {
    use bevy::asset::LoadState;
    let mut assets_loaded: Vec<String> = Vec::new();

    for (name, asset) in &assets_to_load.assets {
        match server.get_load_state(&asset.data) {
            LoadState::Failed => {
                panic!("Asset from path: {} failed to load.", asset.path);
            }
            LoadState::Loaded => {
                info!("Asset from path: {} was loaded.", asset.path);
                assets_loaded.push(name.clone());
            }
            _ => {
                info!("Asset from path: {} still loading.", asset.path);
            }
        }
    }

    for name in assets_loaded {
        assets_to_load.remove_asset(&name);
    }
}
