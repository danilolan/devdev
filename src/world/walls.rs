use crate::world::grid::Grid;
use bevy::{
    gizmos,
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    transform::commands,
};

pub struct WallsPlugin;

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, start_wall);
        app.init_resource::<WallPoints>();
    }
}

//----resources----
#[derive(Resource)]
pub struct WallPoints {
    points: Vec<Vec3>,
}

impl Default for WallPoints {
    fn default() -> Self {
        WallPoints { points: Vec::new() }
    }
}
//----components----
#[derive(Component)]
struct WallMesh {}

const SIZE: f32 = 1.0;
const HEIGHT: f32 = 2.0;

//----systems----
fn start_wall(
    mut commands: Commands,
    grid: Res<Grid>,
    mut res_mesh: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let points: Vec<[i32; 2]> = vec![[0, 0], [10, 0], [10, 10], [0, -10], [5, 5], [10, 20]];
    let mut points_connected: Vec<[usize; 2]> = Vec::new();

    let mut builder = MeshBuilder::new();

    for (index, &point) in points.iter().enumerate() {
        let adjacent_points = find_adjacent_points(&points, point);

        for adjacent_index in adjacent_points.iter().filter_map(|&option| option) {
            if !points_connected.contains(&[index, adjacent_index])
                || !points_connected.contains(&[index, adjacent_index])
            {
                let start = grid.coord_to_tile(points[index]);
                let end = grid.coord_to_tile(points[adjacent_index]);
                create_wall(start, end, SIZE, HEIGHT, &mut builder);

                points_connected.push([index, adjacent_index])
            }
        }

        let pillar_position = grid.coord_to_tile(point);
        create_pillar(pillar_position, adjacent_points, SIZE, HEIGHT, &mut builder);
    }

    let mesh = builder.build();
    let mesh_handle = res_mesh.add(mesh);

    let material = materials.add(Color::rgb(0.2, 0.2, 0.2).into());
    commands.spawn(PbrBundle {
        mesh: mesh_handle,
        material,
        ..Default::default()
    });
}

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
        // Os vértices são adicionados à lista
        let start_index = self.vertices.len() as u32;
        self.vertices.push(p0);
        self.vertices.push(p1);
        self.vertices.push(p2);
        self.vertices.push(p3);

        // Cálculo das normais (supondo que os pontos são coplanares e em sentido horário)
        let normal = (p1 - p0).cross(p2 - p0).normalize();

        self.normals.push(normal);
        self.normals.push(normal);
        self.normals.push(normal);
        self.normals.push(normal);

        // Adicionando índices para formar o quadrado como dois triângulos
        // Triângulo 1: p0, p1, p2
        // Triângulo 2: p0, p2, p3
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

fn find_adjacent_points(points: &Vec<[i32; 2]>, current_point: [i32; 2]) -> [Option<usize>; 4] {
    let mut result = [None, None, None, None];

    for (index, &point) in points.iter().enumerate() {
        // Para cima
        if point == [current_point[0], current_point[1] + 10] {
            result[1] = Some(index);
        }
        // Para a cima
        if point == [current_point[0] + 10, current_point[1]] {
            result[0] = Some(index);
        }
        // Para baixo
        if point == [current_point[0], current_point[1] - 10] {
            result[3] = Some(index);
        }
        // Para a baixo
        if point == [current_point[0] - 10, current_point[1]] {
            result[2] = Some(index);
        }
    }

    result
}

pub fn create_wall(start: Vec3, end: Vec3, size: f32, height: f32, builder: &mut MeshBuilder) {
    let points = calc_wall_points(start, end, size, height);

    // Adicionando o primeiro quadrado
    builder.add_square(points[6], points[7], points[3], points[2]);
    builder.add_square(points[1], points[3], points[7], points[5]);
    builder.add_square(points[0], points[1], points[5], points[4]);
}

pub fn create_pillar(
    position: Vec3,
    sides_connected: [Option<usize>; 4],
    size: f32,
    height: f32,
    builder: &mut MeshBuilder,
) {
    let points = calc_pillar_points(position, size, height);

    // Adicionando o primeiro quadrado
    builder.add_square(points[1], points[5], points[7], points[3]);

    if !sides_connected[0].is_some() {
        builder.add_square(points[0], points[1], points[3], points[2]);
    }
    if !sides_connected[1].is_some() {
        builder.add_square(points[2], points[3], points[7], points[6]);
    }
    if !sides_connected[2].is_some() {
        builder.add_square(points[5], points[4], points[6], points[7]);
    }
    if !sides_connected[3].is_some() {
        builder.add_square(points[1], points[0], points[4], points[5]);
    }
}

fn calc_pillar_points(position: Vec3, size: f32, height: f32) -> [Vec3; 8] {
    let direction = Vec3::X;
    //find perpendicular points
    let start = Vec3::new(position.x + (size / 4.0), position.y, position.z);
    let end = Vec3::new(position.x - (size / 4.0), position.y, position.z);
    let (point_left_start, point_right_start) =
        calc_perpendicular_points(start, direction, (size / 2.0));
    let (point_left_end, point_right_end) = calc_perpendicular_points(end, direction, (size / 2.0));

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
    println!("direction {:?}", direction);
    println!("{:?}", points);

    return points;
}

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
