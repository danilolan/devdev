use bevy::prelude::*;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash)]
pub enum BuildingState {
    Wall,
    Window,
    Pillar,
    Door,
    Destroy,
    None,
}

impl Default for BuildingState {
    fn default() -> Self {
        Self::None
    }
}
