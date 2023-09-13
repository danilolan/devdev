//file used to declare components, systems... to use in world physics

use bevy::{gizmos, prelude::*};
use bevy_rapier3d::prelude::Collider;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, show_colliders);
    }
}

//systems
const SHOW_COLLIDERS: bool = true;
fn show_colliders(
    mut gizmos: Gizmos,
    collider_query: Query<(&Transform, &BoxCollider), With<BoxCollider>>,
) {
    if !SHOW_COLLIDERS {
        return;
    }
    for (transform, collider) in collider_query.iter() {
        gizmos.cuboid(
            Transform {
                translation: transform.translation,
                rotation: transform.rotation,
                scale: collider.size,
            },
            Color::GREEN,
        )
    }
}
//collider
#[derive(Component)]
pub struct BoxCollider {
    size: Vec3,
}
