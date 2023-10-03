use crate::world::grid::resources::Grid;
use bevy::prelude::*;

pub fn show_grid(grid: Res<Grid>, mut gizmos: Gizmos) {
    for i in 0..50 {
        for t in 0..50 {
            let center_world_position = grid.coord_to_tile([i, t]);

            let tile_color = match grid.get_tile_status(i, t) {
                Some(true) => Color::RED,
                Some(false) | None => Color::GREEN,
            };

            gizmos.rect(
                center_world_position,
                Quat::from_rotation_x(90.0_f32.to_radians()),
                Vec2::new(grid.tile_size - 0.02, grid.tile_size - 0.02),
                tile_color,
            )
        }
    }
}
