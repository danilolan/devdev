use bevy::{prelude::*, utils::HashMap};

enum AssetType {
    Image(Handle<Image>),
    Audio(Handle<AudioSource>),
    Scene(Handle<DynamicScene>),
}

pub struct AssetToLoad {
    pub path: String,
    pub data: HandleUntyped,
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
    pub fn insert_asset(&mut self, name: &str, data: HandleUntyped, path: &str) {
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

pub struct AssetsLoaded {
    assets: HashMap<String, AssetType>,
}

/* impl AssetsLoaded {
    pub fn new() -> Self {
        AssetsLoaded {
            assets: HashMap::new(),
        }
    }

    pub fn insert_asset(&mut self, path: String, asset_type: AssetType) {
        self.assets.insert(path, asset_type);
    }

    pub fn get_asset_image(&self, path: &str) -> Option<&Handle<Image>> {
        match self.assets.get(path) {
            Some(AssetType::Image(handle)) => Some(handle),
            _ => None,
        }
    }

    // Você pode adicionar funções semelhantes para outros tipos de ativos
} */
