use bevy::{prelude::*, input::mouse::{MouseMotion, MouseWheel}, window::PrimaryWindow};
use bevy::render::camera::Camera;
use bevy_rapier3d::prelude::*;
use super::super::Player;
use std::f32::consts::PI;

use super::components::*;

pub fn spawn_camera(
    mut commands: Commands
){
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-5.0, 5.0, 5.0)
            .looking_at(Vec3::ZERO, Vec3::new(0.0, 1.0, 0.0)),
            ..default()  
        },
        CameraDefault {
            focus: Vec3::new(0.0,0.0,0.0),
            mouse_sensitivity: 2.0,
            radius: 10.0,
            zoom_sensitivity: 1.0,
            zoom_bounds: (10.0, 50.0),
            button: MouseButton::Right
        }
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

            if up_vector.y > 0.5 && up_vector.y < 0.9{
                cam_transform.rotation = new_rotation;
            }
        }
    
    }
    let rot_matrix = Mat3::from_quat(cam_transform.rotation);
    cam_transform.translation = cam.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, cam.radius));

}

pub fn zoom_mouse(
    mut scroll_evr: EventReader<MouseWheel>, 
    mut cam_q: Query<&mut CameraDefault>
) {
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

pub fn mouse_click_world(
    buttons: Res<Input<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    cam_q: Query<(&Camera, &GlobalTransform), With<CameraDefault>>,
    rapier_context: Res<RapierContext>
) {
    if !buttons.just_pressed(MouseButton::Left) {return;}
    if let Some(position) = q_windows.single().cursor_position() {
        let Ok((camera, camera_transform)) = cam_q.get_single() else { return };
        let ray_option = camera.viewport_to_world(camera_transform, position);
    
        if let Some(ray) = ray_option {
            let ray_pos = ray.origin;
            let ray_dir = ray.direction;
            let max_toi = 100.0;
            let solid = true;
            let filter: QueryFilter = Default::default();
        
            if let Some((entity, toi)) = rapier_context.cast_ray(
                ray_pos, ray_dir, max_toi, solid, filter
            )  {
                let hit_point = ray_pos + ray_dir * toi;
                println!("Entity {:?} hit at point {}", entity, hit_point);
            }  
        }
    }
}