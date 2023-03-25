use bevy::prelude::*;

pub mod components;
pub mod card_components;
pub mod card_systems;
mod systems;

use bevy_tweening::TweeningPlugin;
use components::*;
use card_systems::*;
use systems::*;


pub struct InventoryPlugin;
impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(TweeningPlugin)
            .init_resource::<Inventory>()
            .add_startup_system(init_inventory.before(init_render_cards))
            .add_startup_system(init_render_cards)
            .add_system(maintain_inventory.before(card_handler))
            .add_system(card_handler.after(maintain_inventory))
            .add_system(inventory_changed.after(card_handler));
    }
}