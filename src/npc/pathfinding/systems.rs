use bevy::prelude::*;
use futures_lite::future;

use super::components::{Pathfinding, PathfindingTask};

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
                }
            }
        }
    }
}
