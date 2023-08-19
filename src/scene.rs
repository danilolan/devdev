use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_scene);
    }
}

pub fn spawn_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
){
    commands.spawn(
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane::from_size(1000.0))),
            material: materials.add(Color::rgb(0.8, 0.8, 0.8).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    );

    commands.spawn(
        (PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube::from(shape::Cube { size: 2.0 }))),
            material: materials.add(Color::rgb(0.2, 0.8, 0.8).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
    ).insert(
        Collider::cuboid(1.0, 1.0, 1.0)
    );

    commands.spawn(DirectionalLightBundle{
        directional_light: DirectionalLight{
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.7, 0.5, 0.0)),
        ..Default::default()
    });
}