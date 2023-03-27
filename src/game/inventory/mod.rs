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
            .add_systems(
                (
                    init_inventory.before(init_render_cards),
                    init_render_cards,
                ).in_set(OnEnter(GameState::InGame))
            )
            .add_systems(
                (
                maintain_inventory,
                card_handler,
                play_card,
                inventory_changed,
                ).in_set(OnUpdate(GameState::InGame))
            );
    }
}