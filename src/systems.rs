use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(960.0 / 2.0, 540.0 / 2.0, 12.0f32),
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::AutoMin {
                min_height: 540.0f32,
                min_width: 960.0f32,
            },
            ..default()
        },
        ..default()
    });
}

pub fn spawn_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((SpriteBundle {
        transform: Transform::from_xyz(960.0 / 2.0, 540.0 / 2.0, 0.0f32)
            .with_scale(Vec3::new(3.0f32, 3.0f32, 0.0f32)),
        texture: asset_server.load("sprites/background.png"),

        ..default()
    },));
}

pub fn game_over_handler(mut game_over_event_reader: EventReader<crate::events::GameOver>) {
    for event in game_over_event_reader.iter() {
        println!("Final score was {}", event.score.to_string())
    }
}
