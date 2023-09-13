//file used to declare components, systems... to use in world physics

use bevy::{gizmos, prelude::*};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, show_colliders);
        app.add_systems(Update, handle_colliders);
    }
}

//systems
fn handle_colliders(mut collider_query: Query<(&mut BoxCollider, &Transform), With<BoxCollider>>) {
    for (mut collider, transform) in collider_query.iter_mut() {
        let translation = transform.translation;
        let rotation = transform.rotation;

        collider.translation = Vec3::new(
            translation.x,
            translation.y + (collider.scale.y / 2.0),
            translation.z,
        );
        collider.rotation = rotation;
    }
}
const SHOW_COLLIDERS: bool = true;
fn show_colliders(mut gizmos: Gizmos, collider_query: Query<&BoxCollider, With<BoxCollider>>) {
    if !SHOW_COLLIDERS {
        return;
    }
    for collider in collider_query.iter() {
        gizmos.cuboid(
            Transform {
                translation: collider.translation,
                rotation: collider.rotation,
                scale: collider.scale,
            },
            Color::GREEN,
        )
    }
}
//collider
#[derive(Component)]
pub struct BoxCollider {
    pub scale: Vec3,
    pub translation: Vec3,
    pub rotation: Quat,
}
