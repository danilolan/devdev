use bevy::prelude::*;

#[derive(Component)]
pub struct SmoothMovement {
    pub translation: Vec3,
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
        inset: f32,
    ) -> bool {
        for (other_entity, other) in others.iter() {
            if self_entity == other_entity {
                continue; // Skip the current entity
            }
            // Obtain the oriented axes of the BoxCollider
            let axes_a = self.get_axes();
            let axes_b = other.get_axes();

            let mut is_colliding = true; // initially assume it's colliding

            // Check if the projections along all axes are overlapping
            for axis in axes_a.iter().chain(axes_b.iter()) {
                if !self.projections_overlap(other, *axis, inset) {
                    is_colliding = false; // Projections do not overlap along this axis
                    break;
                }
            }

            // If, after all tests, we still believe it's colliding, then it's true
            if is_colliding {
                return true;
            }
        }
        return false; // If we get here, there was no collision with any of the other BoxColliders
    }

    /// Returns the three main axes of the BoxCollider
    fn get_axes(&self) -> [Vec3; 3] {
        let mat = Mat3::from_quat(self.rotation);
        [mat.col(0).into(), mat.col(1).into(), mat.col(2).into()]
    }

    /// Checks if the projections of two BoxColliders along an axis overlap
    fn projections_overlap(&self, other: &BoxCollider, axis: Vec3, inset: f32) -> bool {
        let self_proj = self.project_onto_axis(&axis);
        let other_proj = other.project_onto_axis(&axis);

        // Check if the projections overlap beyond the inset
        self_proj.0 <= (other_proj.1 + inset) && self_proj.1 >= (other_proj.0 - inset)
    }

    /// Projects the points of the BoxCollider onto an axis and returns the min and max
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

    /// Computes the corners of the BoxCollider
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
