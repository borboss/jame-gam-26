use bevy::prelude::*;

use super::components::GameOverMenuComponentMarker;
use crate::game::SimulationState;
use crate::game::player::components::HP;
use crate::main_menu::components::{MenuAnimationIndices, MenuAnimationTimer};

pub fn game_over_initiator(mut commands: Commands, hp: Res<HP>) {
    if hp.is_changed() {
        if hp.hp <= 0 {
            commands.insert_resource(NextState(Some(SimulationState::GameOver)));
        }
    }
}

pub fn restart_game_init() {

}

pub fn spawn_game_over(mut commands: Commands, asset_server: Res<AssetServer>) {
    // CHANGE TO GAME OVER SCREEN PNG
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(960.0 / 2.0, 540.0 / 2.0, 10.0f32)
                .with_scale(Vec3::new(3.0f32, 3.0f32, 1.0f32)),
            texture: asset_server.load("sprites/pausescreen.png"),

            ..default()
        },
        GameOverMenuComponentMarker,
    ));
}

pub fn spawn_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // CHANGE TO GAME OVER TEXT
    let texture_handle = asset_server.load("sprites/pausescreen-text.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(320.0, 180.0), 2, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_xyz(960.0 / 2.0, 540.0 / 2.0, 11.0f32)
                .with_scale(Vec3::new(3.0f32, 3.0f32, 1.0f32)),
            ..default()
        },
        MenuAnimationIndices {
            first: 0,
            last: 1,
            delete_on_end: false,
        },
        MenuAnimationTimer(Timer::from_seconds(1.0, TimerMode::Repeating)),
        GameOverMenuComponentMarker,
    ));
}