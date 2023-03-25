use bevy::prelude::Component;

use super::card_components::Card;




#[derive(Component, Debug)]
pub struct Inventory {
    pub cards: Vec<Card>,
}

#[derive(Component)]
pub struct InventoryMarker;