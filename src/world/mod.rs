use bevy::prelude::*;
pub struct WorldPlugin;

//plugins
pub mod object;
use object::*;

pub mod grid;
use grid::*;

pub mod walls;
use walls::*;

pub mod physics;
use physics::*;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        //systems
        app.add_systems(Update, handle_lerp_movement);
        //plugins
        app.add_plugins(ObjectPlugin);
        app.add_plugins(GridPlugin);
        app.add_plugins(WallsPlugin);
        app.add_plugins(PhysicsPlugin);
    }
}

//----components----
#[derive(Component)]
pub struct LerpMovement {
    pub target_position: Vec3,
    pub current_position: Vec3,
    pub speed: f32,
    pub max_speed: f32,
    pub acceleration: f32,
}

impl LerpMovement {
    pub fn set_target(&mut self, target: Vec3) {
        self.target_position = target;
    }
}

//----systems----
fn handle_lerp_movement(time: Res<Time>, mut query: Query<(&mut LerpMovement, &mut Transform)>) {
    for (mut lerp_movement, mut transform) in query.iter_mut() {
        let direction =
            (lerp_movement.target_position - lerp_movement.current_position).normalize();
        let distance_to_target = lerp_movement
            .current_position
            .distance(lerp_movement.target_position);

        lerp_movement.speed += lerp_movement.acceleration * time.delta_seconds();

        if lerp_movement.speed > lerp_movement.max_speed {
            lerp_movement.speed = lerp_movement.max_speed;
        }

        let move_distance = lerp_movement.speed * time.delta_seconds();

        // Se a distância a mover for maior que a distância até o alvo, definimos a distância a mover para a distância até o alvo
        let move_distance = move_distance.min(distance_to_target);

        lerp_movement.current_position += direction * move_distance;
        transform.translation = lerp_movement.current_position;
    }
}
