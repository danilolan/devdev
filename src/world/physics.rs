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

//----collider

#[derive(Component)]
pub struct BoxCollider {
    pub scale: Vec3,
    pub translation: Vec3,
    pub rotation: Quat,
}

impl BoxCollider {
    pub fn new(translation: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self {
            scale,
            translation,
            rotation,
        }
    }
    pub fn is_colliding_with(
        &self,
        self_entity: Entity,
        others: &Query<(Entity, &BoxCollider)>,
    ) -> bool {
        for (other_entity, other) in others.iter() {
            if self_entity == other_entity {
                continue; // Skip the current entity
            }
            // Adquira os eixos orientados do BoxCollider
            let axes_a = self.get_axes();
            let axes_b = other.get_axes();

            let mut is_colliding = true; // pressupomos inicialmente que está colidindo

            // Verifique se as projeções ao longo de todos os eixos estão se sobrepondo
            for axis in axes_a.iter().chain(axes_b.iter()) {
                if !self.projections_overlap(other, *axis) {
                    is_colliding = false; // As projeções não se sobrepõem ao longo deste eixo
                    break;
                }
            }

            // Se, depois de todos os testes, ainda acreditarmos que está colidindo, então é verdade
            if is_colliding {
                return true;
            }
        }
        return false; // Se chegarmos aqui, não houve colisão com nenhum dos outros BoxColliders
    }

    // Retorna os três eixos principais do BoxCollider
    fn get_axes(&self) -> [Vec3; 3] {
        let mat = Mat3::from_quat(self.rotation);
        [mat.col(0).into(), mat.col(1).into(), mat.col(2).into()]
    }

    // Verifica se as projeções de dois BoxColliders ao longo de um eixo se sobrepõem
    fn projections_overlap(&self, other: &BoxCollider, axis: Vec3) -> bool {
        let self_proj = self.project_onto_axis(&axis);
        let other_proj = other.project_onto_axis(&axis);

        // Verifique se as projeções se sobrepõem
        self_proj.0 <= other_proj.1 && self_proj.1 >= other_proj.0
    }

    // Projeta os pontos do BoxCollider em um eixo e retorna o mínimo e o máximo
    fn project_onto_axis(&self, axis: &Vec3) -> (f32, f32) {
        let corners = self.get_corners();
        let mut min = f32::INFINITY;
        let mut max = f32::NEG_INFINITY;

        for corner in &corners {
            let dot = corner.dot(*axis);
            min = min.min(dot);
            max = max.max(dot);
        }

        (min, max)
    }

    // Calcula os cantos do BoxCollider
    fn get_corners(&self) -> [Vec3; 8] {
        let half_extents = self.scale * 0.5;
        let rot_matrix = Mat3::from_quat(self.rotation);

        let mut corners = [Vec3::ZERO; 8];
        for i in 0..8 {
            let sign = Vec3::new(
                (i & 1) as f32 * 2.0 - 1.0,
                ((i >> 1) & 1) as f32 * 2.0 - 1.0,
                ((i >> 2) & 1) as f32 * 2.0 - 1.0,
            );
            corners[i] = self.translation + rot_matrix.mul_vec3(half_extents * sign);
        }

        corners
    }
}

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
