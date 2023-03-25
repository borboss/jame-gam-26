use bevy::prelude::{App, Plugin};

use self::systems::*;

pub mod components;
mod systems;

pub struct CardPlugin;
impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(init_render_cards)
            .add_system(debug_cards);
    }
}