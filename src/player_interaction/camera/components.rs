use bevy::prelude::*;

/// The default component for main camera.

#[derive(Component)]
pub struct CameraDefault {
    pub focus: Vec3,
    /// the point where the camera are looking for
    pub radius: f32,
    /// the actual radius of the camera formed by the distance between the focus and the translation for the camera
    pub target_radius: f32,
    /// a target radius to use with lerp function
    pub mouse_sensitivity: f32,
    /// the sensitivy of the orbit camera
    pub zoom_sensitivity: f32,
    pub zoom_bounds: (f32, f32),
    /// a tuple that holds the max and min zoom possible
    pub button: MouseButton,
}
