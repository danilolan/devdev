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
        app.add_systems(Update, rotate_object);
    }
}

//----resources----
#[derive(Resource)]
pub struct ObjectToolData {
    pub entity: Option<Entity>,
    pub grid_size: Option<f32>,
    pub current_angle: f32,
    pub angle_step: f32,
}
impl Default for ObjectToolData {
    fn default() -> Self {
        ObjectToolData {
            entity: None,
            grid_size: Some(0.2),
            current_angle: 0.0,
            angle_step: 90.0,
        }
    }
}
impl ObjectToolData {
    pub fn set_new_entity(&mut self, entity: Entity, commands: &mut Commands) {
        if let Some(entity) = self.entity {
            commands.entity(entity).despawn_recursive();
        }

        commands
            .entity(entity)
            .insert(LerpMovement::new(25.0, None, None, None));

        self.entity = Some(entity);
    }

    pub fn delete_entity(&mut self, commands: &mut Commands) {
        if let Some(entity) = self.entity {
            commands.entity(entity).despawn_recursive();
            self.entity = None;
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

            lerp_movement.set_target_translation(position);
        }
    }
}

fn rotate_object(
    mut object_tool_data: ResMut<ObjectToolData>,
    mut query: Query<(&mut Transform, &mut LerpMovement)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let entity = match object_tool_data.entity {
        Some(entity) => entity,
        None => return,
    };

    let (mut transform, mut lerp_movement) = match query.get_mut(entity) {
        Ok(transform) => transform,
        Err(_) => return,
    };

    if keyboard_input.just_pressed(KeyCode::E) {
        object_tool_data.current_angle += object_tool_data.angle_step;
        let new_rotation = object_tool_data.current_angle.to_radians();
        lerp_movement.set_target_rotation(Quat::from_rotation_y(new_rotation));
    }

    if keyboard_input.just_pressed(KeyCode::Q) {
        object_tool_data.current_angle -= object_tool_data.angle_step;
        let new_rotation = object_tool_data.current_angle.to_radians();
        lerp_movement.set_target_rotation(Quat::from_rotation_y(new_rotation));
    }
}

fn place_object(mut object_tool_data: ResMut<ObjectToolData>, buttons: Res<Input<MouseButton>>) {
    if buttons.just_pressed(MouseButton::Left) {
        object_tool_data.entity = None;
    }
}
