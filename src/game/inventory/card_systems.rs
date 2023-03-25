use bevy::window::PrimaryWindow;
use bevy::{prelude::*, transform};
use bevy_tweening::Lerp;

use crate::game::inventory::card_components::Card;
use crate::game::inventory::components::Inventory;

pub const IDLE_POSITIONS: [Vec3; 3] = [
    Vec3::new(75.0, 90.0, 9.01),
    Vec3::new(175.0, 95.0, 9.02),
    Vec3::new(275.0, 90.0, 9.03),
];

pub const HOVER_POSITIONS: [Vec3; 3] = [
    Vec3::new(75.0, 100.0, 9.11),
    Vec3::new(175.0, 105.0, 9.12),
    Vec3::new(275.0, 100.0, 9.13),
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
    1.0 is Player
    0.91 is Attacks
    0.9 is Enemies
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
            Card { id: j as u8, ..default() },
        ));
        j += 1;
    }
}

pub fn debug_cards(card_query: Query<&mut Transform, With<Card>>) {
    for card_transform in card_query.iter() {}
}

pub fn invis_cards(
    mut card_query: Query<(Entity, &mut Transform, &mut Sprite, &Card), With<Card>>,
    mut camera_query: Query<(&GlobalTransform, &mut Camera), With<Camera>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    let window: &Window = window_query.get_single().unwrap();
    let (camera_transform, camera) = camera_query.single_mut();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        if world_position.x > 400.0f32 || world_position.y > 200.0f32 {
            // mouse is too far away
            for (_, mut transform, mut sprite, card) in card_query.iter_mut() {
                let a = sprite.color.a();
                sprite
                    .color
                    .set_a(a.lerp(&(40.0 / 255.0), &(5.0 * time.delta_seconds())));
                transform.translation = IDLE_POSITIONS[card.id as usize];
            }
        } else {
            // mouse is nearby
            for (_, mut transform, mut sprite, card) in card_query.iter_mut() {
                let a = sprite.color.a();
                sprite
                    .color
                    .set_a(a.lerp(&(1.0), &(12.0 * time.delta_seconds())));
                transform.translation = IDLE_POSITIONS[card.id as usize];
            }

            find_closest_card(
                &mut card_query,
                Vec3::new(world_position.x, world_position.y, 9.0f32),
            );
        }
    }
}

fn find_closest_card(
    card_query: &mut Query<(Entity, &mut Transform, &mut Sprite, &Card), With<Card>>,
    target_position: Vec3,
){
    if let Some((_, mut transform, _, card)) = card_query.iter_mut().min_by(|(_, a, _, _), (_, b, _, _)| {
        let distance_a = a.translation.distance_squared(target_position);
        let distance_b = b.translation.distance_squared(target_position);
        distance_a.partial_cmp(&distance_b).unwrap()
    }) {
        transform.translation = HOVER_POSITIONS[card.id as usize];
    }
}
