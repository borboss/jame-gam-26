use bevy::prelude::*;

pub mod components;
mod systems;

use components::*;
use systems::*;

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            .add_system(animate_enemy_sprite)
            .add_system(spawn_enemies_time.before(tick_enemy_spawn_timer))
            .add_system(tick_enemy_spawn_timer)
            .add_system(swordsman_handler)
            .add_system(archer_handler)
            .add_system(cooldown_manager)
            .add_system(animate_sprites)
            .add_system(enemy_death_handler);
    }
}
