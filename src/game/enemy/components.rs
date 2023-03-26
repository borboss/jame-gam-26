use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Enemy {
    pub direction: Vec3,
    pub health: i32,
    pub max_health: i32,
    pub speed: f32,
    pub enemy_type: EnemyType,
}

#[derive(Default)]
pub enum EnemyType {
    #[default]
    Other, // Other in any type will crash the game. Do not instantiate anything with an Other type.
    Swordsman,
    Archer,
    Mage,
}

use bevy::{prelude::Resource, time::Timer, time::TimerMode};
pub const ENEMY_SPAWN_TIME: u8 = 2;

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}
impl Default for EnemySpawnTimer {
    fn default() -> EnemySpawnTimer {
        EnemySpawnTimer {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIME as f32, TimerMode::Repeating),
        }
    }
}

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);
