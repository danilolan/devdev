use bevy::prelude::*;

pub mod systems;
use systems::*;
pub mod components;
use components::*;


pub struct ObjectsPlugin;

impl Plugin for ObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_object);

        app.add_systems(Update, handle_popup_state);
    }
}