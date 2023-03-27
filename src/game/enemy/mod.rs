use bevy::prelude::*;

pub mod components;
mod systems;


use crate::main_menu::components::GameState;
use components::*;
use systems::*;

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            .add_system(animate_enemy_sprite.run_if(in_state(GameState::InGame)))
            .add_system(spawn_enemies_time.before(tick_enemy_spawn_timer).run_if(in_state(GameState::InGame)))
            .add_system(tick_enemy_spawn_timer.run_if(in_state(GameState::InGame)))
            .add_system(swordsman_handler.run_if(in_state(GameState::InGame)))
            .add_system(archer_handler.run_if(in_state(GameState::InGame)))
            .add_system(cooldown_manager.run_if(in_state(GameState::InGame)))
            .add_system(animate_sprites.run_if(in_state(GameState::InGame)))
            .add_system(enemy_death_handler.run_if(in_state(GameState::InGame)));
    }
}
