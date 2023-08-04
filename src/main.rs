use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

mod camera;
use camera::CameraPlugin;

mod scene;
use scene::ScenePlugin;

fn main() {
    App::new()
        // Window Setup
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        
        //Plugins
        .add_plugins(CameraPlugin)
        .add_plugins(ScenePlugin)
        .run();
}
