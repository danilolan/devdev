use bevy::prelude::*;

use crate::npc::{components::Npc, pathfinding::components::Pathfinding};

use super::states::BehaviorState;

/// Handles the path following for each NPC
pub fn handle_walking(
    mut npcs: Query<(&mut Transform, &mut Pathfinding, &Npc), With<Npc>>,
    time: Res<Time>,
) {
    for (mut transform, mut pathfinding, npc) in npcs.iter_mut() {
        // Ensure that this NPC is in walking state
        if npc.behavior_state != BehaviorState::Walking {
            return;
        }

        if let Some(path) = &pathfinding.path {
            // Ensure the NPC still has steps to follow
            if pathfinding.current_step < path.steps.len() {
                let target = path.steps[pathfinding.current_step];
                let direction = (target - transform.translation).normalize();

                // Move the NPC towards the step
                transform.translation += direction * npc.movement_speed * time.delta_seconds();

                // Calculate the desired rotation using `look_at`
                let mut target_transform = Transform::from_translation(transform.translation);
                target_transform.look_at(target, Vec3::Y);
                let target_rotation = target_transform.rotation;

                // Smoothly interpolate between current rotation and desired rotation
                transform.rotation = transform
                    .rotation
                    .slerp(target_rotation, npc.rotation_speed * time.delta_seconds());

                // Check if NPC has reached the step (use a small error margin)
                if transform.translation.distance(target) < 0.1 {
                    pathfinding.current_step += 1;
                }
            }
        }
    }
}
