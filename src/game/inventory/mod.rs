use bevy::prelude::*;
use bevy_tweening::*;

pub mod components;
pub mod card_components;
pub mod card_systems;
mod systems;

use components::*;
use card_systems::*;
use systems::*;


pub struct InventoryPlugin;
impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Inventory>()
            .add_plugin(TweeningPlugin)
            .add_startup_system(init_inventory.before(init_render_cards))
            .add_startup_system(init_render_cards)
            .add_system(maintain_inventory)
            .add_system(card_handler)
            .add_system(play_card)
            .add_system(inventory_changed);
    }
}