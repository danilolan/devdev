//! Holds all the spawner logic.
//!
//! Evertime you will need spawn some entity use the structs hold in this module.
//!
//! ## Example
//! If you need to spawn a new Npc
//!
//! ```
//! commands.add(NpcSpawner {});
//! ```
//!
//! ## Creation
//! If you need to create a new spawner just create a struct to the spawner and impl the Command trait.
//!
//! ```
//! pub struct NpcSpawner {}
//!
//!impl Command for NpcSpawner {
//!    fn apply(self, world: &mut World) {
//!        let assets = world.get_resource::<AssetsLoaded>();
//!
//!        if let Some(assets) = assets {
//!            let asset = assets.get_asset_scene("models/building/wall");
//!
//!            let bundles = (
//!                SceneBundle {
//!                    scene: asset.clone(),
//!                    transform: Transform {
//!                        translation: Vec3::new(0.0, 0.0, 0.0),
//!                        ..Default::default()
//!                    },
//!                    ..Default::default()
//!                },
//!                Npc::default(),
//!                Pathfinding::default(),
//!                Name::new("npc".to_string()),
//!            );
//!
//!            world.spawn(bundles);
//!        }
//!    }
//!}
//! ```

pub mod npc;
