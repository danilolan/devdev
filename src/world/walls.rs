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
        app.add_systems(Update, render_walls);
    }
}

pub fn render_walls(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut building_tiles: ResMut<BuildingTiles>,
    mut query: Query<&mut Visibility>,
) {
    let recently_updated_tiles = building_tiles.recently_updated_tiles.clone();

    for &[x, y] in &recently_updated_tiles {
        let tile_size = building_tiles.grid.tile_size;
        let center_pos = building_tiles.grid.coord_to_tile([x, y]);

        let tile_data = &mut building_tiles.tiles[x as usize][y as usize];

        for entity_option in tile_data.entities.iter_mut() {
            if let Some(entity) = entity_option.take() {
                let mut visibility = query.get_mut(entity).unwrap();
                *visibility = Visibility::Hidden;
            }
        }

        for (i, &wall_exists) in tile_data.directions.iter().enumerate() {
            if wall_exists != 0 {
                let (rotation, offset) = match i {
                    0 => (
                        Quat::from_rotation_y(std::f32::consts::FRAC_PI_2),
                        Vec3::new(0.0, 0.0, -tile_size / 2.0),
                    ),
                    1 => (
                        Quat::from_rotation_y(0.0),
                        Vec3::new(tile_size / 2.0, 0.0, 0.0),
                    ),
                    2 => (
                        Quat::from_rotation_y(std::f32::consts::FRAC_PI_2),
                        Vec3::new(0.0 / 2.0, 0.0, tile_size / 2.0),
                    ),
                    3 => (
                        Quat::from_rotation_y(0.0),
                        Vec3::new(-tile_size / 2.0, 0.0, 0.0),
                    ),
                    _ => unreachable!(),
                };

                let mut translation = center_pos + offset;
                translation.y = 1.5 / 2.0;

                let wall: Handle<Scene> = asset_server.load("./models/wall.gltf#Scene0");

                let entity = commands
                    .spawn((
                        SceneBundle {
                            scene: wall.clone(),
                            transform: Transform {
                                translation: center_pos + offset,
                                rotation,
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        Name::from(y.to_string()),
                    ))
                    .id();

                tile_data.entities[i] = Some(entity);
            }
        }
    }

    building_tiles.recently_updated_tiles.clear();
}

#[derive(Component)]
pub struct Wall {
    room: i32,
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
    entities: [Option<Entity>; 4],
    room: i32,
}

#[derive(Resource)]

pub struct BuildingTiles {
    pub grid: Grid,
    tiles: Vec<Vec<Tile>>,
    recently_updated_tiles: HashSet<[i32; 2]>,
    pub wall_index: HashMap<[i32; 2], Vec<Entity>>,
    pub current_room: i32,
    pub tool: i32,
}

impl Default for BuildingTiles {
    fn default() -> Self {
        let tile = Tile {
            directions: [0; 4],
            room: 0,
            entities: [None; 4],
        };

        Self {
            grid: Grid {
                tile_size: TILE_SIZE,
            },
            tiles: vec![vec![tile.clone(); WORLD_SIZE]; WORLD_SIZE],
            recently_updated_tiles: HashSet::new(),
            wall_index: HashMap::new(),
            tool: -1,
            current_room: 1,
        }
    }
}

impl BuildingTiles {
    fn calc_walls_in_tiles(&mut self) {
        for &[i, j] in &self.recently_updated_tiles {
            let x = i as usize;
            let y = j as usize;

            println!("{:?}", self.tool);

            let mut directions = [
                if y == 0 || self.tiles[x][y - 1].room != self.current_room {
                    1
                } else {
                    0
                },
                if x == WORLD_SIZE - 1 || self.tiles[x + 1][y].room != self.current_room {
                    1
                } else {
                    0
                },
                if y == WORLD_SIZE - 1 || self.tiles[x][y + 1].room != self.current_room {
                    1
                } else {
                    0
                },
                if x == 0 || self.tiles[x - 1][y].room != self.current_room {
                    1
                } else {
                    0
                },
            ];

            self.tiles[x][y].directions = directions;
        }
    }

    //set just one tile to a room number
    pub fn set_room_to_tile(&mut self, position: [i32; 2]) {
        if !Self::check_position_in_range(position) {
            return;
        }

        let (x, y) = (position[0] as usize, position[1] as usize);

        // setting the room
        self.tiles[x][y].room = self.current_room;

        // Add the tile to the list of recently updated tiles
        self.recently_updated_tiles.insert([x as i32, y as i32]);

        // Add adjacent tiles to the update set
        let mut tiles_to_update: HashSet<[i32; 2]> = HashSet::new();
        Self::add_adjacent_tiles(x, y, &self.tiles, &mut tiles_to_update, self.current_room);
        self.recently_updated_tiles.extend(tiles_to_update);

        // Calculate the walls for each updated tile
        self.calc_walls_in_tiles();
    }

    //set many tiles to a room number, receive two points that defines a rect
    pub fn set_room_to_tiles(&mut self, positions: [[i32; 2]; 2]) {
        for position in positions {
            if !Self::check_position_in_range(position) {
                return;
            }
        }
        let (x1, y1) = (positions[0][0] as usize, positions[0][1] as usize);
        let (x2, y2) = (positions[1][0] as usize, positions[1][1] as usize);

        let x_min = x1.min(x2);
        let x_max = x1.max(x2);

        let y_min = y1.min(y2);
        let y_max = y1.max(y2);

        //seting each tile of square
        for x in x_min..=x_max {
            for y in y_min..=y_max {
                // Add the tile to the list of recently updated tiles
                self.set_room_to_tile([x as i32, y as i32]);
            }
        }

        //calculating the tiles
        self.calc_walls_in_tiles();
    }

    //check if position is in range between 0 and WORLD SIZE
    fn check_position_in_range(position: [i32; 2]) -> bool {
        if (position[0] >= 0 && position[0] <= WORLD_SIZE as i32)
            && (position[1] >= 0 && position[1] < WORLD_SIZE as i32)
        {
            return true;
        }

        return false;
    }

    fn add_adjacent_tiles(
        x: usize,
        y: usize,
        tiles: &Vec<Vec<Tile>>,
        set: &mut HashSet<[i32; 2]>,
        current_room: i32,
    ) {
        if x > 0 && tiles[x - 1][y].room == current_room {
            set.insert([x as i32 - 1, y as i32]);
        }
        if x < WORLD_SIZE - 1 && tiles[x + 1][y].room == current_room {
            set.insert([x as i32 + 1, y as i32]);
        }
        if y > 0 && tiles[x][y - 1].room == current_room {
            set.insert([x as i32, y as i32 - 1]);
        }
        if y < WORLD_SIZE - 1 && tiles[x][y + 1].room == current_room {
            set.insert([x as i32, y as i32 + 1]);
        }
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
