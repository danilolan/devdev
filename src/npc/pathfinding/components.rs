use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
};
use pathfinding::*;

use crate::world::grid::resources::{Grid, Path, PathfindingError};

#[derive(Component)]
pub struct Pathfinding {
    pub path: Option<Path>,
}

impl Default for Pathfinding {
    fn default() -> Self {
        Self { path: None }
    }
}

impl Pathfinding {}

#[derive(Component)]
pub struct PathfindingTask(pub Task<Result<Path, PathfindingError>>);

pub fn spawn_optimized_pathfinding_task(
    commands: &mut Commands,
    target: Entity,
    grid: &Grid,
    start: Vec3,
    end: Vec3,
) {
    // Fail early if end is not valid
    if grid.obstructed(&end) {
        return;
    }

    let thread_pool = AsyncComputeTaskPool::get();

    // Must clone because the grid can change between frames
    // Must box to prevent stack overflows on very large grids
    let grid = Box::new(grid.clone());

    let task = thread_pool.spawn(async move {
        let mut path = grid.find_path(&start, &end);
        let _ = path.as_mut().map(|p| p.optimize_corners());
        path
    });

    commands.entity(target).insert(PathfindingTask(task));
}
