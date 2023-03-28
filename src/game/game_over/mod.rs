use bevy::prelude::*;

mod components;
mod systems;

use systems::*;
use super::SimulationState;

pub struct GameOverPlugin;
impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
        .add_system(game_over_initiator.in_set(OnUpdate(SimulationState::Running)))
        .add_systems((spawn_game_over, spawn_text).in_schedule(OnEnter(SimulationState::GameOver)))
        ;
    }
}