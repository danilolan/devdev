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
    translation: Vec3,
    velocity: Vec3,
    acceleration: f32,
    desaceleration: f32,
    max_velocity: f32,
}

impl SmoothMovement {
    pub fn new(
        translation: Vec3,
        acceleration: f32,
        desaceleration: f32,
        max_velocity: f32,
        velocity: Vec3,
    ) -> Self {
        Self {
            translation,
            acceleration,
            desaceleration,
            max_velocity,
            velocity,
        }
    }

    pub fn change_translation(&mut self, direction: Vec3, dt: f32) {
        if direction.length().abs() > 0.0 {
            // apply acceleration
            self.velocity += direction.normalize() * self.acceleration * dt;
        } else {
            // apply desacceleration
            if self.velocity.length().abs() > 0.0 {
                self.velocity -= self.velocity.normalize() * self.desaceleration * dt;
            } else {
                self.velocity = Vec3::ZERO;
            }
        }

        // limit max velocity
        if self.velocity.length() > self.max_velocity {
            self.velocity = self.velocity.normalize() * self.max_velocity;
        }

        println!("{:?}", self.velocity);

        // move the transform
        self.translation += self.velocity * dt;

        // avoid tiny velocitys
        if self.velocity.length() < 0.01 {
            self.velocity = Vec3::ZERO;
        }
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
