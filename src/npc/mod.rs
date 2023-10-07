use bevy::prelude::*;

pub mod pathfinding;
use self::pathfinding::*;

pub mod components;
use components::*;

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PathfindingPlugin);
    }
}
