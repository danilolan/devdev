use bevy::prelude::*;

#[derive(Component)]
pub struct CameraDefault {
    pub focus: Vec3,
    pub radius: f32,
    pub mouse_sensitivity: f32,
    pub zoom_sensitivity: f32,
    pub zoom_bounds: (f32, f32),
    pub button: MouseButton
}