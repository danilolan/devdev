use bevy::prelude::*;
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        //systems
        app.add_systems(Update, handle_lerp_movement);
    }
}

//----components----
#[derive(Component)]
pub struct LerpMovement {
    pub target_position: Vec3,
    pub current_position: Vec3,
    pub speed: f32,
}

impl LerpMovement {
    pub fn set_target(&mut self, target: Vec3) {
        self.target_position = target;
    }
}

//----systems----
fn handle_lerp_movement(time: Res<Time>, mut query: Query<(&mut LerpMovement, &mut Transform)>) {
    for (mut lerp_movement, mut transform) in query.iter_mut() {
        let t = lerp_movement.speed * time.delta_seconds();
        lerp_movement.current_position = lerp_movement
            .current_position
            .lerp(lerp_movement.target_position, t);
        transform.translation = lerp_movement.current_position;
    }
}
