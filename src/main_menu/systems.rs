use bevy::{
    ecs::query::Or,
    prelude::{
        default, AssetServer, Assets, Commands, Entity, Input, KeyCode, NextState, Query, Res,
        ResMut, State, Transform, Vec2, Vec3, With,
    },
    sprite::{SpriteBundle, SpriteSheetBundle, TextureAtlas, TextureAtlasSprite},
    time::{Time, Timer, TimerMode},
};

use crate::{
    game::game_over::components::GameOverMenuComponentMarker, game::SimulationState, AppState,
};

use super::components::{MainMenuComponentMarker, MenuAnimationIndices, MenuAnimationTimer};

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(960.0 / 2.0, 540.0 / 2.0, 10.0f32)
                .with_scale(Vec3::new(3.0f32, 3.0f32, 1.0f32)),
            texture: asset_server.load("sprites/titlescreen.png"),

            ..default()
        },
        MainMenuComponentMarker,
    ));
}

pub fn spawn_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("sprites/PressAnyButton-text.png");
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
        MainMenuComponentMarker,
    ));
}

pub fn clean_up_main_menu(
    mut commands: Commands,
    menu_assets: Query<Entity, With<MainMenuComponentMarker>>,
) {
    for asset_entity in menu_assets.iter() {
        commands.entity(asset_entity).despawn();
    }
}

pub fn animate_menu_text(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &MenuAnimationIndices,
            &mut MenuAnimationTimer,
            &mut TextureAtlasSprite,
        ),
        Or<(
            With<MainMenuComponentMarker>,
            With<GameOverMenuComponentMarker>,
        )>,
    >,
) {
    for (entity, indices, mut timer, mut sprite) in query.iter_mut() {
        timer.tick(time.delta());
        println!("Anim");
        if timer.just_finished() {
            if sprite.index == indices.last {
                if indices.delete_on_end {
                    commands.entity(entity).despawn();
                } else {
                    sprite.index = indices.first;
                }
            } else {
                sprite.index = sprite.index + 1;
            };
        }
    }
}

pub fn begin_game_handler(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if keyboard_input.get_pressed().len() > 0 {
        if app_state.0 == AppState::MainMenu {
            commands.insert_resource(NextState(Some(AppState::Game)));
            commands.insert_resource(NextState(Some(SimulationState::Running)));
        }
    }
}
