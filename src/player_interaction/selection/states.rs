use bevy::prelude::*;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash)]
pub enum CanPlaceState {
    True,
    False,
}

impl Default for CanPlaceState {
    fn default() -> Self {
        Self::True
    }
}
