use bevy::{asset::HandleId, prelude::*, utils::HashMap};

pub struct AssetToLoad {
    pub path: String,
    pub data: AssetType,
}

#[derive(Resource)]
pub struct AssetsToLoad {
    pub assets: HashMap<String, AssetToLoad>,
}

impl Default for AssetsToLoad {
    fn default() -> Self {
        Self {
            assets: HashMap::new(),
        }
    }
}

impl AssetsToLoad {
    pub fn insert_asset(&mut self, name: String, data: AssetType, path: String) {
        self.assets.insert(
            name.to_string(),
            AssetToLoad {
                path: path.to_string(),
                data,
            },
        );
    }

    pub fn remove_asset(&mut self, name: &str) {
        self.assets.remove(name);
    }
}
#[derive(Clone)]
pub enum AssetType {
    Image(Handle<Image>),
    Audio(Handle<AudioSource>),
    Scene(Handle<Scene>),
}
impl AssetType {
    pub fn handle_id(&self) -> HandleId {
        match self {
            AssetType::Image(handle) => handle.id(),
            AssetType::Audio(handle) => handle.id(),
            AssetType::Scene(handle) => handle.id(),
        }
    }
}

#[derive(Resource)]
pub struct AssetsLoaded {
    pub assets: HashMap<String, AssetType>,
}

impl Default for AssetsLoaded {
    fn default() -> Self {
        Self {
            assets: HashMap::new(),
        }
    }
}

impl AssetsLoaded {
    pub fn insert_asset(&mut self, path: &str, asset_type: AssetType) {
        self.assets.insert(path.to_string(), asset_type);
    }

    pub fn get_asset_image(&self, path: &str) -> &Handle<Image> {
        match self.assets.get(path) {
            Some(AssetType::Image(handle)) => handle,
            _ => panic!("The image in path: {} was not found", path),
        }
    }
    pub fn get_asset_scene(&self, path: &str) -> &Handle<Scene> {
        match self.assets.get(path) {
            Some(AssetType::Scene(handle)) => handle,
            _ => panic!("The scene in path: {} was not found", path),
        }
    }
    pub fn get_asset_audio(&self, path: &str) -> &Handle<AudioSource> {
        match self.assets.get(path) {
            Some(AssetType::Audio(handle)) => handle,
            _ => panic!("The audio in path: {} was not found", path),
        }
    }
}
