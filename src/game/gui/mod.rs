use bevy::{prelude::*, window::PrimaryWindow};

use super::player::components::*;
pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_render_bar)
            .add_system(update_health_bar.after(init_render_bar));
    }
}

fn init_render_bar(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window: &Window = window_query.get_single().unwrap();
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(232.0/255.0, 59.0/255.0, 59.0/255.0),
                custom_size: Some(Vec2::new(250.0, 12.5)),
                ..default()
            },
            transform: Transform::from_xyz(window.width() / 2.0, window.height() - 25.0, 9.9f32),
            ..default()
        },
        StatusBar {},
        HpMarker {},
    ));
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(70.0/255.0, 90.0/255.0, 1.0),
                custom_size: Some(Vec2::new(250.0, 12.5)),
                ..default()
            },
            transform: Transform::from_xyz(window.width() / 2.0, window.height() - 25.0, 9.9f32),
            ..default()
        },
        StatusBar {},
        MpMarker {},
    ));
}

fn update_health_bar(
    mut hp_bar: Query<&mut Sprite, With<HpMarker>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    player_query: Query<&HP, With<Player>>,
) {
    let mut _health_bar_bundle = hp_bar.get_single_mut().unwrap();
    let player_hp = player_query.get_single().unwrap();
    let window: &Window = window_query.get_single().unwrap();
    _health_bar_bundle.custom_size = Some(Vec2::new(
        250.0 * (player_hp.hp as f32 / player_hp.max_hp as f32),
        12.5f32,
    ));
}

#[derive(Component)]
pub struct StatusBar {}

#[derive(Component)]
pub struct HpMarker {}

#[derive(Component)]
pub struct MpMarker {}
