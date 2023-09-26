use bevy::prelude::*;

use crate::world::physics::BoxCollider;

use super::{picking::PickingData, selection::ObjectToolData};

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
    }
}

const WALL_KEY: KeyCode = KeyCode::F1;
const PILLAR_KEY: KeyCode = KeyCode::F2;
const WINDOW_KEY: KeyCode = KeyCode::F3;
const DOOR_KEY: KeyCode = KeyCode::F4;

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
    if keys.just_released(PILLAR_KEY) {
        building_state.set(BuildingState::Pillar);
        object_tool_data.delete_entity(&mut commands)
    }
    if keys.pressed(WINDOW_KEY) {
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
    mut picking: Res<PickingData>,
    insert_collider: bool,
) {
    let hit_point = picking.get_hit_in_ground();
    let translation: Vec3 = match object_tool_data.grid_size {
        Some(grid_size) => (hit_point / grid_size).round() * grid_size,
        None => hit_point,
    };

    println!("{:?}", object_tool_data.current_angle);
    let entity = commands
        .spawn(((SceneBundle {
            scene: asset.clone(),
            transform: Transform {
                translation,
                rotation: Quat::from_rotation_y(object_tool_data.current_angle.to_radians()),
                scale: Vec3::ONE,
            },
            ..Default::default()
        }),))
        .insert(Name::from("building".to_string()))
        .id();

    if insert_collider {
        commands.entity(entity).insert(BoxCollider {
            scale: Vec3::new(0.2, 1.5, 1.0),
            translation: Vec3::ZERO,
            rotation: Quat::default(),
        });
    }

    object_tool_data.set_new_entity(entity, &mut commands);
}

fn handle_wall(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut object_tool_data: ResMut<ObjectToolData>,
    mut picking: Res<PickingData>,
) {
    let wall: Handle<Scene> = server.load("./models/wall.gltf#Scene0");

    if object_tool_data.entity.is_none() {
        spawn_asset(commands, wall, object_tool_data, picking, true)
    }
}

fn handle_window(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut object_tool_data: ResMut<ObjectToolData>,
    mut picking: Res<PickingData>,
) {
    let window: Handle<Scene> = server.load("./models/window.gltf#Scene0");

    if object_tool_data.entity.is_none() {
        spawn_asset(commands, window, object_tool_data, picking, true)
    }
}

fn handle_pillar(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut object_tool_data: ResMut<ObjectToolData>,
    mut picking: Res<PickingData>,
) {
    let wall: Handle<Scene> = server.load("./models/pillar.gltf#Scene0");

    if object_tool_data.entity.is_none() {
        spawn_asset(commands, wall, object_tool_data, picking, false)
    }
}

fn handle_door(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut object_tool_data: ResMut<ObjectToolData>,
    mut picking: Res<PickingData>,
) {
    let wall: Handle<Scene> = server.load("./models/pillar.gltf#Scene0");

    if object_tool_data.entity.is_none() {
        spawn_asset(commands, wall, object_tool_data, picking, true)
    }
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash)]
enum BuildingState {
    Wall,
    Window,
    Pillar,
    Door,
    None,
}

impl Default for BuildingState {
    fn default() -> Self {
        Self::None
    }
}
