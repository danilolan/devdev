//file used to declare components, systems... to use in world physics

use bevy::{gizmos, prelude::*, transform};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, show_colliders);
        app.add_systems(Update, handle_colliders);
        app.add_systems(Update, handle_smooth_movement);
        app.add_systems(Update, handle_lerp_movement);
    }
}

//smooth movement
#[derive(Component)]
pub struct SmoothMovement {
    translation: Vec3,
    velocity: Vec3,
    pub acceleration: f32,
    pub desacceleration: f32,
    pub max_velocity: f32,
}

impl SmoothMovement {
    pub fn new(
        translation: Vec3,
        acceleration: f32,
        desacceleration: f32,
        max_velocity: f32,
        velocity: Vec3,
    ) -> Self {
        Self {
            translation,
            acceleration,
            desacceleration,
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
                self.velocity -= self.velocity.normalize() * self.desacceleration * dt;
            } else {
                self.velocity = Vec3::ZERO;
            }
        }

        // limit max velocity
        if self.velocity.length() > self.max_velocity {
            self.velocity = self.velocity.normalize() * self.max_velocity;
        }

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

//----lerp movement----
#[derive(Component)]
pub struct LerpMovement {
    pub target_position: Option<Vec3>,
    pub current_position: Vec3,
    pub speed: f32,
}

impl LerpMovement {
    pub fn new(speed: f32, start_translation: Vec3) -> Self {
        Self {
            target_position: None,
            current_position: start_translation,
            speed,
        }
    }
    pub fn set_target(&mut self, target: Vec3) {
        self.target_position = Some(target);
    }
}

fn handle_lerp_movement(time: Res<Time>, mut query: Query<(&mut LerpMovement, &mut Transform)>) {
    for (mut lerp_movement, mut transform) in query.iter_mut() {
        if let Some(target_position) = lerp_movement.target_position {
            let t = lerp_movement.speed * time.delta_seconds();

            transform.translation = transform.translation.lerp(target_position, t);

            lerp_movement.target_position = None;
        }
    }
}
