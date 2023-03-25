use bevy::prelude::*;

use crate::game::inventory::components::Inventory;

use super::components::Card;
pub const IDLE_POSITIONS:[Vec3; 3] = [Vec3::new(75.0, 150.0, 9.01), Vec3::new(175.0, 2000.0, 9.02), Vec3::new(275.0, 150.0, 9.03)];
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
    inventory_query: Query<&Inventory>,
    asset_server: Res<AssetServer>,
) {
    // get cards:
    for inventory in inventory_query.iter() {
        // should only be one inventory?
        let mut j:usize = 0;
        for card in &inventory.cards {
            commands.spawn(
                (
                    SpriteBundle {
                        texture: asset_server.load(&card.sprite_path),
                        transform: Transform::from_translation(IDLE_POSITIONS[j]).with_scale(Vec3::new(2.5f32, 2.5f32, 2.5f32)),
                        ..default()
                    },
                    Card {..default()},
                ),
            );
            j += 1;
            println!("Spawning card {}", card.name);
        }
        println!("Spawning cardS");
    }
    println!("Render cardS done");
}

pub fn debug_cards (
    mut commands: Commands,
    card_query: Query<&mut Transform, With<Card>>
) {
    for card_transform in card_query.iter() {
        println!("{}", card_transform.translation);
    }
}