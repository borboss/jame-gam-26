use bevy::prelude::{App, Plugin, SystemSet};

mod systems;
pub mod components;

use components::*;
use systems::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {    
        app
        .add_state::<GameState>()
        ;
    }
}