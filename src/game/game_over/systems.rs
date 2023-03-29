use bevy::prelude::*;

use super::components::GameOverMenuComponentMarker;
use crate::game::player::components::HP;
use crate::game::SimulationState;
use crate::main_menu::components::{MenuAnimationIndices, MenuAnimationTimer};
use crate::AppState;

pub fn game_over_initiator(
    mut commands: Commands,
    hp: Res<HP>,
    app_state: Res<State<AppState>>,
    sim_state: Res<State<SimulationState>>,
) {
    if hp.is_changed() && app_state.0 == AppState::Game && sim_state.0 == SimulationState::Running {
        if hp.hp <= 0 {
            commands.insert_resource(NextState(Some(SimulationState::GameOver)));
        }
    }
}

pub fn restart_game_init(mut commands: Commands, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Return) || input.just_pressed(KeyCode::NumpadEnter) {
        println!("RESTART");
        commands.insert_resource(NextState(Some(SimulationState::Running)));
        commands.insert_resource(NextState(Some(AppState::MainMenu)));
    }
}

pub fn spawn_game_over(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(480.0, 265.0, 1.0f32)
                .with_scale(Vec3::new(5.0f32, 5.0f32, 1.0f32)),
            texture: asset_server.load("sprites/empty_throne.png"),

            ..default()
        },
        GameOverMenuComponentMarker,
    ));

    let texture_handle = asset_server.load("sprites/gameoverscreen.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(320.0, 180.0), 5, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_xyz(960.0 / 2.0, 540.0 / 2.0, 10.0f32)
                .with_scale(Vec3::new(3.0f32, 3.0f32, 1.0f32)),
            ..default()
        },
        MenuAnimationIndices {
            first: 0,
            last: 4,
            delete_on_end: false,
            stop_on_end: true,
        },
        MenuAnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        GameOverMenuComponentMarker,
    ));
}

pub fn spawn_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("sprites/gameoverscreen-text.png");
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
            stop_on_end: false,
        },
        MenuAnimationTimer(Timer::from_seconds(1.0, TimerMode::Repeating)),
        GameOverMenuComponentMarker,
    ));
}

pub fn despawn_pause_menu(mut commands: Commands, gameover_menu_components: Query<Entity, With<GameOverMenuComponentMarker>>) {
    for entity in gameover_menu_components.iter() {
        commands.entity(entity).despawn();
    }
}