use bevy::prelude::*;

pub mod systems;
use systems::*;
pub mod components;
use components::*;
pub mod resources;
use resources::*;
pub mod states;
use states::*;

pub struct PathfindingPlugin;

impl Plugin for PathfindingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_pathfinding_tasks);
    }
}
