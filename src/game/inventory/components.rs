use bevy::{prelude::{Component, Resource}, utils::default};
use crate::game::card::components::Card;




#[derive(Component, Debug)]
pub struct Inventory {
    pub cards: Vec<Card>,
}
