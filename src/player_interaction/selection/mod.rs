//! Handles the player interaction to the Objects

use bevy::prelude::*;

pub mod systems;
use systems::*;
pub mod components;
use components::*;
pub mod resources;
use resources::*;
pub mod states;
use states::*;

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        //states
        app.add_state::<CanPlaceState>();

        //resources
        app.init_resource::<ObjectToolData>();

        //systems
        app.add_systems(Update, handle_object);
        app.add_systems(Update, rotate_object);
        app.add_systems(Update, handle_can_place_state);
        app.add_systems(Update, place_object.run_if(in_state(CanPlaceState::True)));
        app.add_systems(Update, handle_entities);
    }
}
