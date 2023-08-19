use bevy::{prelude::*, transform};
use bevy_rapier3d::prelude::Collider;
use super::picking::PickingData;

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
  fn build(&self, app: &mut App) {
    //resources
    app.init_resource::<ObjectToolData>();
    
    //systems
    app.add_systems(Update, handle_object);
    app.add_systems(Update, place_object);
    app.add_systems(Update, test);
  }
}

//----resources----
#[derive(Resource)]
pub struct ObjectToolData {
  pub entity: Option<Entity>,
  pub grid_size: Option<f32>,
}
impl Default for ObjectToolData {
  fn default() -> Self {
    ObjectToolData { 
      entity: None,
      grid_size: None
    }
  }
}
//----systems----
fn handle_object(
  asset_server: Res<AssetServer>,
  picking: Res<PickingData>,
  mut object_tool_data: ResMut<ObjectToolData>,
  mut global_query: Query<(&mut Transform,)>
) {
  if let Some(entity) = object_tool_data.entity {
    let hit_point = picking.hit_position_ground.unwrap_or_default();
    if let Ok((mut transform,)) = global_query.get_mut(entity) {
      let position = match object_tool_data.grid_size {
        Some(grid_size) => (hit_point / grid_size).round() * grid_size,
        None => hit_point,
      };
      transform.translation = position;
    }
  }
}

fn place_object(
  mut object_tool_data: ResMut<ObjectToolData>,
  buttons: Res<Input<MouseButton>>,
) {
  if buttons.just_pressed(MouseButton::Left) {
    object_tool_data.entity = None;
  }
}

fn test(
  mut object_tool_data: ResMut<ObjectToolData>,
  buttons: Res<Input<MouseButton>>,
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  if buttons.just_pressed(MouseButton::Right) {
    let entity = commands.spawn(
      (PbrBundle {
          mesh: meshes.add(Mesh::from(shape::Cube::from(shape::Cube { size: 2.0 }))),
          material: materials.add(Color::rgb(0.2, 0.8, 0.8).into()),
          transform: Transform::from_xyz(0.0, 0.0, 0.0),
          ..default()
      })
      ).insert(
        Collider::cuboid(1.0, 1.0, 1.0)
      ).id();
    object_tool_data.entity = Some(entity);
  }
}