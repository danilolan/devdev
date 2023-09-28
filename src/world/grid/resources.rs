use bevy::prelude::*;
use hashbrown::HashMap;

use crate::world::physics::components::BoxCollider;

#[derive(Resource, Clone)]
pub struct Grid {
    pub tile_size: f32,
    pub hashmap: HashMap<(i32, i32), bool>,
}

impl Default for Grid {
    fn default() -> Self {
        Grid {
            tile_size: 0.2,
            hashmap: HashMap::new(),
        }
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

    pub fn mark_tiles_from_collider(&mut self, collider: &BoxCollider) {
        // Pega os cantos da caixa do colisor
        let corners = collider.get_corners();

        // Determine os índices de grid mínimos e máximos que a caixa pode ocupar
        let min_index = self.world_to_coord(
            corners
                .iter()
                .fold(Vec3::splat(f32::INFINITY), |acc, &corner| acc.min(corner)),
        );
        let max_index = self.world_to_coord(
            corners
                .iter()
                .fold(Vec3::splat(f32::NEG_INFINITY), |acc, &corner| {
                    acc.max(corner)
                }),
        );

        // Itere sobre o range dos índices e verifique se o tile está dentro da caixa
        for x in min_index[0]..=max_index[0] {
            for z in min_index[1]..=max_index[1] {
                // Converta o índice do grid para a posição do mundo
                let tile_center = self.coord_to_tile([x, z]);

                // Criamos um colisor fictício para o tile e verificamos se ele está colidindo com o colisor original.
                let tile_collider =
                    BoxCollider::new(tile_center, Quat::IDENTITY, Vec3::splat(self.tile_size));

                if collider.is_colliding_with_tile(&tile_collider) {
                    self.hashmap.insert((x, z), true);
                }
            }
        }
    }

    pub fn get_tile_status(&self, x: i32, z: i32) -> Option<bool> {
        self.hashmap.get(&(x, z)).cloned()
    }
}
