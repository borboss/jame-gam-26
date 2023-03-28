use bevy::prelude::*;

pub mod components;
mod systems;

use components::*;
use systems::*;

use crate::AppState;

use super::SimulationState;

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<EnemySpawnTimer>()
            .init_resource::<TotalEnemySpawns>()
            .add_systems(
            (
                animate_enemy_sprite,
                spawn_enemies_time,
                tick_enemy_spawn_timer,
                swordsman_handler,
                archer_handler,
                cooldown_manager,
                animate_sprites,
                enemy_death_handler,
            )
                .in_set(OnUpdate(AppState::Game))
                .in_set(OnUpdate(SimulationState::Running)),
        );
    }
}
