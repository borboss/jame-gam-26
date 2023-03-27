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
            .add_systems((animate_enemy_sprite,
            spawn_enemies_time,
            tick_enemy_spawn_timer,
            swordsman_handler,
            archer_handler,
            cooldown_manager,
            animate_sprites,
            enemy_death_handler,).in_set(OnUpdate(GameState::InGame)));
    }
}
