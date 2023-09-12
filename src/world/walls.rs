use crate::world::grid::Grid;
use bevy::{
    gizmos,
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
use bevy_rapier3d::prelude::Collider;

pub struct WallsPlugin;

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_walls);
        app.add_systems(Startup, start_walls);
        app.init_resource::<WallPoints>();
    }
}

//----resources----
#[derive(Resource)]
pub struct WallPoints {
    line: Option<[[i32; 2]; 2]>,
}

impl WallPoints {
    pub fn add_line(&mut self, line: [[i32; 2]; 2]) {
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

        self.line = Some([line[0], point2]);
    }
}

impl Default for WallPoints {
    fn default() -> Self {
        WallPoints { line: None }
    }
}
//----components----
#[derive(Component)]
struct WallMesh {}

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

    if let Some(points) = walls_points.line {
        let start = grid.coord_to_tile(points[0]);
        let end = grid.coord_to_tile(points[1]);

        let direction = end - start;
        let scale = direction.length();
        let rotation = Quat::from_rotation_arc(Vec3::X, direction.normalize());
        let midpoint = start + (direction / 2.0);
        let scale_multiplier = 10.0;
        let collider = Collider::cuboid(2.0, 1.0, 1.0);

        commands
            .spawn((
                SceneBundle {
                    scene: wall.clone(),
                    transform: Transform {
                        translation: midpoint,
                        rotation,
                        scale: Vec3::new(scale * scale_multiplier, 1.0, 1.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                collider,
            ))
            .insert(Name::from("wall".to_string()));

        walls_points.line = None;
    }
}
