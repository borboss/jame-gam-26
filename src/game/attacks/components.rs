use bevy::prelude::{Component, Vec2};

#[derive(Component)]
pub struct SpawnedProjectile {
    pub direction: Vec2,
    pub total_bounces: u8,
    pub max_bounces: u8
}
#[derive(Component)]
pub struct DamageEnemy {}
#[derive(Component)]
pub struct DamagePlayer {}
#[derive(Component)]
pub struct FadeSoon {
    pub time: u8
}
