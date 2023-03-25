use bevy::prelude::Component;
use bevy::prelude::States;

#[derive(Component, Default)]
pub struct Player {
    pub state: PlayerState,
    pub direction: PlayerDirection,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum PlayerState {
    MOVE,
    #[default] IDLE,
    ATTACK,
    DEAD
}
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum PlayerDirection {
    UP,
    LEFT,
    #[default] DOWN,
    RIGHT,
}