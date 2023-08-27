use bevy::{prelude::*, render::render_resource::PrimitiveTopology};

pub struct ObjectPlugin;

impl Plugin for ObjectPlugin {
    fn build(&self, app: &mut App) {
        //app.add_systems(Startup, setup);
    }
}

//----components----
#[derive(Component)]
pub struct Object {}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let wall_mesh = create_wall();
    let wall_material = materials.add(Color::rgb(0.8, 0.7, 0.6).into());
    let tile_size = 1.0;
    let rows = 5;
    let cols = 5;
    let wall_width = 0.1;

    for i in 0..rows {
        for j in 0..cols {
            if i == 0 || i == rows - 1 || j == 0 || j == cols - 1 {
                let mut transform = Transform::from_translation(Vec3::new(
                    i as f32 * tile_size,
                    0.0,
                    j as f32 * tile_size,
                ));
                commands.spawn(PbrBundle {
                    mesh: meshes.add(wall_mesh.clone()),
                    material: wall_material.clone(),
                    transform,
                    ..Default::default()
                });

                transform.translation += if i == 0 {
                    Vec3::new(-wall_width, 0.0, 0.0)
                } else if i == rows - 1 {
                    Vec3::new(wall_width, 0.0, 0.0)
                } else if j == 0 {
                    Vec3::new(0.0, 0.0, -wall_width)
                } else {
                    Vec3::new(0.0, 0.0, wall_width)
                };

                commands.spawn(PbrBundle {
                    mesh: meshes.add(wall_mesh.clone()),
                    material: wall_material.clone(),
                    transform,
                    ..Default::default()
                });
            }
        }
    }
}

fn create_wall() -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [1.0, 0.0, 0.0],
        ],
    );
    mesh
}
