//! Handle the main camera logics of the game

use bevy::prelude::*;
use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    window::PrimaryWindow,
};

pub mod systems;
use systems::*;

pub mod components;
use components::*;
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Update, orbit_mouse);
        app.add_systems(Update, zoom_mouse);
    }
}
