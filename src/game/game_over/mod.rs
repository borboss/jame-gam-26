use bevy::prelude::*;

pub mod components;
mod systems;

use super::SimulationState;
use systems::*;

pub struct GameOverPlugin;
impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(game_over_initiator.in_set(OnUpdate(SimulationState::Running)))
            .add_systems(
                (spawn_game_over, spawn_text).in_schedule(OnEnter(SimulationState::GameOver)),
            );
    }
}
