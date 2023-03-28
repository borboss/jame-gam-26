use bevy::prelude::*;

mod components;
mod systems;
use systems::*;
use super::SimulationState;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(pause_handler)
        .add_systems((spawn_pause_screen, spawn_text).in_schedule(OnEnter(SimulationState::Paused)));
    }
}
