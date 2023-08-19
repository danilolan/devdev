use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier3d::{prelude::{RapierContext, QueryFilter, Collider}, na::Rotation};

use crate::player_interaction::camera::CameraDefault;

pub struct PickingPlugin;

impl Plugin for PickingPlugin {
  fn build(&self, app: &mut App) {
    //resources
    app.init_resource::<PickingData>();

    //systems
    app.add_systems(Update, update_picking);
    app.add_systems(Update, show_colliders);
  }
}

//----resource----
#[derive(Resource)]
#[derive(Clone)]
pub struct PickingData {
  pub hit_position: Option<Vec3>,
  pub hit_position_ground: Option<Vec3>,
  pub entity: Option<Entity>
}

impl Default for PickingData {
  fn default() -> Self {
    PickingData {
      hit_position: None,
      hit_position_ground: None,
      entity: None
    }
  }

}

impl PickingData {
  pub fn set(&mut self, hit_position: Option<Vec3>, hit_position_ground: Option<Vec3>,entity: Option<Entity>) {
    self.hit_position = hit_position;
    self.entity = entity;
    self.hit_position_ground = hit_position_ground;
  }
}

//----systems----
fn update_picking(
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
  let ray_pos = ray.origin;
  let ray_dir = ray.direction;
  let max_toi = 100.0;
  let solid = true;
  let filter: QueryFilter = Default::default();

  //get the point that the ray hit the plane Y
  let t = -ray_pos.y / ray_dir.y;
  let hit_position_ground = ray_pos + t * ray_dir;

  match rapier_context.cast_ray(ray_pos, ray_dir, max_toi, solid, filter) {
    Some((entity, toi)) => {
      let hit_position = ray_pos + ray_dir * toi;
      picking.set(Some(hit_position),Some(hit_position_ground), Some(entity));
    }
    None => {
      picking.set(None, Some(hit_position_ground), None);
    }
  }
}

const SHOW_COLLIDER: bool = false;

fn show_colliders(
  colliders_q: Query<(&Transform, &Collider), With<Collider>>,
  mut gizmos: Gizmos,
) {
  if !SHOW_COLLIDER {return;}

  for (transform, collider) in colliders_q.iter() {
    let cubo = collider.as_cuboid().unwrap();
    let size = Vec3::new(cubo.raw.half_extents[0] * 2.0, cubo.raw.half_extents[1] * 2.0, cubo.raw.half_extents[2] * 2.0);
    let transform_point = Transform::from_translation(transform.translation).with_scale(size);
    gizmos.cuboid(transform_point, Color::GREEN);
  }
}