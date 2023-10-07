use bevy::prelude::*;

pub mod pathfinding;
use self::pathfinding::*;

pub mod components;
use components::*;

pub mod states;
use states::*;

pub mod behavior;
use behavior::*;

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PathfindingPlugin);
        app.add_plugins(BehaviorPlugin);
    }
}
