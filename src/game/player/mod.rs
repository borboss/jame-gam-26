use bevy::prelude::{App, Plugin};

pub mod components;
mod systems;

use self::systems::*;

pub const PLAYER_SPEED: f32 = 400.0f32;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(spawn_player)
        .add_system(move_player);
    }
}