use std::f32::consts::PI;

use bevy::{prelude::*, transform, render::camera, input::mouse::{MouseMotion, MouseWheel}, window::PrimaryWindow};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Update, plane_movement);
        app.add_systems(Update, orbit_mouse);
        app.add_systems(Update, sync_player_rotation);
        app.add_systems(Update, zoom_mouse);
    }
}

#[derive(Component)]
pub struct Player {}
#[derive(Component)]
pub struct Camera {
    pub focus: Vec3,
    pub radius: f32,
    pub mouse_sensitivity: f32,
    pub zoom_sensitivity: f32,
    pub zoom_bounds: (f32, f32),
    pub button: MouseButton
}

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
        Camera {
            focus: Vec3::new(0.0,0.0,0.0),
            mouse_sensitivity: 4.0,
            radius: 10.0,
            zoom_sensitivity: 1.0,
            zoom_bounds: (10.0, 50.0),
            button: MouseButton::Right
        }
    ));
}

fn orbit_mouse(
    window_q: Query<&Window, With<PrimaryWindow>>,
    mut cam_q: Query<(&mut Camera, &mut Transform), With<Camera>>,
    mut mouse_evr: EventReader<MouseMotion>,
    player_q: Query<&Transform, (With<Player>, Without<Camera>)>,
    buttons: Res<Input<MouseButton>>,
) {
    let mut rotation = Vec2::ZERO;
    for ev in mouse_evr.iter() {
        rotation = ev.delta;
    }

    let Ok((mut cam, mut cam_transform)) = cam_q.get_single_mut() else { return };
    let Ok(player_transform) = player_q.get_single() else { return };

    rotation *= cam.mouse_sensitivity;
    cam.focus = player_transform.translation;

    if rotation.length_squared() > 0.0 {
        let window = window_q.get_single().unwrap();
        let delta_x = {
            let delta = rotation.x / window.width() * std::f32::consts::PI;
            delta
        };

        let delta_y = rotation.y / window.height() * PI;
        let yaw = Quat::from_rotation_y(-delta_x);
        let pitch = Quat::from_rotation_x(-delta_y);
        if buttons.pressed(cam.button) {
            println!("down");
            cam_transform.rotation = yaw * cam_transform.rotation; // rotate around global y axis

            // Calculate the new rotation without applying it to the camera yet
            let new_rotation = cam_transform.rotation * pitch;

            // check if new rotation will cause camera to go beyond the 180 degree vertical bounds
            let up_vector = new_rotation * Vec3::Y;

            if up_vector.y > 0.0 && up_vector.y < 0.99{
                cam_transform.rotation = new_rotation;
            }
        }
    
    }
    let rot_matrix = Mat3::from_quat(cam_transform.rotation);
    cam_transform.translation = cam.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, cam.radius));

}

fn zoom_mouse(mut scroll_evr: EventReader<MouseWheel>, mut cam_q: Query<&mut Camera>) {
    let mut scroll = 0.0;
    for ev in scroll_evr.iter() {
        scroll += ev.y;
    }

    if let Ok(mut cam) = cam_q.get_single_mut() {
        if scroll.abs() >= 0.0 {
            let new_radius = cam.radius - scroll * cam.radius * 0.1 * cam.zoom_sensitivity;
            cam.radius = new_radius.clamp(cam.zoom_bounds.0, cam.zoom_bounds.1);
        }
    }
}