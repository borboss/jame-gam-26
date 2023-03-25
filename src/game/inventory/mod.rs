use bevy::prelude::{App, Plugin};

pub mod components;
mod systems;

use components::*;
use systems::*;


pub struct InventoryPlugin;
impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app;
    }
}