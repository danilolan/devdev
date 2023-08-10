use bevy::prelude::*;

mod camera;
use camera::CameraPlugin;

//default
mod systems;
use systems::*;
mod components;
use components::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, plane_movement);
        app.add_systems(Update, sync_player_rotation);

        app.add_plugins(CameraPlugin);
    }
}