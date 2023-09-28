use bevy::prelude::*;
pub struct WorldPlugin;

pub mod grid;
use grid::*;

pub mod physics;
use physics::*;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        //systems
        //plugins
        app.add_plugins(GridPlugin);
        app.add_plugins(PhysicsPlugin);
    }
}
