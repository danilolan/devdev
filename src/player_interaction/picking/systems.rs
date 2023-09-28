use bevy::{prelude::*, window::PrimaryWindow};

use crate::player_interaction::camera::components::CameraDefault;

use super::resources::PickingData;

/// Update the current ray in the resource.
///
/// The ray is casted by the mouse position in screen to the world
pub fn handle_picking(
    mut picking: ResMut<PickingData>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    cam_q: Query<(&Camera, &GlobalTransform), With<CameraDefault>>,
) {
    let position = match q_windows.single().cursor_position() {
        Some(pos) => pos,
        None => return,
    };
    let (camera, camera_transform) = match cam_q.get_single() {
        Ok((cam, transform)) => (cam, transform),
        Err(_) => return,
    };
    let ray = match camera.viewport_to_world(&camera_transform, position) {
        Some(r) => r,
        None => return,
    };

    picking.ray = ray;
}

/* fn test(mut picking: ResMut<PickingData>, collider_query: Query<(Entity, &BoxCollider)>) {
    let entity = picking.clone().get_entity(collider_query);
}
 */
