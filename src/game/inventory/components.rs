use bevy::prelude::Component;
use super::super::card::*;


#[derive(Component)]
pub struct Inventory {
    cards: Vec<Card>,
}