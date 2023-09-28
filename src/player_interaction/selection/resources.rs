use bevy::prelude::*;

use crate::world::physics::components::LerpMovement;

/// Holds the current object data to will be placed.
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
