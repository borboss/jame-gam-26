use bevy::prelude::*;

use super::components::{Player, HP, MP};
use super::*;

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    mut player_position: ResMut<PlayerPosition>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        player_position.position = player_transform.translation;
        let mut direction: Vec3 = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        player_transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut hp: ResMut<HP>,
    mut mp: ResMut<MP>,
    mut player_position: ResMut<PlayerPosition>,
) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/player.png"),
            transform: Transform::from_xyz(480.0, 265.0, 1.0f32)
                .with_scale(Vec3::new(5.0f32, 5.0f32, 1.0f32)),

            ..default()
        },
        Player { ..default() },
    ));

    // init resources
    hp.max_hp = 35;
    hp.hp = 35;
    mp.max_mp = 50;
    mp.mp = 50;

    player_position.position = Vec3::new(480.0, 265.0, 1.0f32);
}

// 41, 16 top corner
pub fn confine_player_movement(mut player_query: Query<&mut Transform, With<Player>>) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let half_player_size: Vec2 = 0.5f32 * PLAYER_SPRITE_SIZE;

        let x_min: f32 = 0.0 + half_player_size.x;
        let x_max: f32 = 960.0 - half_player_size.x;
        let y_min: f32 = (10.0f32 * 5.0f32) + half_player_size.y;
        let y_max: f32 = 540.0 - 100.0 - half_player_size.y;

        let mut translation: Vec3 = player_transform.translation;

        // Bound x position
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        // Bound y position
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }
}

pub fn despawn_player(mut commands: Commands, query: Query<Entity, With<Player>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
