mod components;
mod systems;

use bevy::prelude::*;
use systems::*;

use crate::AppState;

use super::SimulationState;

pub struct StatBarPlugin;

impl Plugin for StatBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(init_render_bar.in_schedule(OnEnter(AppState::Game)))
            .add_system(
                update_bars
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            );
    }
}
