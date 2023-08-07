use bevy::{prelude::*, transform, render::camera};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Update, plane_movement);
        app.add_systems(Update, camera_movement);
        app.add_systems(Update, sync_player_rotation);
    }
}

#[derive(Component)]
pub struct Player {}
#[derive(Component)]
pub struct Camera {}

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
){
    commands.spawn((
        Player {},
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube {
                size: 2.0
            })),
            material: materials.add(Color::rgb(0.8, 0.84, 0.12).into()),
            ..default()
        },
        Name::new("player")
    ));
}

pub fn plane_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>
){
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 0.0, 1.0)
        }
        if keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(-0.0, 0.0, -1.0)
        }
        if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(-1.0,0.0 ,0.0)
        }
        if keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(1.0, 0.0, 0.0)
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
            direction = transform.rotation.mul_vec3(direction);
        }

        transform.translation +=  direction * 5.0 * time.delta_seconds();
    }
}

pub fn sync_player_rotation(
    mut player_q: Query<&mut Transform, (With<Player>, Without<Camera>)>,
    cam_q: Query<&Transform, With<Camera>>,
) {
    let Ok(mut player_transform) = player_q.get_single_mut() else { return };
    let Ok(camera_transform) = cam_q.get_single() else { return };

    let mut target = camera_transform.translation;
    target.y = 0.0;
    player_transform.look_at(target, Vec3::Y);
} 

pub fn spawn_camera(
    mut commands: Commands
){
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-5.0, 5.0, 5.0)
            .looking_at(Vec3::ZERO, Vec3::new(0.0, 1.0, 0.0)),
            ..default()  
        },
        Camera {}
    ));
}


pub fn camera_movement(
    player_q: Query<&Transform, (With<Player>, Without<Camera>)>,
    mut cam_q: Query<&mut Transform, With<Camera>>,
) {
    let Ok(player_transform) = player_q.get_single() else { return };
    let Ok(mut camera_transform) = cam_q.get_single_mut() else { return };

    camera_transform.look_at(player_transform.translation, Vec3::Y);

    let offset = Vec3::new(-5.0, -5.0, 5.0);
    camera_transform.translation = player_transform.translation - offset;
} 