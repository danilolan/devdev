use bevy::prelude::*;
use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    window::PrimaryWindow,
};

use super::Player;
use std::f32::consts::PI;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Update, orbit_mouse);
        app.add_systems(Update, zoom_mouse);
    }
}

//Components
#[derive(Component)]
pub struct CameraDefault {
    pub focus: Vec3,
    pub radius: f32,
    pub mouse_sensitivity: f32,
    pub zoom_sensitivity: f32,
    pub zoom_bounds: (f32, f32),
    pub button: MouseButton,
}

//Systems
pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-5.0, 5.0, 5.0)
                .looking_at(Vec3::ZERO, Vec3::new(0.0, 1.0, 0.0)),
            ..default()
        },
        CameraDefault {
            focus: Vec3::new(0.0, 0.0, 0.0),
            mouse_sensitivity: 2.0,
            radius: 10.0,
            zoom_sensitivity: 1.0,
            zoom_bounds: (5.0, 70.0),
            button: MouseButton::Right,
        },
    ));
}

pub fn orbit_mouse(
    window_q: Query<&Window, With<PrimaryWindow>>,
    mut cam_q: Query<(&mut CameraDefault, &mut Transform), With<CameraDefault>>,
    mut mouse_evr: EventReader<MouseMotion>,
    player_q: Query<&Transform, (With<Player>, Without<CameraDefault>)>,
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
            cam_transform.rotation = yaw * cam_transform.rotation; // rotate around global y axis

            // Calculate the new rotation without applying it to the camera yet
            let new_rotation = cam_transform.rotation * pitch;

            // check if new rotation will cause camera to go beyond the 180 degree vertical bounds
            let up_vector = new_rotation * Vec3::Y;

            if up_vector.y > 0.5 && up_vector.y < 0.9 {
                cam_transform.rotation = new_rotation;
            }
        }
    }
    let rot_matrix = Mat3::from_quat(cam_transform.rotation);
    cam_transform.translation = cam.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, cam.radius));
}

pub fn zoom_mouse(mut scroll_evr: EventReader<MouseWheel>, mut cam_q: Query<&mut CameraDefault>) {
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
