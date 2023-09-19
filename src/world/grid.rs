use bevy::{gizmos, prelude::*};

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        //app.add_systems(Update, show_grid);
        //resources
        app.init_resource::<Grid>();
    }
}

//----resources----
#[derive(Resource, Clone, Copy)]
pub struct Grid {
    pub tile_size: f32,
}

impl Default for Grid {
    fn default() -> Self {
        Grid { tile_size: 0.5 }
    }
}
impl Grid {
    //world: position in world
    //tile: center position of a tile
    //index: index of a tile

    pub fn world_to_coord(&self, world_pos: Vec3) -> [i32; 2] {
        let index_vec = (world_pos / self.tile_size).floor();
        return [index_vec[0] as i32, index_vec[2] as i32];
    }

    pub fn coord_to_tile(&self, index: [i32; 2]) -> Vec3 {
        let index_vec = Vec3::new(index[0] as f32, 0.0, index[1] as f32);
        let mut tile = (index_vec + 0.5) * self.tile_size;
        tile.y = 0.0;
        return tile;
    }

    pub fn world_to_tile(self, position: Vec3) -> Vec3 {
        let index = self.world_to_coord(position);
        let tile = self.coord_to_tile(index);
        tile
    }
}

fn show_grid(grid: Res<Grid>, mut gizmos: Gizmos) {
    for i in 0..100 {
        for j in 0..100 {
            let position = grid.coord_to_tile([i, j]);
            let rotation = Quat::from_rotation_x(std::f32::consts::PI / 2.0);
            gizmos.rect(
                position,
                rotation,
                Vec2::new(grid.tile_size, grid.tile_size),
                Color::Rgba {
                    red: 0.2,
                    green: 0.2,
                    blue: 0.2,
                    alpha: 0.2,
                },
            )
        }
    }
}
