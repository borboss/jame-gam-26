use bevy::prelude::{App, Plugin, IntoSystemConfig};

pub mod components;
mod systems;

use systems::*;


pub struct InventoryPlugin;
impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(init_inventory)
        .add_system(maintain_inventory)
        .add_system(print_inventory);
    }
}