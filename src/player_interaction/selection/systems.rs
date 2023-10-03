use bevy::prelude::*;
use bevy_inspector_egui::egui::epaint::tessellator::path;

use crate::{
    npc::pathfinding::components::{spawn_optimized_pathfinding_task, Pathfinding},
    player_interaction::picking::resources::PickingData,
    world::{
        grid::{self, resources::Grid},
        physics::components::{BoxCollider, LerpMovement},
    },
};

use super::{resources::ObjectToolData, states::CanPlaceState};

use crate::scene::Player;

pub fn handle_object(
    picking: Res<PickingData>,
    object_tool_data: ResMut<ObjectToolData>,
    mut global_query: Query<(&mut LerpMovement,)>,
) {
    if let Some(entity) = object_tool_data.entity {
        let hit_point = picking.get_hit_in_ground();
        if let Ok((mut lerp_movement,)) = global_query.get_mut(entity) {
            let position: Vec3 = match object_tool_data.grid_size {
                Some(grid_size) => (hit_point / grid_size).round() * grid_size,
                None => hit_point,
            };

            lerp_movement.set_target_translation(position);
        }
    }
}

pub fn rotate_object(
    mut object_tool_data: ResMut<ObjectToolData>,
    mut query: Query<(&mut Transform, &mut LerpMovement)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let entity = match object_tool_data.entity {
        Some(entity) => entity,
        None => return,
    };

    let (mut transform, mut lerp_movement) = match query.get_mut(entity) {
        Ok(transform) => transform,
        Err(_) => return,
    };

    if keyboard_input.just_pressed(KeyCode::E) {
        object_tool_data.current_angle += object_tool_data.angle_step;
        let new_rotation = object_tool_data.current_angle.to_radians();
        lerp_movement.set_target_rotation(Quat::from_rotation_y(new_rotation));
    }

    if keyboard_input.just_pressed(KeyCode::Q) {
        object_tool_data.current_angle -= object_tool_data.angle_step;
        let new_rotation = object_tool_data.current_angle.to_radians();
        lerp_movement.set_target_rotation(Quat::from_rotation_y(new_rotation));
    }
}

pub fn place_object(
    mut object_tool_data: ResMut<ObjectToolData>,
    buttons: Res<Input<MouseButton>>,
    grid: Res<Grid>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        object_tool_data.place_entity_in_world();
    }
}

pub fn handle_can_place_state(
    query_colliders: Query<(Entity, &BoxCollider)>,
    object_tool_data: Res<ObjectToolData>,
    mut can_place_state: ResMut<NextState<CanPlaceState>>,
) {
    if let Some(entity) = object_tool_data.entity {
        if let Ok((_, current_collider)) = query_colliders.get(entity) {
            let is_colliding = current_collider.is_colliding_with(entity, &query_colliders, -0.1);
            if is_colliding {
                can_place_state.set(CanPlaceState::False)
            } else {
                can_place_state.set(CanPlaceState::True)
            }
        } else {
            can_place_state.set(CanPlaceState::True)
        }
    }
}
pub fn handle_entities(
    mut object_tool_data: ResMut<ObjectToolData>,
    mut grid: ResMut<Grid>,
    query_entity: Query<(&BoxCollider, &LerpMovement), With<BoxCollider>>,
    mut commands: Commands,
) {
    // Mark tiles for entities that are to be placed
    for &entity in &object_tool_data.entities_to_place {
        if let Ok((collider, lerp_movement)) = query_entity.get(entity) {
            if lerp_movement.target_translation.is_none() {
                grid.mark_tiles_from_collider(collider);
            }
        }
    }

    // Filter out entities that still need placement
    let entities_still_to_place: Vec<_> = object_tool_data
        .entities_to_place
        .iter()
        .filter(|&&entity| {
            if let Ok((_, lerp_movement)) = query_entity.get(entity) {
                lerp_movement.target_translation.is_some()
            } else {
                true
            }
        })
        .cloned()
        .collect();
    object_tool_data.entities_to_place = entities_still_to_place;

    // Handle entities that are to be removed
    for &entity in &object_tool_data.entities_to_remove {
        if let Ok((collider, _)) = query_entity.get(entity) {
            grid.unmark_tiles_from_collider(collider);
        }
        commands.entity(entity).despawn_recursive();
    }
    object_tool_data.entities_to_remove.clear();
}

pub fn show_path(
    query_entity: Query<Entity, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands,
    grid: Res<Grid>,
    mut gizmos: Gizmos,
    pathfinding_query: Query<&Pathfinding, With<Player>>,
) {
    if keyboard_input.pressed(KeyCode::F) {
        if let Ok(entity) = query_entity.get_single() {
            spawn_optimized_pathfinding_task(
                &mut commands,
                entity,
                &grid,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(5.0, 0.0, 0.0),
            );
        }
    }

    if let Ok(pathfinding) = pathfinding_query.get_single() {
        if let Some(path) = &pathfinding.path {
            for i in 0..path.steps.len() {
                if i == path.steps.len() - 1 {
                    break;
                }
                gizmos.line(path.steps[i], path.steps[i + 1], Color::BLUE)
            }
        }
    }
}
