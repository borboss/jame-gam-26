use bevy::prelude::*;

pub mod player;
mod inventory;
mod stat_bar;
pub mod attacks;

use stat_bar::*;
use player::*;
use inventory::*;


pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(PlayerPlugin)
        .add_plugin(InventoryPlugin)
        .add_plugin(StatBarPlugin);
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    Running,
    #[default] Paused,
}