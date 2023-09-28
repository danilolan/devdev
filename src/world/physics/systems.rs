use bevy::prelude::*;

use super::components::{BoxCollider, LerpMovement, SmoothMovement};

pub fn handle_smooth_movement(
    mut query: Query<(&mut Transform, &SmoothMovement), With<SmoothMovement>>,
) {
    for (mut transform, movement) in query.iter_mut() {
        transform.translation = movement.translation;
    }
}

pub fn handle_colliders(
    mut collider_query: Query<(&mut BoxCollider, &Transform), With<BoxCollider>>,
) {
    for (mut collider, transform) in collider_query.iter_mut() {
        let translation = transform.translation;
        let rotation = transform.rotation;

        collider.translation = Vec3::new(
            translation.x,
            translation.y + (collider.scale.y / 2.0),
            translation.z,
        );
        collider.rotation = rotation;
    }
}
const SHOW_COLLIDERS: bool = true;
pub fn show_colliders(mut gizmos: Gizmos, collider_query: Query<&BoxCollider, With<BoxCollider>>) {
    if !SHOW_COLLIDERS {
        return;
    }
    for collider in collider_query.iter() {
        gizmos.cuboid(
            Transform {
                translation: collider.translation,
                rotation: collider.rotation,
                scale: collider.scale,
            },
            Color::GREEN,
        )
    }
}

const TRANSLATION_DIFF: f32 = 0.01;
const ROTATION_DIFF: f32 = 0.9999;
const SCALE_DIFF: f32 = 0.01;

pub fn handle_lerp_movement(
    time: Res<Time>,
    mut query: Query<(&mut LerpMovement, &mut Transform)>,
) {
    for (mut lerp_movement, mut transform) in query.iter_mut() {
        if let Some(target) = lerp_movement.target_translation {
            let t = lerp_movement.speed * time.delta_seconds();
            let new_translation = transform.translation.lerp(target, t);

            if (new_translation - target).length() < TRANSLATION_DIFF {
                transform.translation = target;
                lerp_movement.target_translation = None;
            } else {
                transform.translation = new_translation;
            }
        }

        if let Some(target) = lerp_movement.target_rotation {
            let t = lerp_movement.speed * time.delta_seconds();
            let new_rotation = transform.rotation.lerp(target, t);

            if new_rotation.dot(target).abs() > ROTATION_DIFF {
                transform.rotation = target;
                lerp_movement.target_rotation = None;
            } else {
                transform.rotation = new_rotation;
            }
        }

        if let Some(target) = lerp_movement.target_scale {
            let t = lerp_movement.speed * time.delta_seconds();
            let new_scale = transform.scale.lerp(target, t);

            if (new_scale - target).length() < SCALE_DIFF {
                transform.scale = target;
                lerp_movement.target_scale = None;
            } else {
                transform.scale = new_scale;
            }
        }
    }
}
