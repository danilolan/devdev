use crate::world::grid::Grid;
use bevy::{
    gizmos,
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

use super::physics::BoxCollider;

pub struct WallsPlugin;

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BuildingTiles>();
        app.add_systems(Update, handle_tiles);
        app.add_systems(Update, show_tiles);
    }
}

use bevy::prelude::*;

// ...

pub fn handle_tiles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut building_tiles: ResMut<BuildingTiles>,
) {
    if !building_tiles.is_changed() {
        return;
    }

    /* for i in 0..50 {
        let mut string = "".to_string();
        for j in 0..50 {
            string = string + &building_tiles.tiles[i][j].room.to_string() + "-";
        }
        println!("{:?}", string);
    } */

    for &[x, y] in &building_tiles.recently_updated_tiles {
        // Get the current tile's information
        let tile_data = &building_tiles.tiles[x as usize][y as usize];

        // Get the world center position of the current tile
        let center_pos = building_tiles.grid.coord_to_tile([x, y]);

        // For each direction, check if a wall should be spawned
        for (i, &wall_exists) in tile_data.directions.iter().enumerate() {
            if wall_exists != 0 {
                // Determine rotation and position based on the direction
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

                // Create the entity with the appropriate mesh, material, and transformation
                let wall: Handle<Scene> = asset_server.load("./models/wall.gltf#Scene0");
                commands
                    .spawn(
                        (SceneBundle {
                            scene: wall.clone(),
                            transform: Transform {
                                translation: center_pos + offset,
                                rotation,
                                ..Default::default()
                            },
                            ..Default::default()
                        }),
                    )
                    .insert(Name::from("wall".to_string()));
            }
        }
    }

    &building_tiles.recently_updated_tiles.clear();
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
    recently_updated_tiles: Vec<[i32; 2]>,
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
            recently_updated_tiles: Vec::new(),
        }
    }
}

impl BuildingTiles {
    fn calc_walls_in_tiles(&mut self) {
        for &[i, j] in &self.recently_updated_tiles {
            let x = i as usize;
            let y = j as usize;

            // Get the current room
            let current_room = self.tiles[x][y].room;

            // Check the tiles around to determine where the walls should be placed
            let directions = [
                // Check if it is at the top boundary or if the room above is different
                if y == 0 || self.tiles[x][y - 1].room != current_room {
                    1
                } else {
                    0
                },
                // Check if it is at the right boundary or if the room to the right is different
                if x == WORLD_SIZE - 1 || self.tiles[x + 1][y].room != current_room {
                    1
                } else {
                    0
                },
                // Check if it is at the bottom boundary or if the room below is different
                if y == WORLD_SIZE - 1 || self.tiles[x][y + 1].room != current_room {
                    1
                } else {
                    0
                },
                // Check if it is at the left boundary or if the room to the left is different
                if x == 0 || self.tiles[x - 1][y].room != current_room {
                    1
                } else {
                    0
                },
            ];

            println!("{:?}", [x, y]);
            println!("{:?}", directions);

            // Set the current tile with the new wall directions
            self.tiles[x][y].directions = directions;
        }
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
        self.recently_updated_tiles.push([x as i32, y as i32]);

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
                self.recently_updated_tiles.push([x as i32, y as i32]);
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
