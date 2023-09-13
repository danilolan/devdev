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
        app.add_systems(Update, handle_walls);
        app.add_systems(Startup, start_walls);
        app.init_resource::<WallPoints>();
    }
}

//----systems----
fn start_walls(mut commands: Commands) {
    commands
        .spawn((WallMesh {}, Transform::default()))
        .insert(Name::from("walls".to_string()));
}
fn handle_walls(
    mut walls_points: ResMut<WallPoints>,
    server: Res<AssetServer>,
    mut commands: Commands,
    grid: Res<Grid>,
    query: Query<Entity, With<WallMesh>>,
) {
    if !walls_points.is_changed() {
        return;
    }

    let wall: Handle<Scene> = server.load("./models/wall.gltf#Scene0");
    let pillar: Handle<Scene> = server.load("./models/pillar.gltf#Scene0");

    if let Some(points) = &walls_points.line {
        let start = grid.coord_to_tile(points[0]);
        let end = grid.coord_to_tile(points[1]);
        let object_size = 0.4;
        let scale_multiplier = 2.5;

        let direction = end - start;
        let scale = direction.length();
        let rotation = Quat::from_rotation_arc(Vec3::X, direction.normalize());
        let midpoint = start + (direction / 2.0);

        //wall
        commands
            .spawn((
                (SceneBundle {
                    scene: wall.clone(),
                    transform: Transform {
                        translation: midpoint,
                        rotation,
                        scale: Vec3::new(scale * scale_multiplier, 1.0, 1.0),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
                BoxCollider {
                    scale: Vec3::new(scale + object_size, 2.0, object_size),
                    translation: midpoint,
                    rotation,
                },
            ))
            .insert(Name::from("wall".to_string()));

        walls_points.line = None;
    }

    //pillars
    for point in &walls_points.pillars {
        let translation = grid.coord_to_tile(*point);

        commands
            .spawn(
                (SceneBundle {
                    scene: pillar.clone(),
                    transform: Transform {
                        translation,
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            )
            .insert(Name::from("pillar".to_string()));
    }
    walls_points.pillars.clear();
}

//----resources----
#[derive(Resource)]
pub struct WallPoints {
    line: Option<[[i32; 2]; 2]>,
    pillars: Vec<[i32; 2]>,
    points: Vec<[i32; 2]>,
    connections: Vec<[usize; 2]>,
}

impl WallPoints {
    // Function to calculate the end point based on the initial point and the differences dx and dy
    fn calculate_point_end(line: [[i32; 2]; 2]) -> [i32; 2] {
        let [x1, y1] = line[0];
        let [x2, y2] = line[1];

        let dx = x2 - x1;
        let dy = y2 - y1;

        if dx.abs() < dy.abs() {
            [x1, y2]
        } else {
            [x2, y1]
        }
    }

    // Function to check if two points are equal
    fn points_are_equal(point1: [i32; 2], point2: [i32; 2]) -> bool {
        point1 == point2
    }

    // Function to add points and pillars if they don't already exist in the respective vectors
    fn add_points_and_pillars(&mut self, point_start: [i32; 2], point_end: [i32; 2]) {
        for &point in &[point_start, point_end] {
            if !self.points.contains(&point) {
                self.points.push(point);
                self.pillars.push(point);
            }
        }
    }

    pub fn add_line(&mut self, line: [[i32; 2]; 2]) {
        let point_start = line[0];

        // Using the helper function to calculate the end point
        let point_end = Self::calculate_point_end(line);

        // Using the helper function to check if the points are equal
        if Self::points_are_equal(point_start, point_end) {
            return;
        }

        // Using the helper function to add points and pillars
        self.add_points_and_pillars(point_start, point_end);

        // Find indices of start and end points in the points vector
        if let Some(index_start) = self.points.iter().position(|&x| x == point_start) {
            if let Some(index_end) = self.points.iter().position(|&x| x == point_end) {
                if !self.connections.contains(&[index_start, index_end]) {
                    self.line = Some([point_start, point_end]);
                    self.connections.push([index_start, index_end]);
                }
            }
        }
    }
}

impl Default for WallPoints {
    fn default() -> Self {
        WallPoints {
            line: None,
            pillars: Vec::new(),
            points: Vec::new(),
            connections: Vec::new(),
        }
    }
}

//----components----
#[derive(Component)]
struct WallMesh {}
