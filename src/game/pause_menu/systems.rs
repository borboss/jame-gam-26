use bevy::{
    prelude::{
        default, AssetServer, Assets, Commands, Input, KeyCode, NextState, Res, ResMut, State,
        Transform, Vec2, Vec3, Entity, Query, With,
    },
    sprite::{SpriteBundle, SpriteSheetBundle, TextureAtlas, TextureAtlasSprite},
    time::{Timer, TimerMode},
};

use crate::{
    game::SimulationState,
    main_menu::components::{MenuAnimationIndices, MenuAnimationTimer},
};

use super::components::PauseMenuComponentMarker;

pub fn spawn_pause_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(960.0 / 2.0, 540.0 / 2.0, 10.0f32)
                .with_scale(Vec3::new(3.0f32, 3.0f32, 1.0f32)),
            texture: asset_server.load("sprites/pausescreen.png"),

            ..default()
        },
        PauseMenuComponentMarker,
    ));
}

pub fn spawn_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
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
        PauseMenuComponentMarker,
    ));
}

pub fn pause_handler(
    mut commands: Commands,
    pause_components: Query<Entity, With<PauseMenuComponentMarker>>,
    keyboard_input: Res<Input<KeyCode>>,
    sim_state: Res<State<SimulationState>>,
) {
    if keyboard_input.just_released(KeyCode::Escape) && sim_state.0 != SimulationState::Running {
        for pause_component_entity in pause_components.iter() {
            commands.entity(pause_component_entity).despawn();
        }
        commands.insert_resource(NextState(Some(SimulationState::Running)));
    } else if keyboard_input.just_released(KeyCode::Escape) && sim_state.0 != SimulationState::Paused {
        commands.insert_resource(NextState(Some(SimulationState::Paused)));
    }
}
