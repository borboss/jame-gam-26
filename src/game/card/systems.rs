use bevy::prelude::Commands;

pub fn init_cards (mut commands: Commands) {
    Commands.spawn(Card {
        card_type: CardType::ATTACK,
        name: "".to_string(),
        description: "".to_string(),
        cost: 10u8,
        used: false,
    });
    commands.spawn(Card {
        card_type: CardType::ENEMY,
        name: "".to_string(),
        description: "".to_string(),
        cost: 10u8,
        used: false,
    });
}
