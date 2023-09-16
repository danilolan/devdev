use bevy::prelude::*;

use crate::player_interaction::picking::PickingData;
use crate::world::grid::Grid;
use crate::world::walls::WallPoints;

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
        for mut _point in self.points {
            _point = None;
        }
    }
}

const WALLSKEY: KeyCode = KeyCode::F1;
const WINDOWS_KEY: KeyCode = KeyCode::F2;
const DESTROYING_KEY: KeyCode = KeyCode::F3;

//----systems----
fn handle_state(mut app_state: ResMut<NextState<BuildingState>>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(WALLSKEY) {
        app_state.set(BuildingState::WALLS);
    }
    if keys.just_released(WINDOWS_KEY) {
        app_state.set(BuildingState::WINDOWS);
    }
    if keys.pressed(DESTROYING_KEY) {
        app_state.set(BuildingState::DESTROYING);
    }
}

fn handle_building_walls(
    picking: Res<PickingData>,
    grid: Res<Grid>,
    buttons: Res<Input<MouseButton>>,
    mut mouse_points: ResMut<MousePoints>,
    mut wall_points: ResMut<WallPoints>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        //get first point
        let hit_point = picking.get_hit_in_ground();
        mouse_points.points[0] = Some(grid.world_to_coord(hit_point));
    }

    if buttons.just_released(MouseButton::Left) {
        //get second point
        let hit_point = picking.get_hit_in_ground();
        mouse_points.points[1] = Some(grid.world_to_coord(hit_point));

        //change walls
        if let (Some(first_point), Some(second_point)) =
            (mouse_points.points[0], mouse_points.points[1])
        {
            wall_points.add_line([first_point, second_point]);
            wall_points.set_changed();
        }

        //reset
        mouse_points.reset();
    }
}

fn handle_building_windows(
    picking: Res<PickingData>,
    grid: Res<Grid>,
    buttons: Res<Input<MouseButton>>,
    mut wall_points: ResMut<WallPoints>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        //get first point
        let hit_point = picking.get_hit_in_ground();
        let hit_coord = grid.world_to_coord(hit_point);

        wall_points.add_window(hit_coord);
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum BuildingState {
    #[default]
    WALLS,
    WINDOWS,
    DESTROYING,
}
