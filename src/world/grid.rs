use bevy::prelude::*;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
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
        Grid { tile_size: 1.0 }
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
