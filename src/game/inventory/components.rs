use bevy::{prelude::{Component, Resource}, utils::default};
use crate::game::card::components::Card;

use super::super::card::*;


#[derive(Resource, Default)]
pub struct Inventory {
    pub cards: [Card; 4],
}

