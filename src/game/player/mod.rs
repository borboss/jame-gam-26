use bevy::prelude::{App, Plugin, IntoSystemConfig, Vec2};

pub mod components;
mod systems;

use self::systems::*;

pub const PLAYER_SPEED: f32 = 400.0f32;
pub const PLAYER_SPRITE_SIZE: Vec2 = Vec2::new(24.0f32 * 5.0f32, 12.0f32 * 5.0f32);

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(spawn_player)
        .add_system(move_player.before(confine_player_movement))
        .add_system(confine_player_movement);
    }
}