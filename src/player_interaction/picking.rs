use bevy::math::EulerRot;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier3d::prelude::{Collider, QueryFilter, RapierContext};

use crate::{player_interaction::camera::CameraDefault, world::physics::BoxCollider};

pub struct PickingPlugin;

impl Plugin for PickingPlugin {
    fn build(&self, app: &mut App) {
        //resources
        app.init_resource::<PickingData>();

        //systems
        app.add_systems(Update, handle_picking);
        app.add_systems(Update, test);
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
    pub fn get_entity(self, collider_query: Query<(Entity, &BoxCollider)>) -> Option<Entity> {
        for (entity, collider) in collider_query.iter() {
            if Self::intersects(self.ray, collider) {
                return Some(entity);
            }
        }
        None
    }

    pub fn get_hit_in_ground(self) -> Vec3 {
        let t = -self.ray.origin.y / self.ray.direction.y;
        let hit_position_ground = self.ray.origin + t * self.ray.direction;

        return hit_position_ground;
    }

    fn intersects(ray: Ray, collider: &BoxCollider) -> bool {
        let transform = Transform {
            translation: collider.translation,
            rotation: collider.rotation,
            scale: collider.scale,
        };
        // Transformar o ray para o espaço local do cubo
        let inv_rot = transform.rotation.inverse();
        let local_origin = inv_rot * (ray.origin - transform.translation);
        let local_direction = inv_rot * ray.direction;

        let min = -transform.scale / 2.0;
        let max = transform.scale / 2.0;

        // Passo 2: Verifique a interseção usando o método dos "slabs" no espaço local
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

        // A interseção é válida se tmin está dentro do intervalo [0, tmax]
        tmin >= 0.0 && tmin <= tmax
    }
}

//----systems----
fn handle_picking(
    mut picking: ResMut<PickingData>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    cam_q: Query<(&Camera, &GlobalTransform), With<CameraDefault>>,
    rapier_context: Res<RapierContext>,
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
