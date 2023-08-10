use bevy::prelude::*;

mod systems;
use systems::*;

mod components;
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Update, orbit_mouse);
        app.add_systems(Update, zoom_mouse);
        app.add_systems(Update, mouse_click_world);
    }
}