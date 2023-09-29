use bevy::prelude::*;

use crate::world::{
    grid::resources::Grid,
    physics::components::{BoxCollider, LerpMovement},
};

/// Holds the current object data to will be placed.
#[derive(Resource)]
pub struct ObjectToolData {
    pub entity: Option<Entity>,
    pub grid_size: Option<f32>,
    pub current_angle: f32,
    pub angle_step: f32,
    pub entities_to_place: Vec<Entity>,
    pub entities_to_remove: Vec<Entity>,
}
impl Default for ObjectToolData {
    fn default() -> Self {
        ObjectToolData {
            entity: None,
            grid_size: Some(0.2),
            current_angle: 0.0,
            angle_step: 90.0,
            entities_to_place: Vec::new(),
            entities_to_remove: Vec::new(),
        }
    }
}
impl ObjectToolData {
    pub fn set_new_entity_in_tool(&mut self, entity: Entity, commands: &mut Commands) {
        if let Some(entity) = self.entity {
            commands.entity(entity).despawn_recursive();
        }

        commands
            .entity(entity)
            .insert(LerpMovement::new(25.0, None, None, None));

        self.entity = Some(entity);
    }

    pub fn delete_entity_in_tool(&mut self, commands: &mut Commands) {
        if let Some(entity) = self.entity {
            commands.entity(entity).despawn_recursive();
            self.entity = None;
        }
    }

    pub fn place_entity_in_world(&mut self) {
        if let Some(entity) = self.entity {
            self.entity = None;
            self.entities_to_place.push(entity);
        }
    }

    pub fn remove_entity_in_world(&mut self) {
        if let Some(entity) = self.entity {
            self.entity = None;
            self.entities_to_remove.push(entity)
        }
    }
}
