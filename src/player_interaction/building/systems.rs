use bevy::prelude::*;

use crate::{
    player_interaction::{picking::resources::PickingData, selection::resources::ObjectToolData},
    world::{grid::resources::Grid, physics::components::BoxCollider},
};

use super::{components::Building, states::BuildingState};

const WALL_KEY: KeyCode = KeyCode::F1;
const PILLAR_KEY: KeyCode = KeyCode::F2;
const WINDOW_KEY: KeyCode = KeyCode::F3;
const DOOR_KEY: KeyCode = KeyCode::F4;
const DESTROY_KEY: KeyCode = KeyCode::F5;

pub fn handle_states(
    keys: Res<Input<KeyCode>>,
    mut building_state: ResMut<NextState<BuildingState>>,
    mut object_tool_data: ResMut<ObjectToolData>,
    mut commands: Commands,
) {
    if keys.just_pressed(WALL_KEY) {
        object_tool_data.delete_entity_in_tool(&mut commands);
        building_state.set(BuildingState::Wall);
    }
    if keys.just_released(PILLAR_KEY) {
        object_tool_data.delete_entity_in_tool(&mut commands);
        building_state.set(BuildingState::Pillar);
    }
    if keys.pressed(WINDOW_KEY) {
        object_tool_data.delete_entity_in_tool(&mut commands);
        building_state.set(BuildingState::Window);
    }
    if keys.pressed(DOOR_KEY) {
        object_tool_data.delete_entity_in_tool(&mut commands);
        building_state.set(BuildingState::Door);
    }
    if keys.pressed(DESTROY_KEY) {
        object_tool_data.delete_entity_in_tool(&mut commands);
        building_state.set(BuildingState::Destroy);
    }
}

pub fn spawn_asset(
    mut commands: Commands,
    asset: Handle<Scene>,
    mut object_tool_data: ResMut<ObjectToolData>,
    mut picking: Res<PickingData>,
    collider_scale: Vec3,
) {
    let hit_point = picking.get_hit_in_ground();
    let translation: Vec3 = match object_tool_data.grid_size {
        Some(grid_size) => (hit_point / grid_size).round() * grid_size,
        None => hit_point,
    };

    let entity = commands
        .spawn((
            (SceneBundle {
                scene: asset.clone(),
                transform: Transform {
                    translation,
                    rotation: Quat::from_rotation_y(object_tool_data.current_angle.to_radians()),
                    scale: Vec3::ONE,
                },
                ..Default::default()
            }),
            Building {},
        ))
        .insert(Name::from("building".to_string()))
        .id();

    commands.entity(entity).insert(BoxCollider {
        scale: collider_scale,
        translation: Vec3::ZERO,
        rotation: Quat::default(),
    });

    object_tool_data.set_new_entity_in_tool(entity, &mut commands);
}

pub fn handle_destroy(
    mut commands: Commands,
    mut picking: Res<PickingData>,
    collider_query: Query<(Entity, &BoxCollider), With<Building>>,
    buttons: Res<Input<MouseButton>>,
    mut object_tool_data: ResMut<ObjectToolData>,
    mut grid: ResMut<Grid>,
    query_entity: Query<&BoxCollider>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(entity) = picking.get_entity::<Building>(collider_query) {
            object_tool_data.set_new_entity_in_tool(entity, &mut commands);
            object_tool_data.remove_entity_in_world(grid, query_entity, &mut commands);
        }
    }
}

pub fn handle_wall(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut object_tool_data: ResMut<ObjectToolData>,
    mut picking: Res<PickingData>,
) {
    let wall: Handle<Scene> = server.load("./models/wall.gltf#Scene0");

    if object_tool_data.entity.is_none() {
        spawn_asset(
            commands,
            wall,
            object_tool_data,
            picking,
            Vec3::new(0.2, 1.7, 1.0),
        )
    }
}

pub fn handle_window(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut object_tool_data: ResMut<ObjectToolData>,
    mut picking: Res<PickingData>,
) {
    let window: Handle<Scene> = server.load("./models/window.gltf#Scene0");

    if object_tool_data.entity.is_none() {
        spawn_asset(
            commands,
            window,
            object_tool_data,
            picking,
            Vec3::new(0.2, 1., 1.0),
        )
    }
}

pub fn handle_pillar(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut object_tool_data: ResMut<ObjectToolData>,
    mut picking: Res<PickingData>,
) {
    let wall: Handle<Scene> = server.load("./models/pillar.gltf#Scene0");

    if object_tool_data.entity.is_none() {
        spawn_asset(
            commands,
            wall,
            object_tool_data,
            picking,
            Vec3::new(0.2, 1.7, 0.2),
        )
    }
}

pub fn handle_door(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut object_tool_data: ResMut<ObjectToolData>,
    mut picking: Res<PickingData>,
) {
    let wall: Handle<Scene> = server.load("./models/pillar.gltf#Scene0");

    if object_tool_data.entity.is_none() {
        spawn_asset(
            commands,
            wall,
            object_tool_data,
            picking,
            Vec3::new(0.2, 1., 1.0),
        )
    }
}
