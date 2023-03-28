use bevy::prelude::*;
use rand::{rngs::ThreadRng, thread_rng, Rng};

use super::{
    card_components::{Card, CardType, ProjectileType, MeleeType, BuffType},
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
            cost: 5,
            sprite_path: "sprites/cards/projectiles/fireball.png".to_string(),
            id: 0i8,
        },
        Card {
            card_type: CardType::Projectile(ProjectileType::NrgBall),
            cost: 5,
            sprite_path: "sprites/cards/buffs/heal.png".to_string(),
            id: 0i8,
        },
        Card {
            card_type: CardType::Melee(MeleeType::Stomp),
            cost: 10,
            sprite_path: "sprites/cards/melee/stomp.png".to_string(),
            id: 0i8,
        },
        Card {
            card_type: CardType::Buff(BuffType::Heal),
            cost: 5,
            sprite_path: "sprites/cards/buffs/heal.png".to_string(),
            id: 0i8,
        }
    ];

    let mut rng: ThreadRng = thread_rng();
    let index = rng.gen_range(0..cards.len());
    let card = cards[index].clone();
    return card;
}
