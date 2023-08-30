use bevy::prelude::*;

use crate::player_interaction::picking::PickingData;
use crate::world::grid::Grid;
use crate::world::walls::WallPoints;

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        //systems
        app.add_systems(Update, handle_building);

        //resources
        app.init_resource::<MousePoints>();
    }
}

//resources
#[derive(Resource)]
struct MousePoints {
    points: [Option<[i32; 2]>; 2],
}

impl Default for MousePoints {
    fn default() -> Self {
        MousePoints {
            points: [None, None],
        }
    }
}

impl MousePoints {
    fn reset(&mut self) {
        for mut point in self.points {
            point = None;
        }
    }
}

//----systems----
fn handle_building(
    picking: Res<PickingData>,
    grid: Res<Grid>,
    buttons: Res<Input<MouseButton>>,
    mut wall_points: ResMut<WallPoints>,
    mut mouse_points: ResMut<MousePoints>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        //get first point
        let hit_point = picking.hit_position_ground.unwrap_or_default();
        mouse_points.points[0] = Some(grid.world_to_coord(hit_point));
    }

    if buttons.just_released(MouseButton::Left) {
        //get second point
        let hit_point = picking.hit_position_ground.unwrap_or_default();
        mouse_points.points[1] = Some(grid.world_to_coord(hit_point));

        //change walls
        if let (Some(first_point), Some(second_point)) =
            (mouse_points.points[0], mouse_points.points[1])
        {
            wall_points.add_square(first_point, second_point);
        }

        //reset
        mouse_points.reset();
    }
}
