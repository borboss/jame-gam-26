use bevy::prelude::Component;

#[derive(Component)]
pub struct Card {
    pub card_type: CardType,
    pub name: string,
    pub description: string,
}

pub enum CardType {
    ENEMY,
    ATTACK
}