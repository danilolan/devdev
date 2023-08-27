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
        Grid { tile_size: 0.1 }
    }
}

impl Grid {
    //world: position in world
    //tile: center position of a tile
    //index: index of a tile

    pub fn world_to_index(&self, world_pos: Vec3) -> Vec3 {
        (world_pos / self.tile_size).floor()
    }

    pub fn index_to_tile(&self, index: Vec3) -> Vec3 {
        (index + 0.5) * self.tile_size
    }

    pub fn world_to_tile(self, position: Vec3) -> Vec3 {
        let index = self.world_to_index(position);
        let tile = self.index_to_tile(index);
        tile
    }
}
