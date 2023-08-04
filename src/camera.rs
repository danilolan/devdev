use bevy::{prelude::*, transform, render::camera};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, plane_movement);
    }
}

#[derive(Component)]
pub struct Player {
    transform: Transform
}

pub fn spawn_player(
    mut commands: Commands
){
    commands.spawn(Player {
        transform: Transform::from_xyz(-0.0, 5.3, 11.5)
    })
    .insert(
        Camera3dBundle {
            transform: Transform::from_xyz(-5.0, 5.0, 5.0)
            .looking_at(Vec3::ZERO, Vec3::new(0.0, 1.0, 0.0)),
            ..default()  
        },
    );
}

pub fn plane_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>
){
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 1.0)
        }
        if keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, -1.0)
        }
        if keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(-1.0,0.0 ,1.0)
        }
        if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(1.0, 0.0, -1.0)
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation +=  direction * 5.0 * time.delta_seconds();
    }
}