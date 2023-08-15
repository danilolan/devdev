use bevy::prelude::*;

pub struct TilesPlugin;

impl Plugin for TilesPlugin {
    fn build(&self, app: &mut App){
        //resources
        app.init_resource::<TileMap>();

        //systems
        app.add_systems(Update, handle_tilemap);
        app.add_systems(Update, draw_tilemap);
    }
}

//----RESOURCES----
#[derive(Clone)]
#[derive(Debug)]
struct Tile {
    entity: Option<Entity>,
    is_open: bool
}

impl Tile {
    fn empty() -> Self {
        Tile {
            entity: None,
            is_open: false
        }
    }
}

#[derive(Resource)]
pub struct TileMap {
    tiles: Vec<Vec<Tile>>,
    tile_size: f32,
    origin: Vec3
}

const MAP_WIDTH: usize = 1000;
const MAP_HEIGHT: usize = 1000;
const TILE_SIZE: f32 = 0.1;

impl Default for TileMap {
    fn default() -> Self {
        TileMap { 
            tiles:  vec![vec![Tile::empty(); MAP_WIDTH]; MAP_HEIGHT], 
            tile_size: TILE_SIZE,
            origin: Vec3::new(-(MAP_WIDTH as f32 * TILE_SIZE) / 2.0, 0.0, -(MAP_WIDTH as f32 * TILE_SIZE) / 2.0)
        }
    }
}

impl TileMap {
    //a method to place an entity that ocupies multiple tiles or just one
    pub fn place_entity(&mut self,entity: Entity, index: [usize; 2], size: [usize; 2], ) -> Result<(), &'static str> {
        // check all tiles that the entity will be ocupied
        for x in index[0]..(index[0] + size[0]) {
            for y in index[1]..(index[1] + size[1]) {
                if self.check_if_tile_is_empty(index) {
                    return Err("Cannot place entity here.");
                }
            }
        }

        // put entity on tile
        for x in index[0]..(index[0] + size[0]) {
            for y in index[1]..(index[1] + size[1]) {
                self.tiles[y][x].entity = Some(entity);
            }
        }

        Ok(())
    }

    //all validations to verify if a tile is completely able to receive a new entity
    fn check_if_tile_is_empty(&self, index: [usize; 2]) -> bool {
        if index[0] >= self.tiles.len() || index[1] >= self.tiles[0].len() {return false;}
        if !self.tiles[index[0]][index[1]].is_open {return false;}
        if !self.tiles[index[0]][index[1]].entity.is_some() {return false;}

        return true;
    }

    //return the position in the world of a tile
    fn tile_index_to_world_pos(&self, index: [usize; 2]) -> Result<Vec3, &'static str> {
        if index[0] > self.tiles.len() || index[1] > self.tiles[0].len() {
            return Err("Index is greater than tilemap size.");
        }

        let x = (index[0] as f32 * self.tile_size) + self.origin.z;
        let y = 0.0;
        let z = (index[1] as f32 * self.tile_size) + self.origin.z;
        return Ok(Vec3::new(x, y, z));
    }

    //return the tile index by a world position
    fn world_pos_to_tile_index(&self, position: Vec3) -> Result<[usize; 2], &'static str> {
        if position.x > self.tiles.len() as f32 * self.tile_size || position.z > self.tiles[0].len() as f32 * self.tile_size {
            return Err("Index is greater than tilemap size.");
        }
        let i = ((position.x - self.origin.x) / self.tile_size) as usize;
        let j = ((position.z - self.origin.z) / self.tile_size) as usize;
        return Ok([i, j]);
    }
}

//----SYSTEMS----

fn handle_tilemap(
    mut tilemap: ResMut<TileMap>
){
    
}

fn draw_tilemap(
    mut gizmos: Gizmos,
    mut tilemap: ResMut<TileMap>
){
    let map_width = tilemap.tiles.len();
    let map_height = tilemap.tiles[0].len();

    let light = Color::Rgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 0.2 };
    let dark = Color::Rgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 0.4 };

    for i in 0..=map_width {
        let start = tilemap.tile_index_to_world_pos([i, 0]).unwrap();
        let end = tilemap.tile_index_to_world_pos([i, map_height]).unwrap();
        let color = if i % 10 == 0 {dark} else {light};
        gizmos.line(start, end, color);
    }

    for j in 0..=map_height {
        let start = tilemap.tile_index_to_world_pos([0, j]).unwrap();
        let end = tilemap.tile_index_to_world_pos([map_width, j]).unwrap();
        let color = if j % 10 == 0 {dark} else {light};
        gizmos.line(start, end, color);
    }
}