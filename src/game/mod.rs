use bevy::prelude::*;

pub mod attacks;
mod enemy;
mod inventory;
mod pause_menu;
pub mod player;
mod stat_bar;
mod game_over;

use attacks::AttackPlugin;
use enemy::EnemyPlugin;
use inventory::InventoryPlugin;
use pause_menu::PauseMenuPlugin;
use player::PlayerPlugin;
use stat_bar::StatBarPlugin;
use game_over::GameOverPlugin;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
            .add_plugin(PlayerPlugin)
            .add_plugin(InventoryPlugin)
            .add_plugin(StatBarPlugin)
            .add_plugin(AttackPlugin)
            .add_plugin(EnemyPlugin)
            .add_plugin(PauseMenuPlugin)
            .add_plugin(GameOverPlugin);
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
    GameOver,
}
