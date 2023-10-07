use bevy::prelude::*;

#[derive(PartialEq)]
pub enum BehaviorState {
    Walking,
    Waiting,
    Working,
    Idle,
}
