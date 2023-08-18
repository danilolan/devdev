use bevy::prelude::*;

mod tiles;
use tiles::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        //app.add_plugins(TilesPlugin);
    }
}