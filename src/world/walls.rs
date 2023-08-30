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
    points: Vec<[i32; 2]>,
}

impl WallPoints {
    pub fn add_square(&mut self, first: [i32; 2], second: [i32; 2]) {
        let points = self.calc_square_vertices(first, second);

        for point in points {
            self.add_point(point);
        }
    }
    pub fn add_point(&mut self, point: [i32; 2]) {
        if self.check_if_point_exists(point) {
            return;
        }

        self.points.push(point);
    }
    pub fn check_if_point_exists(&self, point: [i32; 2]) -> bool {
        if self.points.contains(&point) {
            return true;
        }
        return false;
    }

    fn calc_square_vertices(&self, p1: [i32; 2], p2: [i32; 2]) -> [[i32; 2]; 4] {
        let a = [p1[0], p1[1]];
        let b = [p1[0], p2[1]];
        let c = [p2[0], p2[1]];
        let d = [p2[0], p1[1]];

        [a, b, c, d]
    }
}

impl Default for WallPoints {
    fn default() -> Self {
        WallPoints {
            points: vec![[0, 0], [10, 0], [10, 10], [0, 10]],
        }
    }
}
//----components----
#[derive(Component)]
struct WallMesh {}

const SIZE: f32 = 0.5;
const HEIGHT: f32 = 2.0;

//----systems----
fn handle_walls(
    walls_points: Res<WallPoints>,
    mut commands: Commands,
    grid: Res<Grid>,
    mut res_mesh: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &WallMesh)>,
    mut mesh_query: Query<&mut Handle<Mesh>>,
) {
    if !walls_points.is_changed() {
        return;
    }
    let points = &walls_points.points;
    //vector to hold the state of points that already connected
    let mut points_connected: Vec<[usize; 2]> = Vec::new();

    let mut builder = MeshBuilder::new();

    //looping in all points
    for (index, &point) in points.iter().enumerate() {
        let adjacent_points = find_adjacent_points(&points, point);

        //looping in all connected sides in this point
        for adjacent_index in adjacent_points.iter().filter_map(|&option| option) {
            //only continue if the points_points connected does not contain actual point and adjacent point connection
            if !points_connected.contains(&[index, adjacent_index])
                || !points_connected.contains(&[index, adjacent_index])
            {
                //get the starts and end position with the pillar offset
                let start = grid.coord_to_tile(points[index]);
                let end = grid.coord_to_tile(points[adjacent_index]);
                let direction = end - start;
                let start_offset = start + (direction.normalize() * (0.25 * SIZE));
                let end_offset = end - (direction.normalize() * (0.25 * SIZE));

                //create walls
                create_wall(start_offset, end_offset, SIZE, HEIGHT, &mut builder);

                points_connected.push([index, adjacent_index])
            }
        }

        //create pillars
        let pillar_position = grid.coord_to_tile(point);
        create_pillar(pillar_position, adjacent_points, SIZE, HEIGHT, &mut builder);
    }

    //render the mesh
    let mesh = builder.build();
    let mesh_handle = res_mesh.add(mesh);
    let material = materials.add(Color::rgb(0.2, 0.2, 0.2).into());

    if let Some((entity, _)) = query.iter().next() {
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
            .insert(WallMesh {});
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

    for (index, &point) in points.iter().enumerate() {
        //up
        if point == [current_point[0] + 10, current_point[1]] {
            result[0] = Some(index);
        }
        //right
        if point == [current_point[0], current_point[1] + 10] {
            result[1] = Some(index);
        }
        //down
        if point == [current_point[0] - 10, current_point[1]] {
            result[2] = Some(index);
        }
        //left
        if point == [current_point[0], current_point[1] - 10] {
            result[3] = Some(index);
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
