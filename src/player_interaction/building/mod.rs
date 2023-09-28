//! Handles the bulding logic.
//!
//! Building walls, windows, doors ... and destroying them as well

use bevy::prelude::*;

pub mod systems;
use systems::*;
pub mod components;
use components::*;
pub mod resources;
use resources::*;
pub mod states;
use states::*;

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<BuildingState>();

        app.add_systems(Update, handle_states);
        app.add_systems(Update, handle_wall.run_if(in_state(BuildingState::Wall)));
        app.add_systems(
            Update,
            handle_window.run_if(in_state(BuildingState::Window)),
        );
        app.add_systems(Update, handle_door.run_if(in_state(BuildingState::Door)));
        app.add_systems(
            Update,
            handle_pillar.run_if(in_state(BuildingState::Pillar)),
        );
        app.add_systems(
            Update,
            handle_destroy.run_if(in_state(BuildingState::Destroy)),
        );
    }
}
