use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::components::*;

pub fn spawn_objects(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
){
    commands.spawn((
        Object {},
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube {
                size: 2.0
            })),
            material: materials.add(Color::rgb(0.1, 0.84, 0.92).into()),
            transform: Transform::from_xyz(5.0, 0.0, 5.0),
            ..default()
        },
        PopupState {
            is_open: false
        },
    )).insert(
        Collider::cuboid(1.0, 1.0, 1.0),
    );

    commands.spawn((
        Object {},
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube {
                size: 2.0
            })),
            material: materials.add(Color::rgb(0.1, 0.84, 0.92).into()),
            transform: Transform::from_xyz(-5.0, 0.0, -5.0),
            ..default()
        },
        PopupState {
            is_open: false
        },
    )).insert(
        Collider::cuboid(1.0, 1.0, 1.0),
    );
}

pub fn handle_popup_state(
    mut objects_query: Query<(Entity, &mut PopupState), With<Object>>
){
    for (entity, mut popup_state) in objects_query.iter_mut() {
        if popup_state.is_open {
            println!("{:?} Is open", entity);
        }
        else {
            println!("{:?} Is clse", entity);
        }
    }
}