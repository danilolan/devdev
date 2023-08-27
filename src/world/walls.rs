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
        app.init_resource::<Walls>();
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

//structs
#[derive(Resource)]
struct Walls {
    height: f32,
    size: f32,
    pub meshes: Option<Handle<Mesh>>,
}

impl Default for Walls {
    fn default() -> Self {
        Walls {
            height: 2.0,
            size: 1.0,
            meshes: None,
        }
    }
}

impl Walls {
    //0-1-5-0-5-4,2-3-6-2-6-7,1-3-6-1-6-5
    pub fn add_wall(
        &mut self,
        start: Vec3,
        end: Vec3,
        mut meshes: ResMut<Assets<Mesh>>,
        mut gizmos: Gizmos,
    ) -> Handle<Mesh> {
        let size = self.size;
        let height = self.height;
        let points = calc_wall_points(start, end, size, height);
        let indices = [0, 1, 5, 0, 5, 4, 2, 3, 6, 2, 6, 7, 1, 3, 6, 1, 6, 5];

        let mut builder = MeshBuilder::new();

        // Adicionando o primeiro quadrado
        builder.add_square(points[6], points[7], points[3], points[2]);
        builder.add_square(points[1], points[3], points[7], points[5]);
        builder.add_square(points[0], points[1], points[5], points[4]);
        let mesh = builder.build();

        let mesh_handle = meshes.add(mesh);
        self.meshes = Some(mesh_handle.clone());

        mesh_handle
    }
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
    println!("{:?}", points);

    return points;
}

fn calc_perpendicular_points(point: Vec3, direction: Vec3, size: f32) -> (Vec3, Vec3) {
    let direction_normalized = direction.normalize();

    let perpendicular = Vec2::new(-direction_normalized.y, direction_normalized.x);
    let perpendicular_point = Vec3::new(
        ((size / 2.0) * perpendicular).x,
        0.0,
        ((size / 2.0) * perpendicular).y,
    );
    let point_left = point + perpendicular_point;
    let point_right = point - perpendicular_point;

    (point_right, point_left)
}

//----components----
#[derive(Component)]
struct WallMesh {}

//----systems----
fn start_wall(
    mut walls: ResMut<Walls>,
    mut commands: Commands,
    gizmos: Gizmos,
    mut res_mesh: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    walls.add_wall(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(10.0, 0.0, 0.0),
        res_mesh,
        gizmos,
    );

    let material = materials.add(Color::rgb(0.2, 0.2, 0.2).into());
    if let Some(mesh) = walls.meshes.clone() {
        commands.spawn(PbrBundle {
            mesh,
            material,
            ..Default::default()
        });
    }
}

struct MeshBuilder {
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
