pub mod components;
mod systems;

use components::*;
use systems::*;

pub struct EnemyPlugin {}
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app;
    }
}