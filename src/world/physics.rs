//file used to declare components, systems... to use in world physics

use bevy::{gizmos, prelude::*, transform};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, show_colliders);
        app.add_systems(Update, handle_colliders);
        app.add_systems(Update, handle_smooth_movement);
    }
}

//smooth movement
#[derive(Component)]
pub struct SmoothMovement {
    pub translation: Vec3,
    k1: f32,
    k2: f32,
    k3: f32,
}

impl SmoothMovement {
    pub fn new(translation: Vec3, k1: f32, k2: f32, k3: f32) -> Self {
        Self {
            translation,
            k1: 0.3,
            k2: 0.02,
            k3: 0.0,
        }
    }
    pub fn change_translation(&mut self, translation: Vec3) {
        self.translation = translation;
        println!("{:?}", self.translation)
    }
}

fn handle_smooth_movement(
    mut query: Query<(&mut Transform, &SmoothMovement), With<SmoothMovement>>,
) {
    for (mut transform, movement) in query.iter_mut() {
        transform.translation = movement.translation;
    }
}

//collider
fn handle_colliders(mut collider_query: Query<(&mut BoxCollider, &Transform), With<BoxCollider>>) {
    for (mut collider, transform) in collider_query.iter_mut() {
        let translation = transform.translation;
        let rotation = transform.rotation;

        collider.translation = Vec3::new(
            translation.x,
            translation.y + (collider.scale.y / 2.0),
            translation.z,
        );
        collider.rotation = rotation;
    }
}
const SHOW_COLLIDERS: bool = false;
fn show_colliders(mut gizmos: Gizmos, collider_query: Query<&BoxCollider, With<BoxCollider>>) {
    if !SHOW_COLLIDERS {
        return;
    }
    for collider in collider_query.iter() {
        gizmos.cuboid(
            Transform {
                translation: collider.translation,
                rotation: collider.rotation,
                scale: collider.scale,
            },
            Color::GREEN,
        )
    }
}
#[derive(Component)]
pub struct BoxCollider {
    pub scale: Vec3,
    pub translation: Vec3,
    pub rotation: Quat,
}
