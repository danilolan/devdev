//! Handles the raycast of mouse into the world.

use bevy::prelude::*;

pub mod systems;
use systems::*;
pub mod components;
use components::*;
pub mod resources;
use resources::*;
pub mod states;
use states::*;

pub struct PickingPlugin;

impl Plugin for PickingPlugin {
    fn build(&self, app: &mut App) {
        //resources
        app.init_resource::<PickingData>();

        //systems
        app.add_systems(Update, handle_picking);
        //app.add_systems(Update, test);
    }
}
