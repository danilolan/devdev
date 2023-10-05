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
        //--systems
        app.add_systems(Startup, start_assets);
        app.add_systems(Update, check_assets_ready);
    }
}
