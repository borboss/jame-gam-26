use bevy::prelude::*;

pub mod components;
mod systems;

use components::*;
use systems::*;

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            .add_system(animate_sprite)
            .add_system(spawn_enemies_time.before(tick_enemy_spawn_timer))
            .add_system(tick_enemy_spawn_timer)
            .add_system(move_enemy);
    }
}
