use bevy::prelude::*;
use rand::random;

use crate::game::player::components::PlayerPosition;

use super::components::*;

pub fn spawn_enemy(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    position: &Vec2,
) {
    let texture_handle = asset_server.load("sprites/enemies-Sheet.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 3, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(1),
            transform: Transform::from_translation(Vec3::new(position.x, position.y, 9.0))
                .with_scale(Vec3::new(2.5, 2.5, 2.5)),
            ..default()
        },
        AnimationIndices { first: 1, last: 2 },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Enemy {
            direction: Vec3::ZERO,
            health: 25,
            max_health: 25,
            speed: 50.0,
            enemy_type: EnemyType::Swordsman,
        },
    ));
}

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}

pub fn tick_enemy_spawn_timer(mut enemy_spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    enemy_spawn_timer.timer.tick(time.delta());
}

pub const POSITIONS: [Vec2; 2] = [Vec2::new(930.0, 230.0), Vec2::new(30.0, 230.0)];

pub fn spawn_enemies_time(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    spawn_timer: Res<EnemySpawnTimer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    if spawn_timer.timer.finished() {
        let random_side: Vec2;
        let weight = random::<f32>();
        random_side = POSITIONS[weight.round() as usize];

        let random_position: Vec2 = Vec2::new(
            random::<f32>() * 20.0 + random_side.x,
            random::<f32>() * 20.0 + random_side.y,
        );

        spawn_enemy(
            &mut commands,
            &asset_server,
            &mut texture_atlases,
            &random_position,
        );
    }
}

pub fn move_enemy(
    mut query: Query<(&Enemy, &mut Transform), With<Enemy>>,
    time: Res<Time>,
    player_position: Res<PlayerPosition>,
) {
    for (enemy, mut transform) in query.iter_mut() {
        let direction: Vec3 = Vec3::new(
            player_position.position.x - transform.translation.x,
            player_position.position.y - transform.translation.y,
            0.0f32,
        )
        .normalize();

        transform.translation += direction * enemy.speed * time.delta_seconds();
    }
}
