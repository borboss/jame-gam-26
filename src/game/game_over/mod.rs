use bevy::prelude::*;


use crate::AppState;

use self::systems::game_over_handler;

use super::SimulationState;

mod systems;


pub struct GameOverPlugin;
impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
        .add_system(game_over_handler.in_set(OnUpdate(AppState::Game)).in_set(OnUpdate(SimulationState::Running)))
        ;
    }
}