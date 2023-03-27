use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_tweening::Lerp;
use rand::{random, thread_rng, Rng};

use crate::game::attacks::components::DamageEnemy;
use crate::game::enemy::components::{AnimationTimer, AnimationIndices};
use crate::game::inventory::components::Inventory;
use crate::game::player::components::{PlayerPosition, MP};
use crate::game::{attacks::components::SpawnedProjectile, inventory::card_components::Card};

use super::card_components::{CardType, MeleeType, ProjectileType, MarkedForPlaying};

pub const IDLE_POSITIONS: [Vec3; 3] = [
    Vec3::new(75.0, 90.0, 9.01),
    Vec3::new(175.0, 95.0, 9.02),
    Vec3::new(275.0, 90.0, 9.03),
];

pub const HOVER_POSITIONS: [Vec3; 3] = [
    Vec3::new(75.0, 110.0, 9.11),
    Vec3::new(175.0, 115.0, 9.12),
    Vec3::new(275.0, 110.0, 9.13),
];

/*

    Z-Axis Meanings:
    10.0 is Camera
    9.0:
        9.1X: Hovered Cards
            9.11: Card1
            9.12: Card2
            9.13: Card3
        9.0X: Idle Cards
            9.01: Card1
            9.02: Card2
            9.03: Card3
    2.0 Attacks (Upper Half of Screen)
    1.4 Enemies (Upper Half of Screen)
    1.0 is Player
    0.95 is Attacks (Lower Half of Screen)
    0.9 is Enemies (Lower Half of Screen)
    0.0 is Background
*/

pub fn init_render_cards(
    mut commands: Commands,
    inventory_resource: Res<Inventory>,
    asset_server: Res<AssetServer>,
) {
    // get cards:
    let mut j: usize = 0;
    for card in &inventory_resource.cards {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load(&card.sprite_path),
                transform: Transform::from_translation(IDLE_POSITIONS[j])
                    .with_scale(Vec3::new(2.5f32, 2.5f32, 2.5f32)),
                ..default()
            },
            Card {
                card_type: card.card_type,
                name: card.name.clone(),
                description: card.description.clone(),
                cost: card.cost,
                sprite_path: card.sprite_path.clone(),
                id: j as i8,
            },
        ));
        j += 1;
    }
}

pub fn card_handler(
    mut card_query: Query<(Entity, &mut Transform, &mut Sprite, &Card), With<Card>>,
    mut camera_query: Query<(&GlobalTransform, &mut Camera), With<Camera>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
    btn: Res<Input<MouseButton>>,
    keys: Res<Input<KeyCode>>,
    mut inventory_resource: ResMut<Inventory>,
    mut commands: Commands,
) {
    let window: &Window = window_query.get_single().unwrap();
    let (camera_transform, camera) = camera_query.single_mut();

    if keys.any_just_released([KeyCode::Key1, KeyCode::Key2, KeyCode::Key3]) {
        let mut id: i8 = 0;
        if keys.just_released(KeyCode::Key1) {
            id = 0i8;
        } else if keys.just_released(KeyCode::Key2) {
            id = 1i8;
        } else if keys.just_released(KeyCode::Key3) {
            id = 2i8;
        }
        for (entity, _, _, card) in card_query.iter() {
            if id == card.id {
                commands.entity(entity).insert(MarkedForPlaying);
                inventory_resource.cards.remove(card.id as usize);
            }
        }
        return;
    }

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        if world_position.x > 400.0f32 || world_position.y > 200.0f32 {
            // mouse is too far
            update_card(&(40.0 / 255.0), &mut card_query, time);
        } else {
            // mouse is nearby
            update_card(&1.0f32, &mut card_query, time);
            let closest_card: i8 = find_closest_card(
                &mut card_query,
                Vec3::new(world_position.x, world_position.y, 9.0f32),
            );
            if btn.just_pressed(MouseButton::Left) {
                for (entity, _, _, card) in card_query.iter() {
                    if card.id == closest_card {
                        commands.entity(entity).insert(MarkedForPlaying);
                        inventory_resource.cards.remove(card.id as usize);
                    }
                }
            }
        }
    }
}

fn update_card(
    new_transparency: &f32,
    card_query: &mut Query<(Entity, &mut Transform, &mut Sprite, &Card), With<Card>>,
    time: Res<Time>,
) {
    for (_, mut transform, mut sprite, card) in card_query.iter_mut() {
        let a = sprite.color.a();
        sprite
            .color
            .set_a(a.lerp(new_transparency, &(12.0 * time.delta_seconds())));
        transform.translation = IDLE_POSITIONS[card.id as usize];
    }
}

fn find_closest_card<'a>(
    card_query: &mut Query<(Entity, &mut Transform, &mut Sprite, &Card), With<Card>>,
    target_position: Vec3,
) -> i8 {
    if let Some((_, mut transform, _, card)) =
        card_query.iter_mut().min_by(|(_, a, _, _), (_, b, _, _)| {
            let distance_a = a.translation.distance_squared(target_position);
            let distance_b = b.translation.distance_squared(target_position);
            distance_a.partial_cmp(&distance_b).unwrap()
        })
    {
        if (transform.translation.x - target_position.x).abs() <= 60.0 {
            transform.translation = HOVER_POSITIONS[card.id as usize];
            return card.id;
        }
    }
    return -1;
}

pub fn play_card(
    card_query: Query<&Card, With<MarkedForPlaying>>,
    mut mp: ResMut<MP>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_position: Res<PlayerPosition>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    // CARDS TODO:
    // TODO: Make stomp actually work
    // TODO: Animate stomp to fade, using the enemy modules plugin handler (maybe make this it's own plugin?)
    // TODO: Make fireball animation play properly (Same as above)
    // TODO: More attacks.
    for card in card_query.iter() {
        match &card.card_type {
            CardType::Melee(melee_type) => {
                mp.mp += card.cost as i32;
                mp.mp = mp.mp.clamp(0, mp.max_mp);
                match melee_type {
                    MeleeType::Stomp => {
                        let tile_size = Vec2::new(64.0, 64.0);
                        let texture_handle = asset_server.load("sprites/projectiles/stomp.png");
                        let texture_atlas = TextureAtlas::from_grid(
                            texture_handle,
                            tile_size,
                            4,
                            1,
                            None,
                            None,
                        ); 
                        let texture_atlas_handle = texture_atlases.add(texture_atlas);
                        println!("Melee/Stomp played");
                        commands.spawn((
                            SpriteSheetBundle {
                                texture_atlas: texture_atlas_handle,
                                sprite: TextureAtlasSprite::new(0),
                                transform: Transform::from_translation(Vec3::new(
                                    player_position.position.x,
                                    player_position.position.y,
                                    9.0,
                                ))
                                .with_scale(Vec3::new(2.5f32, 2.5f32, 2.5f32)),
                                ..default()
                            },
                            SpawnedProjectile {
                                direction: Vec2::ZERO,
                                total_bounces: 0,
                                max_bounces: 1,
                                x_radius: tile_size.x,
                                y_radius: tile_size.y,
                                damage: 3,
                            },
                            DamageEnemy {},
                            AnimationIndices {
                                first: 0,
                                last: 3,
                                delete_on_end: true,
                            },
                            AnimationTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
                        ));
                    }
                    MeleeType::Other => panic!("No card should have meleetype other."),
                }
            }
            CardType::Projectile(projectile_type) => {
                mp.mp -= card.cost as i32;
                mp.mp = mp.mp.clamp(0, mp.max_mp);
                match projectile_type {
                    ProjectileType::Fireball => {
                        let tile_size = Vec2::new(64.0, 32.0);
                        let texture_handle = asset_server.load("sprites/projectiles/fireball.png");
                        let texture_atlas = TextureAtlas::from_grid(
                            texture_handle,
                            tile_size,
                            3,
                            1,
                            None,
                            None,
                        ); 
                        let texture_atlas_handle = texture_atlases.add(texture_atlas);
                        let mut rng = thread_rng();
                        println!("Projectile/Fireball played");
                        commands.spawn((
                            SpriteSheetBundle {
                                texture_atlas: texture_atlas_handle,
                                sprite: TextureAtlasSprite::new(0),
                                transform: Transform::from_translation(Vec3::new(
                                    player_position.position.x,
                                    player_position.position.y,
                                    9.0,
                                ))
                                .with_scale(Vec3::new(5.0f32, 5.0f32, 1.0f32)),
                                ..default()
                            },
                            SpawnedProjectile {
                                direction: Vec2::new(rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0)).normalize(),
                                total_bounces: 0,
                                max_bounces: 5,
                                x_radius: 25.0,
                                y_radius: 5.0,
                                damage: 3,
                            },
                            DamageEnemy {},
                            AnimationIndices {
                                first: 0,
                                last: 2,
                                delete_on_end: false,
                            },
                            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
                        ));
                    }
                    ProjectileType::Other => panic!("No card should have projectiletype other."),
                }
            }
            CardType::Other => panic!("No card should have type other."),
        }
    }
}

pub fn inventory_changed(
    mut commands: Commands,
    inventory_resource: Res<Inventory>,
    asset_server: Res<AssetServer>,
    card_query: Query<Entity, With<Card>>,
) {
    if inventory_resource.is_changed() && inventory_resource.cards.len() == 3 {
        for entity in card_query.iter() {
            commands.entity(entity).despawn();
        }
        init_render_cards(commands, inventory_resource, asset_server);
    }
}
