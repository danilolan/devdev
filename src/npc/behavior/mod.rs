use bevy::prelude::*;

pub mod systems;
use systems::*;
pub mod components;
use components::*;
pub mod resources;
use resources::*;
pub mod states;
use states::*;

pub struct BehaviorPlugin;

impl Plugin for BehaviorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_walking);
    }
}
