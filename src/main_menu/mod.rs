use bevy::prelude::{
    App, IntoSystemAppConfig, IntoSystemAppConfigs, IntoSystemConfig, OnEnter, OnUpdate, Plugin,
};

pub mod components;
mod systems;

use systems::*;

use crate::AppState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((spawn_main_menu, spawn_text).in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(clean_up_main_menu.in_schedule(OnEnter(AppState::Game)))
            .add_system(begin_game_handler.in_set(OnUpdate(AppState::MainMenu)))
            .add_system(animate_menu_text);
    }
}
