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
const SHOW_COLLIDERS: bool = true;
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
    pub target_translation: Option<Vec3>,
    pub target_rotation: Option<Quat>,
    pub target_scale: Option<Vec3>,
    pub speed: f32,
}

impl LerpMovement {
    pub fn new(
        speed: f32,
        translation: Option<Vec3>,
        rotation: Option<Quat>,
        scale: Option<Vec3>,
    ) -> Self {
        Self {
            target_translation: translation,
            target_rotation: rotation,
            target_scale: scale,
            speed,
        }
    }
    pub fn set_target_translation(&mut self, target: Vec3) {
        self.target_translation = Some(target);
    }
    pub fn set_target_rotation(&mut self, target: Quat) {
        self.target_rotation = Some(target);
    }
    pub fn set_target_scale(&mut self, target: Vec3) {
        self.target_scale = Some(target);
    }
}

impl Default for LerpMovement {
    fn default() -> Self {
        Self {
            target_translation: None,
            target_rotation: None,
            target_scale: None,
            speed: 30.0,
        }
    }
}

const TRANSLATION_DIFF: f32 = 0.01;
const ROTATION_DIFF: f32 = 0.9999;
const SCALE_DIFF: f32 = 0.01;

fn handle_lerp_movement(time: Res<Time>, mut query: Query<(&mut LerpMovement, &mut Transform)>) {
    for (mut lerp_movement, mut transform) in query.iter_mut() {
        if let Some(target) = lerp_movement.target_translation {
            let t = lerp_movement.speed * time.delta_seconds();
            let new_translation = transform.translation.lerp(target, t);

            if (new_translation - target).length() < TRANSLATION_DIFF {
                transform.translation = target;
                lerp_movement.target_translation = None;
            } else {
                transform.translation = new_translation;
            }
        }

        if let Some(target) = lerp_movement.target_rotation {
            let t = lerp_movement.speed * time.delta_seconds();
            let new_rotation = transform.rotation.lerp(target, t);

            if new_rotation.dot(target).abs() > ROTATION_DIFF {
                transform.rotation = target;
                lerp_movement.target_rotation = None;
            } else {
                transform.rotation = new_rotation;
            }
        }

        if let Some(target) = lerp_movement.target_scale {
            let t = lerp_movement.speed * time.delta_seconds();
            let new_scale = transform.scale.lerp(target, t);

            if (new_scale - target).length() < SCALE_DIFF {
                transform.scale = target;
                lerp_movement.target_scale = None;
            } else {
                transform.scale = new_scale;
            }
        }
    }
}
