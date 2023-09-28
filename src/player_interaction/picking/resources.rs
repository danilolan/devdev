//! Handles the actual ray and your interactions

use bevy::prelude::*;

use crate::world::physics::components::BoxCollider;

#[derive(Resource, Clone, Copy)]
pub struct PickingData {
    pub ray: Ray,
}

impl Default for PickingData {
    fn default() -> Self {
        PickingData {
            ray: Ray::default(),
        }
    }
}

impl PickingData {
    /// Return an entity hitted by the ray
    pub fn get_entity<T: Component>(
        self,
        collider_query: Query<(Entity, &BoxCollider), With<T>>,
    ) -> Option<Entity> {
        for (entity, collider) in collider_query.iter() {
            if Self::intersects(self.ray, collider) {
                return Some(entity);
            }
        }
        None
    }

    /// Return the translation in the ground ( plane xz ) hitted by the ray
    pub fn get_hit_in_ground(self) -> Vec3 {
        let t = -self.ray.origin.y / self.ray.direction.y;
        let hit_position_ground = self.ray.origin + t * self.ray.direction;

        return hit_position_ground;
    }

    /// Check if ray intersects a box
    fn intersects(ray: Ray, collider: &BoxCollider) -> bool {
        let transform = Transform {
            translation: collider.translation,
            rotation: collider.rotation,
            scale: collider.scale,
        };

        let inv_rot = transform.rotation.inverse();
        let local_origin = inv_rot * (ray.origin - transform.translation);
        let local_direction = inv_rot * ray.direction;

        let min = -transform.scale / 2.0;
        let max = transform.scale / 2.0;

        let t1 = (min.x - local_origin.x) / local_direction.x;
        let t2 = (max.x - local_origin.x) / local_direction.x;
        let t3 = (min.y - local_origin.y) / local_direction.y;
        let t4 = (max.y - local_origin.y) / local_direction.y;
        let t5 = (min.z - local_origin.z) / local_direction.z;
        let t6 = (max.z - local_origin.z) / local_direction.z;

        let tmin = t1.min(t2).max(t3.min(t4)).max(t5.min(t6));
        let tmax = t1.max(t2).min(t3.max(t4)).min(t5.max(t6));

        if tmin > tmax {
            return false;
        }

        tmin >= 0.0 && tmin <= tmax
    }
}
