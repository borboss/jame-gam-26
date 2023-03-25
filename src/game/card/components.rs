use bevy::prelude::{Component};

#[derive(Component)]
pub struct Card {
    pub card_type: CardType,
    pub name: String,
    pub description: String,
    pub cost: u8,
    pub used: bool
}
impl Default for Card {
    fn default() -> Card {
        Card {
            card_type: CardType::ATTACK,
            name: "Fireball".to_string(),
            description: "Launches a fireball in a random direction in the game!".to_string(),
            cost: 10u8,
            used: false,
        }
    }
}


#[derive(Default, Clone, Copy)]
pub enum CardType {
    ENEMY,
    #[default] ATTACK
}