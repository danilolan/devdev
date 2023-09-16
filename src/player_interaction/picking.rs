use bevy::math::EulerRot;
use bevy::{prelude::*, window::PrimaryWindow};

use crate::{player_interaction::camera::CameraDefault, world::physics::BoxCollider};

pub struct PickingPlugin;

impl Plugin for PickingPlugin {
    fn build(&self, app: &mut App) {
        //resources
        app.init_resource::<PickingData>();

        //systems
        app.add_systems(Update, handle_picking);
        //app.add_systems(Update, test);
    }
}

//----resource----
#[derive(Resource, Clone, Copy)]
pub struct PickingData {
    ray: Ray,
}

impl Default for PickingData {
    fn default() -> Self {
        PickingData {
            ray: Ray::default(),
        }
    }
}

impl PickingData {
    pub fn get_entity(&self, collider_query: Query<(Entity, &BoxCollider)>) -> Option<Entity> {
        for (entity, collider) in collider_query.iter() {
            if self.intersects(collider) {
                return Some(entity);
            }
        }
        None
    }

    pub fn get_hit_in_ground(&self) -> Vec3 {
        let t = -self.ray.origin.y / self.ray.direction.y;
        let hit_position_ground = self.ray.origin + t * self.ray.direction;

        hit_position_ground
    }

    fn calculate_tmin_tmax(&self, transform: Transform) -> Option<(f32, f32)> {
        let inv_rot = transform.rotation.inverse();
        let local_origin = inv_rot * (self.ray.origin - transform.translation);
        let local_direction = inv_rot * self.ray.direction;

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
            return None;
        }

        Some((tmin, tmax))
    }

    fn intersects(&self, collider: &BoxCollider) -> bool {
        let transform = Transform {
            translation: collider.translation,
            rotation: collider.rotation,
            scale: collider.scale,
        };

        self.calculate_tmin_tmax(transform).is_some()
    }

    pub fn get_hit_point(&self, collider_query: Query<(Entity, &BoxCollider)>) -> Option<(Vec3)> {
        for (entity, collider) in collider_query.iter() {
            let transform = Transform {
                translation: collider.translation,
                rotation: collider.rotation,
                scale: collider.scale,
            };

            if let Some((tmin, _)) = self.calculate_tmin_tmax(transform) {
                let inv_rot = transform.rotation.inverse();
                let local_origin = inv_rot * (self.ray.origin - transform.translation);
                let local_direction = inv_rot * self.ray.direction;

                let hit_point_local = local_origin + tmin * local_direction;
                let hit_point_world = transform.rotation * hit_point_local + transform.translation;

                return Some(hit_point_world);
            }
        }

        None
    }
}

//----systems----
fn handle_picking(
    mut picking: ResMut<PickingData>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    cam_q: Query<(&Camera, &GlobalTransform), With<CameraDefault>>,
) {
    let position = match q_windows.single().cursor_position() {
        Some(pos) => pos,
        None => return,
    };
    let (camera, camera_transform) = match cam_q.get_single() {
        Ok((cam, transform)) => (cam, transform),
        Err(_) => return,
    };
    let ray = match camera.viewport_to_world(&camera_transform, position) {
        Some(r) => r,
        None => return,
    };

    picking.ray = ray;
}

fn test(mut picking: ResMut<PickingData>, collider_query: Query<(Entity, &BoxCollider)>) {
    let entity = picking.clone().get_entity(collider_query);
    println!("{:?}", entity);
}
