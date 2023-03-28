use std::f32::consts::E;

use bevy::prelude::*;
use rand::random;

use crate::game::{
    attacks::components::{DamageEnemy, SpawnedProjectile},
    player::components::{PlayerPosition, HP},
};

use super::components::*;

pub fn spawn_enemy(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    position: &Vec2,
) {
    let weighting = random::<f32>();
    if weighting > 0.5 {
        let texture_handle = asset_server.load("sprites/enemies-Sheet.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 3, 2, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(1),
                transform: Transform::from_translation(Vec3::new(position.x, position.y, 8.0))
                    .with_scale(Vec3::new(2.5, 2.5, 2.5)),
                ..default()
            },
            AnimationIndices {
                first: 0,
                last: 2,
                delete_on_end: false,
            },
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            Enemy {
                direction: Vec3::ZERO,
                health: (random::<f32>() * 5.0).round() as i32,
                max_health: 5,
                speed: 50.0,
                enemy_type: EnemyType::Swordsman,
                state: EnemyState::Moving,
                attack_active: false,
                damage: 1,
            },
            SwordsmanMarker,
        ));
    } else {
        let texture_handle = asset_server.load("sprites/archer-Sheet.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 3, 2, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(1),
                transform: Transform::from_translation(Vec3::new(position.x, position.y, 8.0))
                    .with_scale(Vec3::new(2.5, 2.5, 2.5)),
                ..default()
            },
            AnimationIndices {
                first: 0,
                last: 2,
                delete_on_end: false,
            },
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            Enemy {
                direction: Vec3::ZERO,
                health: (random::<f32>() * 2.0).round() as i32,
                max_health: 2,
                speed: 50.0,
                enemy_type: EnemyType::Archer,
                state: EnemyState::Moving,
                attack_active: false,
                damage: 1,
            },
            ArcherMarker,
        ));
    }
}

pub fn animate_enemy_sprite(
    time: Res<Time>,
    mut query: Query<
        (
            &mut AnimationIndices,
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
            &Enemy,
        ),
        With<Enemy>,
    >,
) {
    for (mut indices, mut timer, mut sprite, enemy) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            match enemy.state {
                EnemyState::Attacking => {
                    indices.first = 3;
                    indices.last = 5;
                }
                EnemyState::Moving => {
                    indices.first = 0;
                    indices.last = 2;
                }
                EnemyState::AttackPossible => {
                    indices.first = 3;
                    indices.last = 5;
                }
            }
            sprite.index = if sprite.index == 5 || sprite.index == 2 {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}

pub fn tick_enemy_spawn_timer(
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
    time: Res<Time>,
) {
    enemy_spawn_timer.timer.tick(time.delta());
}

pub const POSITIONS: [Vec2; 2] = [Vec2::new(930.0, 230.0), Vec2::new(30.0, 230.0)];

pub fn spawn_enemies_time(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    spawn_timer: Res<EnemySpawnTimer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut total_enemies: ResMut<TotalEnemySpawns>,
) {
    if spawn_timer.timer.finished() {
        for _ in 0..=(E
            .powf(total_enemies.total_spawns as f32 / 20.0)
            .clamp(1.0, 20.0) as i32)
        {
            let random_side: Vec2;
            let weight = random::<f32>();
            random_side = POSITIONS[weight.round() as usize];

            let random_position: Vec2 = Vec2::new(
                random::<f32>() * 20.0 + random_side.x,
                random::<f32>() * 30.0 + random_side.y,
            );

            spawn_enemy(
                &mut commands,
                &asset_server,
                &mut texture_atlases,
                &random_position,
            );
        }
        total_enemies.total_spawns += 1;
    }
}

/*  To anyone reading this: I apologize for this shitty handler code.
    It's probably horribly ineffecient, and honestly, I dont really understand it.
    That's why I'm not gonna bother abstracting the positioning code, and just copy and paste the same function for my archer.
*/

pub fn swordsman_handler(
    mut commands: Commands,
    mut query: Query<
        (Entity, &mut Enemy, &mut Transform, &mut TextureAtlasSprite),
        With<SwordsmanMarker>,
    >,
    player_position: Res<PlayerPosition>,
    time: Res<Time>,
    mut hp: ResMut<HP>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let num_enemies = query.iter().count();
    let radius = 50.0; // adjust this to change the radius around the player

    if num_enemies > 0 {
        let angle_step = 2.0 * std::f32::consts::PI / (num_enemies as f32);
        let mut angle: f32 = 0.0;

        for (entity, mut enemy, mut transform, mut sprite) in query.iter_mut() {
            /*
            2.0 Attacks (Upper Half of Screen)
            1.4 Enemies (Upper Half of Screen)
            1.0 is Player
            0.95 is Attacks (Lower Half of Screen)
            0.9 is Enemies (Lower Half of Screen)
            */

            let speed = enemy.speed; // adjust this to change the enemy speed
            let target_position = player_position.position
                + Vec3::new(radius * angle.cos(), radius * angle.sin(), 0.0);
            let direction = (target_position - transform.translation).normalize();

            let distance_to_target = (target_position - transform.translation).length();

            if distance_to_target < 10.0 {
                transform.translation = Vec3::new(target_position.x, target_position.y, transform.translation.z);
                // if attack possible -> attack, spawn attack, attack active, damage player. Once attack not active, then attack not active
                // if attacking -> check if active
                match enemy.state {
                    EnemyState::AttackPossible => {
                        hp.hp -= enemy.damage;
                        commands
                            .entity(entity)
                            .insert(CooldownTimer(Timer::from_seconds(7.5, TimerMode::Once)));

                        let midpoint = (player_position.position + transform.translation) / 2.0;
                        let direction = (player_position.position - midpoint).normalize();
                        let angle: f32 = direction.y.atan2(direction.x);

                        let texture_handle = asset_server.load("sprites/effects/swing.png");
                        let texture_atlas = TextureAtlas::from_grid(
                            texture_handle,
                            Vec2::new(39.0, 16.0),
                            5,
                            1,
                            None,
                            None,
                        );
                        let texture_atlas_handle = texture_atlases.add(texture_atlas);
                        commands.spawn((
                            SpriteSheetBundle {
                                texture_atlas: texture_atlas_handle,
                                sprite: TextureAtlasSprite::new(0),
                                transform: Transform::from_translation(Vec3::new(
                                    midpoint.x, midpoint.y, 1.75,
                                ))
                                .with_scale(Vec3::splat(2.5))
                                .with_rotation(Quat::from_rotation_z(angle + 90.0)),
                                ..default()
                            },
                            AnimationIndices {
                                first: 0,
                                last: 4,
                                delete_on_end: true,
                            },
                            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
                        ));
                        enemy.state = EnemyState::Attacking;
                    }
                    EnemyState::Attacking => {}
                    EnemyState::Moving => {
                        enemy.state = EnemyState::AttackPossible;
                    }
                }
            } else {
                enemy.state = EnemyState::Moving;
                transform.translation += direction * speed * time.delta_seconds();
            }
            if transform.translation.x < player_position.position.x {
                sprite.flip_x = true; // flip the sprite horizontally
            } else {
                sprite.flip_x = false; // don't flip the sprite
            }

            angle += angle_step;
            if transform.translation.y < 540.0 / 2.0 {
                transform.translation.z = 1.4;
            } else {
                transform.translation.z = 0.9;
            }
        }
    }
}

pub fn archer_handler(
    mut commands: Commands,
    mut query: Query<
        (Entity, &mut Enemy, &mut Transform, &mut TextureAtlasSprite),
        With<ArcherMarker>,
    >,
    player_position: Res<PlayerPosition>,
    time: Res<Time>,
    mut hp: ResMut<HP>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let num_enemies = query.iter().count();
    let radius = 125.0; // adjust this to change the radius around the player

    if num_enemies > 0 {
        let angle_step = 2.0 * std::f32::consts::PI / (num_enemies as f32);
        let mut angle: f32 = 0.0;

        for (entity, mut enemy, mut transform, mut sprite) in query.iter_mut() {
            /*
            2.0 Attacks (Upper Half of Screen)
            1.4 Enemies (Upper Half of Screen)
            1.0 is Player
            0.95 is Attacks (Lower Half of Screen)
            0.9 is Enemies (Lower Half of Screen)
            */
            if transform.translation.y < 500.0 / 2.0 {
                transform.translation.z = 1.4;
            } else {
                transform.translation.z = 0.9;
            }
            if transform.translation.x < player_position.position.x {
                sprite.flip_x = true; // flip the sprite horizontally
            } else {
                sprite.flip_x = false; // don't flip the sprite
            }

            let speed = enemy.speed; // adjust this to change the enemy speed
            let target_position = player_position.position
                + Vec3::new(radius * angle.cos(), radius * angle.sin(), 0.0);
            let direction = (target_position - transform.translation).normalize();

            let distance_to_target = (target_position - transform.translation).length();

            if distance_to_target < 10.0 {
                transform.translation = target_position;
                // if attack possible -> attack, spawn attack, attack active, damage player. Once attack not active, then attack not active
                // if attacking -> check if active
                match enemy.state {
                    EnemyState::AttackPossible => {
                        hp.hp -= enemy.damage;
                        commands
                            .entity(entity)
                            .insert(CooldownTimer(Timer::from_seconds(10.0, TimerMode::Once)));

                        let midpoint = (player_position.position + transform.translation) / 2.0;
                        let midmidpoint: Vec3 = (midpoint + transform.translation) / 2.0;
                        let direction = (player_position.position - midpoint).normalize();
                        let angle: f32 = direction.y.atan2(direction.x);

                        let texture_handle =
                            asset_server.load("sprites/effects/bow-shoot-Sheet.png");
                        let texture_atlas = TextureAtlas::from_grid(
                            texture_handle,
                            Vec2::new(39.0, 16.0),
                            5,
                            1,
                            None,
                            None,
                        );
                        let texture_atlas_handle = texture_atlases.add(texture_atlas);
                        commands.spawn((
                            SpriteSheetBundle {
                                texture_atlas: texture_atlas_handle,
                                sprite: TextureAtlasSprite::new(0),
                                transform: Transform::from_translation(Vec3::new(
                                    midmidpoint.x,
                                    midmidpoint.y,
                                    1.75,
                                ))
                                .with_scale(Vec3::splat(2.5))
                                .with_rotation(Quat::from_rotation_z(angle + 90.0)),
                                ..default()
                            },
                            AnimationIndices {
                                first: 0,
                                last: 4,
                                delete_on_end: true,
                            },
                            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
                        ));
                        enemy.state = EnemyState::Attacking;
                    }
                    EnemyState::Attacking => {}
                    EnemyState::Moving => {
                        enemy.state = EnemyState::AttackPossible;
                    }
                }
            } else {
                enemy.state = EnemyState::Moving;
                transform.translation += direction * speed * time.delta_seconds();
            }

            angle += angle_step;
        }
    }
}

pub fn cooldown_manager(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Enemy, &mut CooldownTimer), With<CooldownTimer>>,
    time: Res<Time>,
) {
    for (entity, mut enemy, mut timer) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            enemy.state = EnemyState::AttackPossible;
            commands.entity(entity).remove::<CooldownTimer>();
        }
    }
}

pub fn animate_sprites(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &AnimationIndices,
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
        ),
        Without<Enemy>,
    >,
) {
    for (entity, indices, mut timer, mut sprite) in query.iter_mut() {
        timer.tick(time.delta());
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

pub fn enemy_death_handler(
    mut commands: Commands,
    mut enemy_query: Query<(&mut Enemy, &Transform, Entity), With<Enemy>>,
    projectile_query: Query<(&Transform, &SpawnedProjectile), With<DamageEnemy>>,
) {
    for (projectile_transform, projectile_info) in projectile_query.iter() {
        for (mut enemy, enemy_transform, entity) in enemy_query.iter_mut() {
            if projectile_info.x_radius
                > (projectile_transform.translation.x - enemy_transform.translation.x).abs()
                && projectile_info.y_radius
                    > (projectile_transform.translation.y - enemy_transform.translation.y).abs()
            {
                enemy.health -= projectile_info.damage;
                commands.entity(entity).despawn();
            }

            if enemy.health <= 0 {
                // TODO: death animation.
                commands.entity(entity).despawn();
            }
        }
    }
}
