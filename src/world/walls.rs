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
        app.add_systems(Update, start_wall);
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

//structs
struct Walls {
    height: f32,
    size: f32,
    meshes: Mesh,
}

impl Walls {
    fn new(height: f32, size: f32) -> Self {
        Walls {
            height,
            size,
            meshes: Mesh::new(PrimitiveTopology::TriangleList),
        }
    }

    //0-1-5-0-5-4,2-3-6-2-6-7,1-3-6-1-6-5
    pub fn draw_wall(self, start: Vec3, end: Vec3, commands: Commands, mut gizmos: Gizmos) {
        let direction = end - start;
        //find perpendicular points
        let (point_left_start, point_right_start) =
            perpendicular_points(start, direction, self.size / 2.0);
        let (point_left_end, point_right_end) =
            perpendicular_points(end, direction, self.size / 2.0);

        //create points vector with defined order
        let points = [
            point_left_start,
            Vec3::new(
                point_left_start.x,
                point_left_start.y + self.height,
                point_left_start.z,
            ),
            point_right_start,
            Vec3::new(
                point_right_start.x,
                point_right_start.y + self.height,
                point_right_start.z,
            ),
            point_left_end,
            Vec3::new(
                point_left_end.x,
                point_left_end.y + self.height,
                point_left_end.z,
            ),
            point_right_end,
            Vec3::new(
                point_right_end.x,
                point_right_end.y + self.height,
                point_right_end.z,
            ),
        ];
        for point in points {
            gizmos.sphere(point, Quat::default(), 0.1, Color::RED);
        }
    }

    fn create_triangle(vertices: [Vec3; 3], indices: [u32; 3]) -> Mesh {
        let normal = (vertices[1] - vertices[0])
            .cross(vertices[2] - vertices[0])
            .normalize();
        let normals = vec![normal; 3];

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices.to_vec());
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.set_indices(Some(Indices::U32(indices.to_vec())));

        mesh
    }
}
fn perpendicular_points(point: Vec3, direction: Vec3, size: f32) -> (Vec3, Vec3) {
    let direction_normalized = direction.normalize();

    let perpendicular = Vec2::new(-direction_normalized.y, direction_normalized.x);
    let perpendicular_point = Vec3::new(
        ((size / 2.0) * perpendicular).x,
        0.0,
        ((size / 2.0) * perpendicular).y,
    );
    let point_left = point + perpendicular_point;
    let point_right = point - perpendicular_point;

    (point_left, point_right)
}

//----components----
#[derive(Component)]
struct WallMesh {}

//----systems----
const HEIGHT: f32 = 10.0;
fn start_wall(
    mut wall_points: ResMut<WallPoints>,
    grid: Res<Grid>,
    commands: Commands,
    gizmos: Gizmos,
) {
    let walls = Walls::new(2.0, 1.0);

    walls.draw_wall(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(10.0, 0.0, 0.0),
        commands,
        gizmos,
    );
}
