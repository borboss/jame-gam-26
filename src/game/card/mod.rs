use bevy::prelude::{App, Plugin};

use self::systems::init_cards;

pub mod components;
mod systems;

pub struct CardPlugin;
impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_cards);
    }
}