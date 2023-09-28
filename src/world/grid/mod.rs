//! Handle the world grid.
//!
//! Divides the space in plane xz by the grid_size creating tiles in world
use bevy::prelude::*;

pub mod systems;
use systems::*;
pub mod components;
use components::*;
pub mod resources;
use resources::*;
pub mod states;
use states::*;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Grid>();
    }
}
