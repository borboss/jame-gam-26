use bevy::prelude::Component;

#[derive(Component)]
pub struct Player {

}

enum PlayerState {
    MOVE,
    ATTACK,
    DEAD
}
enum PlayerDirection {
    UP,
    LEFT,
    DOWN,
    RIGHT,
}