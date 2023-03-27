pub mod components;
mod systems;


use crate::main_menu::components::GameState;
use bevy::prelude::*;
use systems::*;

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (move_projectile)
        ).in_set(OnUpdate(GameState::InGame));
    }
}
