use bevy::prelude::*;

mod components;
mod systems;

use systems::*;
use crate::AppState;
use super::SimulationState;

pub struct GameOverPlugin;
impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
        .add_systems((game_over_initiator, spawn_text).in_set(OnUpdate(AppState::Game)).in_set(OnUpdate(SimulationState::Running)))
        .add_system(spawn_game_over.in_schedule(OnEnter(SimulationState::GameOver)))
        ;
    }
}