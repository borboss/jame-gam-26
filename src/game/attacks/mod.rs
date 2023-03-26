pub mod components;
mod systems;

use bevy::prelude::*;
use systems::*;

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app;
    }
}
