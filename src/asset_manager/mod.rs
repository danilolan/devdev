//! Load all assets and provides a resouce to acess this assets.
//!
//! # Structure
//! The assets needs to be import to assets/...
//! The folders can be audio, scene, and image. If you need to create another folder you have to update the [start_assets]
//! system to receive your new asset folder.
//!
//! After that you need to create asset config in config/assets.json following the struture below:
//! ``` json
//! {
//! "scene": [
//!   {
//!     "path": "./scene/wall.gltf#Scene0",
//!     "name": "scene/building/wall"
//!   },
//!   {
//!     "path": "./scene/window.gltf#Scene0",
//!     "name": "scene/building/window"
//!   },
//!   {
//!     "path": "./scene/pillar.gltf#Scene0",
//!     "name": "scene/building/pillar"
//!   }
//! ],
//! "image": [],
//! "audio": []
//! }
//! ```

use bevy::prelude::*;

pub mod systems;
use systems::*;
pub mod components;
use components::*;
pub mod resources;
use resources::*;
pub mod states;
use states::*;

pub struct AssetManagerPlugin;

impl Plugin for AssetManagerPlugin {
    fn build(&self, app: &mut App) {
        //--resources
        app.init_resource::<AssetsToLoad>();
        app.init_resource::<AssetsLoaded>();
        //--systems
        app.add_systems(Startup, start_assets);
        app.add_systems(Update, check_assets_ready);
    }
}
