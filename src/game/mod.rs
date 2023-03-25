use bevy::prelude::*;

pub mod player;
mod inventory;

use player::*;
use inventory::*;


pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(PlayerPlugin)
        .add_plugin(InventoryPlugin);
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    Running,
    #[default] Paused,
}