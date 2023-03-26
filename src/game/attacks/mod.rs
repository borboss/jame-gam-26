pub mod components;
mod systems;

use bevy::prelude::*;
use systems::*;

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(move_projectile)
            .add_system(fader);
    }
}
