use bevy::prelude::*;

use crate::game::{player::components::{HP, MP}, attacks::components::SpawnedProjectile};

use super::components::*;

pub fn init_render_bar(mut commands: Commands) {
    // HP
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(232.0 / 255.0, 59.0 / 255.0, 59.0 / 255.0),
                custom_size: Some(Vec2::new(250.0, 12.5)),
                ..default()
            },
            transform: Transform::from_xyz(960.0 / 2.0, 540.0 - 25.0, 3.0),
            ..default()
        },
        StatusBar {},
        HpMarker {},
    )); // < Actual bar to be edited & renamed
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(17.0 / 255.0, 17.0 / 255.0, 17.0 / 255.0),
            custom_size: Some(Vec2::new(250.0, 12.5)),
            ..default()
        },
        transform: Transform::from_xyz(960.0 / 2.0, 540.0 - 25.0, 2.0),
        ..default()
    },)); // < Base to see it better

    // MP
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(59.0 / 255.0, 94.0 / 255.0, 232.0 / 255.0),
                custom_size: Some(Vec2::new(250.0, 12.5)),
                ..default()
            },
            transform: Transform::from_xyz(960.0 / 2.0, 540.0 - 50.0, 3.0),
            ..default()
        },
        StatusBar {},
        MpMarker {},
    )); // < Actual bar to be edited & renamed
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(17.0 / 255.0, 17.0 / 255.0, 17.0 / 255.0),
            custom_size: Some(Vec2::new(250.0, 12.5)),
            ..default()
        },
        transform: Transform::from_xyz(960.0 / 2.0, 540.0 - 50.0, 2.0),
        ..default()
    },)); // < Base to see it better
}

pub fn update_bars(
    mut hp_bar: Query<&mut Sprite, (With<HpMarker>, Without<MpMarker>)>,
    hp: Res<HP>,
    mut mp_bar: Query<&mut Sprite, (With<MpMarker>, Without<HpMarker>)>,
    mp: Res<MP>,
) {
    if hp.is_changed() {
        let mut _bar_sprite = hp_bar.get_single_mut().unwrap();
        _bar_sprite.custom_size = Some(Vec2::new(
            250.0 * (hp.hp as f32 / hp.max_hp as f32),
            12.5f32,
        ));
    }
    if mp.is_changed() {
        let mut _bar_sprite = mp_bar.get_single_mut().unwrap();
        _bar_sprite.custom_size = Some(Vec2::new(
            250.0 * (mp.mp as f32 / mp.max_mp as f32),
            12.5f32,
        ));
    }
}

