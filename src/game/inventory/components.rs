use bevy::prelude::Resource;

use super::card_components::Card;




#[derive(Resource, Debug, Default)]
pub struct Inventory {
    pub cards: Vec<Card>,
}
