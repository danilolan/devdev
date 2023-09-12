use bevy::prelude::*;

pub struct ObjectPlugin;

impl Plugin for ObjectPlugin {
    fn build(&self, _app: &mut App) {
        
    }
}

//----components----
#[derive(Component)]
pub struct Object {}
