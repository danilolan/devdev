use crate::{
    asset_manager::resources::AssetsLoaded,
    npc::{components::Npc, pathfinding::components::Pathfinding},
};
use bevy::{ecs::system::Command, prelude::*};

pub struct NpcSpawner {}

impl Command for NpcSpawner {
    fn apply(self, world: &mut World) {
        let assets = world.get_resource::<AssetsLoaded>();

        if let Some(assets) = assets {
            let asset = assets.get_asset_scene("scene/building/wall");

            let bundles = (
                SceneBundle {
                    scene: asset.clone(),
                    transform: Transform {
                        translation: Vec3::new(0.0, 0.0, 0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Npc::default(),
                Pathfinding::default(),
                Name::new("npc".to_string()),
            );

            world.spawn(bundles);
        }
    }
}
