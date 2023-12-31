use bevy::prelude::*;

pub mod camera;
use camera::CameraPlugin;
pub mod picking;
use picking::PickingPlugin;
pub mod selection;
use selection::SelectionPlugin;
pub mod building;
use building::BuildingPlugin;

use crate::world::physics::components::SmoothMovement;

pub struct PlayerInteractionPlugin;

impl Plugin for PlayerInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, plane_movement);
        app.add_systems(Update, sync_player_rotation);
        app.add_systems(Update, proportional_smooth_values);

        //plugins
        app.add_plugins(CameraPlugin);
        app.add_plugins(PickingPlugin);
        app.add_plugins(SelectionPlugin);
        app.add_plugins(BuildingPlugin);
    }
}

//Components
#[derive(Component)]
pub struct Player {}

//Systems
const ACCELERATION: f32 = 5.0;
const DESACELERATION: f32 = 5.0;
const MAX_VELOCITY: f32 = 1.0;

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player {},
        SmoothMovement::new(
            Vec3::default(),
            ACCELERATION,
            DESACELERATION,
            MAX_VELOCITY,
            Vec3::default(),
        ),
        Transform::default(),
        Name::new("player"),
    ));
}

pub fn plane_movement(
    mut player_query: Query<(&mut SmoothMovement, &Transform), With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    if let Ok((mut smooth_movement, transform)) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 0.0, 1.0)
        }
        if keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(-0.0, 0.0, -1.0)
        }
        if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(-1.0, 0.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(1.0, 0.0, 0.0)
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
            direction = transform.rotation.mul_vec3(direction);
        }

        smooth_movement.change_translation(direction * time.delta_seconds(), time.delta_seconds());
    }
}

pub fn sync_player_rotation(
    mut player_q: Query<&mut Transform, (With<Player>, Without<Camera>)>,
    cam_q: Query<&Transform, With<Camera>>,
) {
    let Ok(mut player_transform) = player_q.get_single_mut() else { return };
    let Ok(camera_transform) = cam_q.get_single() else { return };

    let mut target = camera_transform.translation;
    target.y = 0.0;
    player_transform.look_at(target, Vec3::Y);
}

pub fn proportional_smooth_values(
    mut player_q: Query<(&mut SmoothMovement, &Transform), (With<Player>, Without<Camera>)>,
    cam_q: Query<&Transform, With<Camera>>,
) {
    let Ok((mut smooth_movement, player_transform)) = player_q.get_single_mut() else { return };
    let Ok(camera_transform) = cam_q.get_single() else { return };

    let distance = (camera_transform.translation - player_transform.translation).length();

    smooth_movement.max_velocity = MAX_VELOCITY * distance;
    smooth_movement.acceleration = ACCELERATION * distance;
    smooth_movement.desacceleration = DESACELERATION * distance;
}
