use crate::world::grid::Grid;
use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

pub struct WallsPlugin;

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_walls);
        app.init_resource::<WallPoints>();
    }
}

//----resources----
#[derive(Resource)]
pub struct WallPoints {
    lines: Vec<[[i32; 2]; 2]>,
}

impl WallPoints {
    pub fn add_line(&mut self, line: [[i32; 2]; 2]) {
        if self.check_if_line_exists(line) {
            return;
        }

        let [x1, y1] = line[0];
        let [x2, y2] = line[1];

        // Calcular a distância em x e y
        let dx = x2 - x1;
        let dy = y2 - y1;

        let point2: [i32; 2];
        if dx.abs() < dy.abs() {
            point2 = [x1, y2]
        } else {
            point2 = [x2, y1]
        }

        self.lines.push([line[0], point2]);
    }
    pub fn check_if_line_exists(&self, line: [[i32; 2]; 2]) -> bool {
        if self.lines.contains(&line) {
            return true;
        }
        return false;
    }
}

impl Default for WallPoints {
    fn default() -> Self {
        WallPoints {
            lines: vec![[[0, 8], [0, 0]], [[0, 0], [10, 0]]],
        }
    }
}
//----components----
#[derive(Component)]
struct WallMesh {}
#[derive(Component)]
struct BaseboardMesh {}
#[derive(Component)]
struct TopMesh {}

const SIZE: f32 = 0.25;
const HEIGHT: f32 = 2.0;
const BASEBOARD_SIZE: f32 = 0.2;
const BASEBOARD_HEIGHT: f32 = 0.2;
const TOP_HEIGHT: f32 = 0.05;

//----systems----
fn handle_walls(
    walls_points: Res<WallPoints>,
    server: Res<AssetServer>,
    mut commands: Commands,
    grid: Res<Grid>,
) {
    if !walls_points.is_changed() {
        return;
    }

    let wall: Handle<Scene> = server.load("./models/wall.gltf#Scene0");

    for points in &walls_points.lines {
        let start = grid.coord_to_tile(points[0]);
        let end = grid.coord_to_tile(points[1]);

        commands
            .spawn(SceneBundle {
                scene: wall.clone(),
                transform: Transform::from_translation(start),
                ..Default::default()
            })
            .insert(Name::from("Wall".to_string()));
    }
}

fn handle_tops(
    walls_points: Res<WallPoints>,
    mut commands: Commands,
    grid: Res<Grid>,
    mut res_mesh: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    top_query: Query<(Entity, &TopMesh)>,
    mut mesh_query: Query<&mut Handle<Mesh>>,
) {
    if !walls_points.is_changed() {
        return;
    }
    let points = &walls_points.lines;
    let mut top_builder = MeshBuilder::new();

    //looping in all points
    for (index, &points) in points.iter().enumerate() {
        let start = grid.coord_to_tile(points[0]);
        let end = grid.coord_to_tile(points[1]);
        let direction = end - start;

        let start_offset = start + (direction.normalize() * -(0.25 * SIZE));
        let end_offset = end - (direction.normalize() * -(0.25 * SIZE));

        let mut start_top = start_offset;
        let mut end_top = end_offset;
        start_top.y = HEIGHT - TOP_HEIGHT;
        end_top.y = HEIGHT - TOP_HEIGHT;
        create_wall(start_top, end_top, SIZE, TOP_HEIGHT, &mut top_builder);
    }

    let mesh = top_builder.build();
    let mesh_handle = res_mesh.add(mesh);
    let material = materials.add(Color::rgb(0.03, 0.06, 0.05).into());

    if let Some((entity, _)) = top_query.iter().next() {
        // Se uma entidade WallMesh já existir
        *mesh_query.get_mut(entity).unwrap() = mesh_handle;
    } else {
        // Se ainda não houver entidade WallMesh, crie uma
        commands
            .spawn(PbrBundle {
                mesh: mesh_handle,
                material,
                ..Default::default()
            })
            .insert(TopMesh {});
    }
}

fn handle_baseboard(
    walls_points: Res<WallPoints>,
    mut commands: Commands,
    grid: Res<Grid>,
    mut res_mesh: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    baseboard_query: Query<(Entity, &BaseboardMesh)>,
    mut mesh_query: Query<&mut Handle<Mesh>>,
) {
    if !walls_points.is_changed() {
        return;
    }
    let points = &walls_points.lines;

    let mut baseboard_builder = MeshBuilder::new();

    //looping in all points
    for (index, &points) in points.iter().enumerate() {
        let start = grid.coord_to_tile(points[0]);
        let end = grid.coord_to_tile(points[1]);
        let direction = end - start;

        let baseboard_start_offset = start + (direction.normalize() * -(0.3 * SIZE));
        let baseboard_end_offset = end - (direction.normalize() * -(0.3 * SIZE));
        create_wall(
            baseboard_start_offset,
            baseboard_end_offset,
            SIZE + BASEBOARD_SIZE,
            BASEBOARD_HEIGHT,
            &mut baseboard_builder,
        );
    }

    let mesh = baseboard_builder.build();
    let mesh_handle = res_mesh.add(mesh);
    let material = materials.add(Color::rgb(0.2, 0.2, 0.2).into());

    if let Some((entity, _)) = baseboard_query.iter().next() {
        // Se uma entidade WallMesh já existir
        *mesh_query.get_mut(entity).unwrap() = mesh_handle;
    } else {
        // Se ainda não houver entidade WallMesh, crie uma
        commands
            .spawn(PbrBundle {
                mesh: mesh_handle,
                material,
                ..Default::default()
            })
            .insert(BaseboardMesh {});
    }

    let mesh = baseboard_builder.build();
    let mesh_handle = res_mesh.add(mesh);
    let material = materials.add(Color::rgb(0.1, 0.8, 0.1).into());

    if let Some((entity, _)) = baseboard_query.iter().next() {
        // Se uma entidade WallMesh já existir
        *mesh_query.get_mut(entity).unwrap() = mesh_handle;
    } else {
        // Se ainda não houver entidade WallMesh, crie uma
        commands
            .spawn(PbrBundle {
                mesh: mesh_handle,
                material,
                ..Default::default()
            })
            .insert(TopMesh {});
    }
}

//builds a mesh adding squares
pub struct MeshBuilder {
    vertices: Vec<Vec3>,
    indices: Vec<u32>,
    normals: Vec<Vec3>,
}

impl MeshBuilder {
    pub fn new() -> Self {
        MeshBuilder {
            vertices: Vec::new(),
            indices: Vec::new(),
            normals: Vec::new(),
        }
    }

    pub fn add_square(&mut self, p0: Vec3, p1: Vec3, p2: Vec3, p3: Vec3) {
        // Add the vertices to the list
        let start_index = self.vertices.len() as u32;
        self.vertices.push(p0);
        self.vertices.push(p1);
        self.vertices.push(p2);
        self.vertices.push(p3);

        // Calculate normals (assuming the points are co-planar and in a clockwise order)
        let normal = (p1 - p0).cross(p2 - p0).normalize();

        self.normals.push(normal);
        self.normals.push(normal);
        self.normals.push(normal);
        self.normals.push(normal);

        // Add indices to form the square as two triangles
        // Triangle 1: p0, p1, p2
        // Triangle 2: p0, p2, p3
        self.indices.push(start_index);
        self.indices.push(start_index + 1);
        self.indices.push(start_index + 2);
        self.indices.push(start_index);
        self.indices.push(start_index + 2);
        self.indices.push(start_index + 3);
    }

    pub fn build(&self) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, self.vertices.clone());
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals.clone());
        mesh.set_indices(Some(Indices::U32(self.indices.clone())));
        mesh
    }
}

//find points adjacents of a points returning a vec with bools telling if have a point in that direction or not
fn find_adjacent_points(points: &Vec<[i32; 2]>, current_point: [i32; 2]) -> [Option<usize>; 4] {
    let mut result = [None, None, None, None];

    let max_distance = 1000;
    // Cria uma hashset para busca rápida
    let points_set: std::collections::HashSet<_> = points.iter().collect();

    for distance in 1..=max_distance {
        let potential_points = [
            [current_point[0] + distance, current_point[1]], // up
            [current_point[0], current_point[1] + distance], // right
            [current_point[0] - distance, current_point[1]], // down
            [current_point[0], current_point[1] - distance], // left
        ];

        for (i, &potential_point) in potential_points.iter().enumerate() {
            if result[i].is_none() && points_set.contains(&potential_point) {
                result[i] = points.iter().position(|&p| p == potential_point);
            }
        }

        // Se todos os pontos adjacentes forem encontrados, saia do loop
        if result.iter().all(|&x| x.is_some()) {
            break;
        }
    }

    result
}

//creates a wall
pub fn create_wall(start: Vec3, end: Vec3, size: f32, height: f32, builder: &mut MeshBuilder) {
    let points = calc_wall_points(start, end, size, height);

    // add the squares of the wall
    builder.add_square(points[6], points[7], points[3], points[2]);
    builder.add_square(points[1], points[3], points[7], points[5]);
    builder.add_square(points[0], points[1], points[5], points[4]);
    builder.add_square(points[2], points[3], points[1], points[0]);
    builder.add_square(points[4], points[5], points[7], points[6]);
}

//creates the pillar hidden the faces that connected
pub fn create_pillar(
    position: Vec3,
    sides_connected: [Option<usize>; 4],
    size: f32,
    height: f32,
    builder: &mut MeshBuilder,
) {
    let points = calc_pillar_points(position, size, height);

    builder.add_square(points[1], points[5], points[7], points[3]);

    //up
    if !sides_connected[0].is_some() {
        builder.add_square(points[0], points[1], points[3], points[2]);
    }
    //right
    if !sides_connected[1].is_some() {
        builder.add_square(points[2], points[3], points[7], points[6]);
    }
    //down
    if !sides_connected[2].is_some() {
        builder.add_square(points[5], points[4], points[6], points[7]);
    }
    //left
    if !sides_connected[3].is_some() {
        builder.add_square(points[1], points[0], points[4], points[5]);
    }
}

//calc the pillar points used to draw the squares
fn calc_pillar_points(position: Vec3, size: f32, height: f32) -> [Vec3; 8] {
    let direction = Vec3::X;
    //find perpendicular points
    let start = Vec3::new(position.x + (size / 4.0), position.y, position.z);
    let end = Vec3::new(position.x - (size / 4.0), position.y, position.z);
    let (point_left_start, point_right_start) =
        calc_perpendicular_points(start, direction, size / 2.0);
    let (point_left_end, point_right_end) = calc_perpendicular_points(end, direction, size / 2.0);

    //create points vector with defined order
    let points = [
        point_left_start,
        Vec3::new(
            point_left_start.x,
            point_left_start.y + height,
            point_left_start.z,
        ),
        point_right_start,
        Vec3::new(
            point_right_start.x,
            point_right_start.y + height,
            point_right_start.z,
        ),
        point_left_end,
        Vec3::new(
            point_left_end.x,
            point_left_end.y + height,
            point_left_end.z,
        ),
        point_right_end,
        Vec3::new(
            point_right_end.x,
            point_right_end.y + height,
            point_right_end.z,
        ),
    ];

    return points;
}

//calc the wall points used to draw the squares
fn calc_wall_points(start: Vec3, end: Vec3, size: f32, height: f32) -> [Vec3; 8] {
    let direction = end - start;
    //find perpendicular points
    let (point_left_start, point_right_start) =
        calc_perpendicular_points(start, direction, size / 2.0);
    let (point_left_end, point_right_end) = calc_perpendicular_points(end, direction, size / 2.0);

    //create points vector with defined order
    let points = [
        point_left_start,
        Vec3::new(
            point_left_start.x,
            point_left_start.y + height,
            point_left_start.z,
        ),
        point_right_start,
        Vec3::new(
            point_right_start.x,
            point_right_start.y + height,
            point_right_start.z,
        ),
        point_left_end,
        Vec3::new(
            point_left_end.x,
            point_left_end.y + height,
            point_left_end.z,
        ),
        point_right_end,
        Vec3::new(
            point_right_end.x,
            point_right_end.y + height,
            point_right_end.z,
        ),
    ];

    return points;
}

//calc the perpendicular points based on a point and a direcion
fn calc_perpendicular_points(point: Vec3, direction: Vec3, size: f32) -> (Vec3, Vec3) {
    let direction_normalized = direction.normalize();

    let perpendicular = Vec2::new(-direction_normalized.z, direction_normalized.x);
    let perpendicular_point = Vec3::new(
        ((size / 2.0) * perpendicular).x,
        0.0,
        ((size / 2.0) * perpendicular).y,
    );
    let point_left = point + perpendicular_point;
    let point_right = point - perpendicular_point;

    (point_right, point_left)
}
