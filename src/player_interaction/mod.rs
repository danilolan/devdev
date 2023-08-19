use bevy::prelude::*;

pub mod camera;
use camera::CameraPlugin;
pub mod picking;
use picking::PickingPlugin;
pub mod selection;
use selection::SelectionPlugin;

pub struct PlayerInteractionPlugin;

impl Plugin for PlayerInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, plane_movement);
        app.add_systems(Update, sync_player_rotation);

        //plugins
        app.add_plugins(CameraPlugin);
        app.add_plugins(PickingPlugin);
        app.add_plugins(SelectionPlugin);
    }
}

//Components
#[derive(Component)]
pub struct Player {}


//Systems
const SPEED: f32 = 10.0;

pub fn spawn_player(
    mut commands: Commands,
){
    commands.spawn((
        Player {},
        Transform {
            ..Default::default()
        },
        Name::new("player")
    ));
}

pub fn plane_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>
){
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 0.0, 1.0)
        }
        if keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(-0.0, 0.0, -1.0)
        }
        if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(-1.0,0.0 ,0.0)
        }
        if keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(1.0, 0.0, 0.0)
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
            direction = transform.rotation.mul_vec3(direction);
        }

        transform.translation +=  direction * SPEED * time.delta_seconds();
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