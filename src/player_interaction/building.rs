use bevy::prelude::*;

use crate::world::physics::BoxCollider;

use super::selection::ObjectToolData;

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
    }
}

const WALL_KEY: KeyCode = KeyCode::F1;
const WINDOW_KEY: KeyCode = KeyCode::F2;
const DOOR_KEY: KeyCode = KeyCode::F3;

fn handle_states(
    keys: Res<Input<KeyCode>>,
    mut building_state: ResMut<NextState<BuildingState>>,
    mut object_tool_data: ResMut<ObjectToolData>,
    mut commands: Commands,
) {
    if keys.just_pressed(WALL_KEY) {
        building_state.set(BuildingState::Wall);
        object_tool_data.delete_entity(&mut commands)
    }
    if keys.just_released(WINDOW_KEY) {
        building_state.set(BuildingState::Window);
        object_tool_data.delete_entity(&mut commands)
    }
    if keys.pressed(DOOR_KEY) {
        building_state.set(BuildingState::Door);
        object_tool_data.delete_entity(&mut commands)
    }
}

fn spawn_asset(
    mut commands: Commands,
    asset: Handle<Scene>,
    mut object_tool_data: ResMut<ObjectToolData>,
) {
    let entity = commands
        .spawn((
            (SceneBundle {
                scene: asset.clone(),
                ..Default::default()
            }),
            BoxCollider {
                scale: Vec3::new(1.0, 1.5, 0.2),
                translation: Vec3::ZERO,
                rotation: Quat::default(),
            },
        ))
        .insert(Name::from("building".to_string()))
        .id();

    object_tool_data.set_new_entity(entity, &mut commands);
}

fn handle_wall(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut object_tool_data: ResMut<ObjectToolData>,
) {
    let wall: Handle<Scene> = server.load("./models/wall.gltf#Scene0");

    if object_tool_data.entity.is_none() {
        spawn_asset(commands, wall, object_tool_data)
    }
}

fn handle_window(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut object_tool_data: ResMut<ObjectToolData>,
) {
    let window: Handle<Scene> = server.load("./models/window.gltf#Scene0");

    if object_tool_data.entity.is_none() {
        spawn_asset(commands, window, object_tool_data)
    }
}

fn handle_door(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut object_tool_data: ResMut<ObjectToolData>,
) {
    let wall: Handle<Scene> = server.load("./models/pillar.gltf#Scene0");

    if object_tool_data.entity.is_none() {
        spawn_asset(commands, wall, object_tool_data)
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
