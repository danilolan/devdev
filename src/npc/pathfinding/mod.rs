//! Handle the pathfinding data attached to an entity
//!
//! The actual pathfinding calculation is in Grid module. This modules handles the async tasks and the data returned by this tasks.

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
