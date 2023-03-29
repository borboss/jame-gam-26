use bevy::prelude::*;

mod components;
mod systems;
use crate::AppState;

use super::SimulationState;
use systems::*;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(pause_handler.in_set(OnUpdate(AppState::Game))).add_systems(
            (spawn_pause_screen, spawn_text).in_schedule(OnEnter(SimulationState::Paused)),
        );
    }
}
