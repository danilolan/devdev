use crate::world::physics::LerpMovement;

use super::picking::PickingData;
use bevy::prelude::*;

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        //resources
        app.init_resource::<ObjectToolData>();

        //systems
        app.add_systems(Update, handle_object);
        app.add_systems(Update, place_object);
        //app.add_systems(Update, test);
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
            grid_size: Some(0.1),
        }
    }
}
//----systems----
fn handle_object(
    picking: Res<PickingData>,
    object_tool_data: ResMut<ObjectToolData>,
    mut global_query: Query<(&mut LerpMovement,)>,
) {
    if let Some(entity) = object_tool_data.entity {
        let hit_point = picking.get_hit_in_ground();
        if let Ok((mut lerp_movement,)) = global_query.get_mut(entity) {
            let position: Vec3 = match object_tool_data.grid_size {
                Some(grid_size) => (hit_point / grid_size).round() * grid_size,
                None => hit_point,
            };
            lerp_movement.set_target(position);
        }
    }
}

fn place_object(mut object_tool_data: ResMut<ObjectToolData>, buttons: Res<Input<MouseButton>>) {
    if buttons.just_pressed(MouseButton::Left) {
        object_tool_data.entity = None;
    }
}
