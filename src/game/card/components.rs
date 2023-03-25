use bevy::prelude::{Component};

#[derive(Component, Debug, Clone)]
pub struct Card {
    pub card_type: CardType,
    pub name: String,
    pub description: String,
    pub cost: u8,
}
impl Default for Card {
    fn default() -> Card {
        Card {
            card_type: CardType::ATTACK,
            name: "Fireball".to_string(),
            description: "Launches a fireball in a random direction in the game!".to_string(),
            cost: 10u8,
        }
    }
}

#[derive(Component)]
pub struct InInventory {}

#[derive(Default, Clone, Copy, Debug)]
pub enum CardType {
    ENEMY,
    #[default] ATTACK
}