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
}
impl Default for ObjectToolData {
    fn default() -> Self {
        ObjectToolData {
            entity: None,
            grid_size: Some(0.1),
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
            .insert(LerpMovement::new(50.0, Vec3::ZERO));

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

            lerp_movement.set_target(position);
        }
    }
}

fn rotate_object(
    object_tool_data: ResMut<ObjectToolData>,
    mut query: Query<&mut Transform>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let entity = match object_tool_data.entity {
        Some(entity) => entity,
        None => return,
    };

    let mut transform = match query.get_mut(entity) {
        Ok(transform) => transform,
        Err(_) => return,
    };

    if keyboard_input.just_pressed(KeyCode::E) {
        transform.rotation *= Quat::from_rotation_y(90.0_f32.to_radians());
    }

    if keyboard_input.just_pressed(KeyCode::Q) {
        transform.rotation *= Quat::from_rotation_y((-90.0_f32).to_radians());
    }
}

fn place_object(mut object_tool_data: ResMut<ObjectToolData>, buttons: Res<Input<MouseButton>>) {
    if buttons.just_pressed(MouseButton::Left) {
        object_tool_data.entity = None;
    }
}
