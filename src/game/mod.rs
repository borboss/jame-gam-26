use bevy::prelude::*;

pub mod player;
mod inventory;
mod stat_bar;
pub mod attacks;
mod enemy;

use stat_bar::*;
use player::*;
use inventory::*;
use enemy::*;

use self::attacks::AttackPlugin;


pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(PlayerPlugin)
        .add_plugin(InventoryPlugin)
        .add_plugin(StatBarPlugin)
        .add_plugin(AttackPlugin)
        .add_plugin(EnemyPlugin);
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    Running,
    #[default] Paused,
}