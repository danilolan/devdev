use super::resources::{AssetsLoaded, AssetsToLoad};
use crate::asset_manager::resources::AssetType;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
#[derive(Deserialize)]
pub struct Asset {
    pub path: String,
    pub name: String,
}
#[derive(Deserialize)]
pub struct AssetsConfig {
    pub scene: Vec<Asset>,
    pub image: Vec<Asset>,
    pub audio: Vec<Asset>,
}

/// Load the json file of the assets config
pub fn load_asset_config(path: &str) -> AssetsConfig {
    let file = File::open(path).expect("Failed to open asset config");
    serde_json::from_reader(file).expect("Error while reading json")
}

/// Start the loading assets.
///
/// Load assets by the asset_server from bevy and push them to [AssetsToLoad] resource to track the state
pub fn start_assets(asset_server: Res<AssetServer>, mut assets_to_load: ResMut<AssetsToLoad>) {
    let config = load_asset_config("./config/assets.json");

    for scene in config.scene {
        let handle_scene: Handle<Scene> = asset_server.load(scene.path.clone());
        assets_to_load.insert_asset(
            scene.name.clone(),
            AssetType::Scene(handle_scene),
            scene.path.clone(),
        );
    }

    for image in config.image {
        let handle_image: Handle<Image> = asset_server.load(image.path.clone());
        assets_to_load.insert_asset(
            image.name.clone(),
            AssetType::Image(handle_image),
            image.path.clone(),
        );
    }
    for audio in config.audio {
        let handle_audio: Handle<AudioSource> = asset_server.load(audio.path.clone());
        assets_to_load.insert_asset(
            audio.name.clone(),
            AssetType::Audio(handle_audio),
            audio.path.clone(),
        );
    }
}

/// Check the state of assets in [AssetsToLoad] resource.
///
/// If assets was loaded push it to the [AssetsLoaded]
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
