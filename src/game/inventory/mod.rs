use bevy::prelude::*;
use bevy_tweening::*;

pub mod card_components;
pub mod card_systems;
pub mod components;
mod systems;

use card_systems::*;
use components::*;
use systems::*;

use crate::AppState;

use super::SimulationState;

pub struct InventoryPlugin;
impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Inventory>()
            .add_plugin(TweeningPlugin)
            .add_systems(
                (init_inventory.before(init_render_cards), init_render_cards)
                    .in_schedule(OnEnter(AppState::Game)),
            )
            .add_systems(
                (
                    maintain_inventory,
                    card_handler,
                    play_card,
                    inventory_changed,
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_system(despawn_cards.in_schedule(OnEnter(SimulationState::GameOver)));
    }
}
