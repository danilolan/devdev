use bevy::prelude::*;
use futures_lite::future;

use super::components::{Pathfinding, PathfindingTask};

/// Get the all the PathfindigTask components and verify if each one has already ended. If succeed atach the returned path to the pathfinding component.
pub fn handle_pathfinding_tasks(
    mut commands: Commands,
    mut pathfinding_query: Query<&mut Pathfinding>,
    mut tasks: Query<(Entity, &mut PathfindingTask)>,
) {
    for (task_entity, mut task) in &mut tasks {
        if let Some(result) = future::block_on(future::poll_once(&mut task.0)) {
            commands.entity(task_entity).remove::<PathfindingTask>();
            if let Ok(mut pathfinding) = pathfinding_query.get_mut(task_entity) {
                if let Ok(path) = result {
                    pathfinding.path = Some(path);
                } else {
                    error!("No path was found the {:?} entity", task_entity);
                }
            }
        }
    }
}
