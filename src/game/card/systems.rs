use bevy::prelude::Commands;

use super::components::{Card, CardType, InInventory};

pub fn init_cards (mut commands: Commands) {
    commands.spawn((Card {
        card_type: CardType::ATTACK,
        name: "1".to_string(),
        description: "".to_string(),
        cost: 10u8,
    }));
    commands.spawn((Card {
        card_type: CardType::ATTACK,
        name: "2".to_string(),
        description: "".to_string(),
        cost: 10u8,
    }));
    commands.spawn((Card {
        card_type: CardType::ATTACK,
        name: "3".to_string(),
        description: "".to_string(),
        cost: 10u8,
    }));
    commands.spawn((Card {
        card_type: CardType::ENEMY,
        name: "4".to_string(),
        description: "".to_string(),
        cost: 10u8,
    }));
}
