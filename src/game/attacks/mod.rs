pub mod components;
mod systems;

use bevy::prelude::*;
use systems::*;

use crate::AppState;

use super::SimulationState;

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        // Add Despawn
        app.add_system(
            move_projectile
                .in_set(OnUpdate(AppState::Game))
                .in_set(OnUpdate(SimulationState::Running)),
        )
        .add_system(despawn_projectiles.in_schedule(OnEnter(SimulationState::GameOver)));
    }
}
