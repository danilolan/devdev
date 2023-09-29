use bevy::prelude::*;

use crate::{
    player_interaction::picking::resources::PickingData,
    world::{
        grid::{self, resources::Grid},
        physics::components::{BoxCollider, LerpMovement},
    },
};

use super::{resources::ObjectToolData, states::CanPlaceState};

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
    mut grid: ResMut<Grid>,
    query_entity: Query<&BoxCollider>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(entity) = object_tool_data.entity {
            object_tool_data.entity = None;
            if let Ok(collider) = query_entity.get(entity) {
                grid.mark_tiles_from_collider(collider);
            }
        }
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