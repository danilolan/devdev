use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier3d::prelude::{RapierContext, QueryFilter};

use crate::player_interaction::camera::CameraDefault;

pub struct PickingPlugin;

impl Plugin for PickingPlugin {
  fn build(&self, app: &mut App) {
    //resources
    app.init_resource::<Picking>();

    //systems
    app.add_systems(Update, update_picking);
  }
}

//----resource----
#[derive(Resource)]
#[derive(Clone)]
struct Picking {
  hit_position: Option<Vec3>,
  entity: Option<Entity>
}

impl Default for Picking {
  fn default() -> Self {
    Picking {
      hit_position: None,
      entity: None
    }
  }

}

impl Picking {
  fn set(&mut self, hit_position: Option<Vec3>,entity: Option<Entity>) {
    self.hit_position = hit_position;
    self.entity = entity;
  }
}

//----systems----
fn update_picking(
  mut picking: ResMut<Picking>,
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

  match rapier_context.cast_ray(ray_pos, ray_dir, max_toi, solid, filter) {
      Some((entity, toi)) => {
          let hit_position = ray_pos + ray_dir * toi;
          picking.set(Some(hit_position), Some(entity));
      }
      None => {
          picking.set(None, None);
      }
  }
}