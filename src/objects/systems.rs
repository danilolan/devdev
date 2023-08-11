use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::components::*;

pub fn spawn_object(
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
            isOpen: false
        },
    )).insert(
        Collider::cuboid(1.0, 1.0, 1.0),
    );
}

pub fn handle_popup_state(
    mut objects_query: Query<&mut PopupState, With<Object>>
){
    if let Ok(mut popup_state) = objects_query.get_single_mut() {
        if popup_state.isOpen {
            println!("Is open");
        }
        else {
            println!("Is closed")
        }
    }
}