use bevy::prelude::{App, Plugin};

pub mod components;
mod systems;

pub struct CardPlugin;
impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
    }
}