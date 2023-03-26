use bevy::prelude::*;
use rand::{rngs::ThreadRng, thread_rng, Rng};

use super::{
    card_components::{Card, CardType, ProjectileType, MeleeType},
    components::Inventory,
};

pub const INVENTORY_SIZE: i32 = 3;

pub fn maintain_inventory(inventory_resource: ResMut<Inventory>) {
    init_inventory(inventory_resource);
}
pub fn init_inventory(mut inventory_resource: ResMut<Inventory>) {
    let needed_cards: i32 = INVENTORY_SIZE - (inventory_resource.cards.len() as i32);
    for _ in 0..needed_cards {
        inventory_resource.cards.push(draw_card());
    }
}

fn draw_card() -> Card {
    let cards = vec![
        Card {
            card_type: CardType::Projectile(ProjectileType::Fireball),
            name: "Fireball".to_string(),
            description: "Launches a fireball in a random direction in the game!".to_string(),
            cost: 1,
            sprite_path: "sprites/cards/projectiles/fireball.png".to_string(),
            id: 0i8,
        },
        Card {
            card_type: CardType::Melee(MeleeType::Stomp),
            name: "Stomp".to_string(),
            description: "Stomp nearby enemies and regain some MP.".to_string(),
            cost: 5,
            sprite_path: "sprites/cards/melee/stomp.png".to_string(),
            id: 0i8,
        },
        Card {
            card_type: CardType::Projectile(ProjectileType::Fireball),
            name: "3".to_string(),
            description: "C".to_string(),
            cost: 3,
            sprite_path: "sprites/cards/projectiles/blank_attack.png".to_string(),
            id: 0i8,
        },
        Card {
            card_type: CardType::Projectile(ProjectileType::Fireball),
            name: "4".to_string(),
            description: "D".to_string(),
            cost: 4,
            sprite_path: "sprites/cards/melee/blank_enemy.png".to_string(),
            id: 0i8,
        },
    ];

    let mut rng: ThreadRng = thread_rng();
    let index = rng.gen_range(0..cards.len());
    let card = cards[index].clone();
    return card;
}
