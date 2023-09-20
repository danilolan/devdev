use std::collections::{HashMap, HashSet};

use crate::world::grid::Grid;
use bevy::{
    gizmos,
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    transform,
};

use super::physics::BoxCollider;

pub struct WallsPlugin;

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BuildingTiles>();
        app.add_systems(Update, show_tiles);
        app.add_systems(Update, (despawn_phase, render_walls).chain());
    }
}

pub fn despawn_phase(
    mut commands: Commands,
    building_tiles: Res<BuildingTiles>,
    query: Query<(Entity, &Wall)>,
) {
    let recently_updated_tiles = building_tiles.recently_updated_tiles.clone();

    let mut to_despawn = HashSet::new();

    for &[x, y] in &recently_updated_tiles {
        for (entity, wall) in query.iter() {
            if wall.position[0] == x && wall.position[1] == y {
                to_despawn.insert(entity);
            }
        }
    }

    for entity in to_despawn {
        commands.entity(entity).despawn();
    }
}

pub fn render_walls(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut building_tiles: ResMut<BuildingTiles>,
) {
    let recently_updated_tiles = building_tiles.recently_updated_tiles.clone();

    let mut to_spawn = Vec::new();

    for &[x, y] in &recently_updated_tiles {
        let tile_data = &building_tiles.tiles[x as usize][y as usize];
        let center_pos = building_tiles.grid.coord_to_tile([x, y]);

        for (i, &wall_exists) in tile_data.directions.iter().enumerate() {
            if wall_exists != 0 {
                let (rotation, offset) = match i {
                    0 => (
                        Quat::from_rotation_y(std::f32::consts::FRAC_PI_2),
                        Vec3::new(0.0, 0.0, -building_tiles.grid.tile_size / 2.0),
                    ),
                    1 => (
                        Quat::from_rotation_y(0.0),
                        Vec3::new(building_tiles.grid.tile_size / 2.0, 0.0, 0.0),
                    ),
                    2 => (
                        Quat::from_rotation_y(std::f32::consts::FRAC_PI_2),
                        Vec3::new(0.0 / 2.0, 0.0, building_tiles.grid.tile_size / 2.0),
                    ),
                    3 => (
                        Quat::from_rotation_y(0.0),
                        Vec3::new(-building_tiles.grid.tile_size / 2.0, 0.0, 0.0),
                    ),
                    _ => unreachable!(),
                };

                let wall: Handle<Scene> = asset_server.load("./models/wall.gltf#Scene0");

                to_spawn.push((
                    SceneBundle {
                        scene: wall.clone(),
                        transform: Transform {
                            translation: center_pos + offset,
                            rotation,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Wall {
                        position: [x, y],
                        direction: i,
                    },
                    Name::from("wall".to_string()),
                ));
            }
        }
    }
    // Spawn todas as novas entidades
    for entity in to_spawn {
        commands.spawn(entity);
    }

    building_tiles.recently_updated_tiles.clear();
}

#[derive(Component)]
pub struct Wall {
    position: [i32; 2],
    direction: usize,
}

const TILE_SIZE: f32 = 1.0;
const WORLD_SIZE: usize = 50;
const ROOM: i32 = 1;
//1+ - rooms
//0 - empty
//-1 - wall
//-2 - window
//-3 - door
#[derive(Clone)]
struct Tile {
    directions: [i32; 4],
    room: i32,
}

#[derive(Resource)]

pub struct BuildingTiles {
    pub grid: Grid,
    tiles: Vec<Vec<Tile>>,
    recently_updated_tiles: HashSet<[i32; 2]>,
    pub wall_index: HashMap<[i32; 2], Vec<Entity>>,
}

impl Default for BuildingTiles {
    fn default() -> Self {
        let tile = Tile {
            directions: [0; 4],
            room: 0,
        };

        Self {
            grid: Grid {
                tile_size: TILE_SIZE,
            },
            tiles: vec![vec![tile.clone(); WORLD_SIZE]; WORLD_SIZE],
            recently_updated_tiles: HashSet::new(),
            wall_index: HashMap::new(),
        }
    }
}

impl BuildingTiles {
    fn calc_walls_in_tiles(&mut self) {
        let mut tiles_to_update: HashSet<[i32; 2]> = HashSet::new();

        for &[i, j] in &self.recently_updated_tiles {
            let x = i as usize;
            let y = j as usize;

            let current_room = self.tiles[x][y].room;

            let directions = [
                if y == 0 || self.tiles[x][y - 1].room != current_room {
                    1
                } else {
                    0
                },
                if x == WORLD_SIZE - 1 || self.tiles[x + 1][y].room != current_room {
                    1
                } else {
                    0
                },
                if y == WORLD_SIZE - 1 || self.tiles[x][y + 1].room != current_room {
                    1
                } else {
                    0
                },
                if x == 0 || self.tiles[x - 1][y].room != current_room {
                    1
                } else {
                    0
                },
            ];

            println!("{:?}", directions);

            self.tiles[x][y].directions = directions;

            tiles_to_update.insert([x as i32, y as i32]);

            // Adicionar tiles adjacentes para revisão
            if x > 0 {
                tiles_to_update.insert([x as i32 - 1, y as i32]);
            }
            if x < WORLD_SIZE - 1 {
                tiles_to_update.insert([x as i32 + 1, y as i32]);
            }
            if y > 0 {
                tiles_to_update.insert([x as i32, y as i32 - 1]);
            }
            if y < WORLD_SIZE - 1 {
                tiles_to_update.insert([x as i32, y as i32 + 1]);
            }
        }

        self.recently_updated_tiles.extend(tiles_to_update);
    }

    //set just one tile to a room number
    pub fn set_room_to_tile(&mut self, position: [i32; 2]) {
        if Self::check_position_in_range(position) {
            return;
        }

        let (x, y) = (position[0] as usize, position[1] as usize);

        //seting the room
        self.tiles[x][y].room = ROOM;

        // Add the tile to the list of recently updated tiles
        self.recently_updated_tiles.insert([x as i32, y as i32]);

        //calculating the walls for each updated tiles
        self.calc_walls_in_tiles();
    }

    //set many tiles to a room number, receive two points that defines a rect
    pub fn set_room_to_tiles(&mut self, positions: [[i32; 2]; 2]) {
        let (x1, y1) = (positions[0][0] as usize, positions[0][1] as usize);
        let (x2, y2) = (positions[1][0] as usize, positions[1][1] as usize);

        let x_min = x1.min(x2);
        let x_max = x1.max(x2);

        let y_min = y1.min(y2);
        let y_max = y1.max(y2);

        //seting each tile of square
        for x in x_min..=x_max {
            for y in y_min..=y_max {
                self.tiles[x][y].room = ROOM;

                // Add the tile to the list of recently updated tiles
                self.recently_updated_tiles.insert([x as i32, y as i32]);
            }
        }

        //calculating the tiles
        self.calc_walls_in_tiles();
    }

    //check if position is in range between 0 and WORLD SIZE
    fn check_position_in_range(position: [i32; 2]) -> bool {
        if (position[0] < 0 || position[0] > WORLD_SIZE as i32)
            || (position[1] < 0 || position[1] > WORLD_SIZE as i32)
        {
            return true;
        }

        return false;
    }
}

fn show_tiles(building_tiles: Res<BuildingTiles>, mut gizmos: Gizmos) {
    for i in 0..WORLD_SIZE {
        for j in 0..WORLD_SIZE {
            let position = building_tiles.grid.coord_to_tile([i as i32, j as i32]);
            let rotation = Quat::from_rotation_x(std::f32::consts::PI / 2.0);

            let mut color = Color::GRAY;

            if building_tiles.tiles[i][j].room == 1 {
                color = Color::GREEN;
            }
            if building_tiles.tiles[i][j].room == 2 {
                color = Color::YELLOW;
            }
            if building_tiles.tiles[i][j].room == 3 {
                color = Color::BLUE;
            }
            if building_tiles.tiles[i][j].room == 4 {
                color = Color::CYAN;
            }

            gizmos.rect(
                position,
                rotation,
                Vec2::new(
                    building_tiles.grid.tile_size - 0.1,
                    building_tiles.grid.tile_size - 0.1,
                ),
                color,
            )
        }
    }
}
