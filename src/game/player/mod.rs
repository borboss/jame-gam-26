use bevy::prelude::{App, IntoSystemAppConfig, OnEnter, Plugin, Vec2};

pub mod components;
mod systems;

use super::super::SimulationState;
use crate::AppState;

use self::{
    components::{PlayerPosition, HP, MP},
    systems::*,
};

pub const PLAYER_SPEED: f32 = 400.0f32;
pub const PLAYER_SPRITE_SIZE: Vec2 = Vec2::new(24.0f32 * 5.0f32, 12.0f32 * 5.0f32);

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HP>()
            .init_resource::<MP>()
            .init_resource::<PlayerPosition>()
            .add_system(spawn_player.in_schedule(OnEnter(AppState::Game))) // player is stationary
            .add_system(despawn_player.in_schedule(OnEnter(SimulationState::Paused)));
                                                                               //.add_system(move_player.before(confine_player_movement))
                                                                               //.add_system(confine_player_movement);
    }
}
