use bevy::prelude::Component;

#[derive[Component, default]]
pub struct Enemy {
    pub health:i32,
    pub max_health:i32,
    pub speed:i32,
    pub enemy_type: EnemyType
}

#[derive(Default)]
enum EnemyType {
    #[default] Other, // Other in any type will crash the game. Do not instantiate anything with an Other type.
    Swordsman,
    Archer,
    Mage
}