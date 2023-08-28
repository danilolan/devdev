use bevy::prelude::*;

pub struct ObjectPlugin;

impl Plugin for ObjectPlugin {
    fn build(&self, _app: &mut App) {
        //app.add_systems(Startup, setup);
    }
}

//----components----
#[derive(Component)]
pub struct Object {}
