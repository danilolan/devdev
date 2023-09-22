use bevy::prelude::*;

use crate::player_interaction::picking::PickingData;
use crate::world::grid::Grid;
use crate::world::physics::BoxCollider;
use crate::world::walls::BuildingTiles;

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        //systems
        app.add_systems(Update, handle_state);
        app.add_systems(
            Update,
            handle_building_walls.run_if(in_state(BuildingState::WALLS)),
        );
        app.add_systems(
            Update,
            handle_building_windows.run_if(in_state(BuildingState::WINDOWS)),
        );

        //resources
        app.init_resource::<MousePoints>();

        //states
        app.add_state::<BuildingState>();
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
        for i in 0..self.points.len() {
            self.points[i] = None;
        }
    }
}

const WALLSKEY: KeyCode = KeyCode::F1;
const WINDOWS_KEY: KeyCode = KeyCode::F2;
const DESTROYING_KEY: KeyCode = KeyCode::F3;

//----systems----
fn handle_state(
    mut app_state: ResMut<NextState<BuildingState>>,
    keys: Res<Input<KeyCode>>,
    mut building_tiles: ResMut<BuildingTiles>,
) {
    if keys.just_pressed(WALLSKEY) {
        app_state.set(BuildingState::WALLS);
    }
    if keys.just_released(WINDOWS_KEY) {
        building_tiles.current_room += 1;
    }
    if keys.pressed(DESTROYING_KEY) {
        building_tiles.current_room = 0;
    }
}

fn handle_building_walls(
    picking: Res<PickingData>,
    buttons: Res<Input<MouseButton>>,
    mut mouse_points: ResMut<MousePoints>,
    mut building_tiles: ResMut<BuildingTiles>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        //get first point
        let hit_point = picking.get_hit_in_ground();
        mouse_points.points[0] = Some(building_tiles.grid.world_to_coord(hit_point));
    }

    if buttons.just_released(MouseButton::Left) {
        //get second point
        let hit_point = picking.get_hit_in_ground();
        let point = Some(building_tiles.grid.world_to_coord(hit_point));

        if point != mouse_points.points[0] {
            mouse_points.points[1] = point;
        }

        //change walls
        match (mouse_points.points[0], mouse_points.points[1]) {
            (Some(ref first_point), Some(ref second_point)) => {
                building_tiles.set_room_to_tiles([*first_point, *second_point]);
                // Set that resource changed
                building_tiles.set_changed()
            }
            (Some(ref first_point), None) => {
                building_tiles.set_room_to_tile(*first_point);
                // Set that resource changed
                building_tiles.set_changed()
            }
            _ => {}
        }

        //reset
        mouse_points.reset();
    }
}

fn handle_building_windows(
    picking: Res<PickingData>,
    grid: Res<Grid>,
    buttons: Res<Input<MouseButton>>,
    mut mouse_points: ResMut<MousePoints>,
    mut building_tiles: ResMut<BuildingTiles>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        //get first point
        let hit_point = picking.get_hit_in_ground();
        mouse_points.points[0] = Some(building_tiles.grid.world_to_coord(hit_point));
    }

    if buttons.just_released(MouseButton::Left) {
        //get second point
        let hit_point = picking.get_hit_in_ground();
        let point = Some(building_tiles.grid.world_to_coord(hit_point));

        if point != mouse_points.points[0] {
            mouse_points.points[1] = point;
        }

        //change walls
        match (mouse_points.points[0], mouse_points.points[1]) {
            (Some(ref first_point), Some(ref second_point)) => {
                building_tiles.set_room_to_tiles([*first_point, *second_point]);
                // Set that resource changed
                building_tiles.set_changed()
            }
            (Some(ref first_point), None) => {
                building_tiles.set_room_to_tile(*first_point);
                // Set that resource changed
                building_tiles.set_changed()
            }
            _ => {}
        }

        //reset
        mouse_points.reset();
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum BuildingState {
    #[default]
    WALLS,
    WINDOWS,
    DESTROYING,
}
