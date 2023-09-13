use bevy::{prelude::*, diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

mod player_interaction;
use player_interaction::PlayerInteractionPlugin;

mod world;
use world::WorldPlugin;

mod scene;
use scene::ScenePlugin;

fn main() {
    App::new()
        // Window Setup
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))

        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())

        //fps
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(LogDiagnosticsPlugin::default())
        
        //Plugins
        .add_plugins(PlayerInteractionPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(ScenePlugin)
        .run();
}
