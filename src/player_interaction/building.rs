use bevy::prelude::*;

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<BuildingState>();

        app.add_systems(Update, handle_states);
    }
}
const WALL_KEY: KeyCode = KeyCode::F1;
const WINDOW_KEY: KeyCode = KeyCode::F2;
const DOOR_KEY: KeyCode = KeyCode::F3;

fn handle_states(keys: Res<Input<KeyCode>>, mut building_state: ResMut<State<BuildingState>>) {
    if keys.just_pressed(WALL_KEY) {}
    if keys.just_released(WINDOW_KEY) {
        // Left Ctrl was released
    }
    if keys.pressed(DOOR_KEY) {
        // W is being held down
    }
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash)]
enum BuildingState {
    Wall,
    Window,
    Door,
    None,
}

impl Default for BuildingState {
    fn default() -> Self {
        Self::None
    }
}
